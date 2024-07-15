use anyhow::bail;
use std::{borrow::Cow, ops::Add};

use crate::{Document, Editor, View};

#[inline]
pub fn expand<'a>(editor: &Editor, input: &'a str) -> anyhow::Result<Cow<'a, str>> {
    let (view, doc) = current_ref!(editor);
    let shell = &editor.config().shell;

    // TODO: Only supports single `%sh{...}` block
    if let Some(start) = input.find("%sh{") {
        let start = start + 4; // Moves after `%sh{`
        let input = substitute_variables(doc, view, input)?;
        let mut output = String::new();

        // Include all prior part before `%sh{`
        output.push_str(&input[..start]);

        let mut braces = 1;
        let mut end = start;

        for (idx, ch) in input[start..].char_indices() {
            match ch {
                '{' => braces += 1,
                '}' => {
                    braces -= 1;
                    if braces == 0 {
                        end += idx;
                        break;
                    }
                }
                _ => {}
            }
        }

        if braces != 0 {
            bail!("`%sh` block was not closed");
        }

        let result = run_cmd(shell, input[start..end].trim())?;
        output.push_str(&result);
        if end + 1 < input.len() {
            // Include all `%sh{..}`
            output.push_str(&input[end + 1..]);
        }

        Ok(Cow::Owned(output))
    } else {
        substitute_variables(doc, view, input)
    }
}

#[allow(clippy::too_many_lines)]
fn substitute_variables<'a>(
    doc: &Document,
    view: &View,
    input: &'a str,
) -> anyhow::Result<Cow<'a, str>> {
    if input.is_empty() {
        return Ok(Cow::Borrowed(""));
    }
    let mut output = String::new();
    let mut remaining = input;
    let mut found_variable = false;

    while let Some(start) = remaining.find("%{") {
        if !found_variable {
            found_variable = true;
            output.reserve(input.len());
        }
        output.push_str(&remaining[..start]);
        if let Some(end) = remaining[start..].find('}') {
            let var_end = start + end;
            let var = remaining[start + 2..var_end].trim();

            match var {
                "basename" => {
                    let replacement = doc
                        .path()
                        .and_then(|it| it.file_name().and_then(|it| it.to_str()))
                        .unwrap_or(crate::document::SCRATCH_BUFFER_NAME);

                    output.push_str(replacement);
                    remaining = &remaining[var_end + 1..];
                }
                "filename" => {
                    let replacement = doc
                        .path()
                        .and_then(|path| path.to_str())
                        .unwrap_or(crate::document::SCRATCH_BUFFER_NAME);

                    output.push_str(replacement);
                    remaining = &remaining[var_end + 1..];
                }
                "dirname" => {
                    let replacement = doc
                        .path()
                        .and_then(|p| p.parent())
                        .and_then(std::path::Path::to_str)
                        .unwrap_or(crate::document::SCRATCH_BUFFER_NAME);

                    output.push_str(replacement);
                    remaining = &remaining[var_end + 1..];
                }
                "cwd" => {
                    let dir = helix_stdx::env::current_working_dir();
                    let replacement = dir.to_str().unwrap();
                    output.push_str(replacement);
                    remaining = &remaining[var_end + 1..];
                }
                "linenumber" => {
                    let replacement = (doc
                        .selection(view.id)
                        .primary()
                        .cursor_line(doc.text().slice(..))
                        + 1)
                    .to_string();

                    output.push_str(&replacement);
                    remaining = &remaining[var_end + 1..];
                }
                "selection" => {
                    let replacement = doc
                        .selection(view.id)
                        .primary()
                        .fragment(doc.text().slice(..));

                    output.push_str(&replacement);
                    remaining = &remaining[var_end + 1..];
                }
                "cursorcolumn" => {
                    let replacement = doc
                        .selection(view.id)
                        .primary()
                        .cursor(doc.text().slice(..))
                        .add(1)
                        .to_string();

                    output.push_str(&replacement);
                    remaining = &remaining[var_end + 1..];
                }
                "lang" => {
                    let replacement = doc.language_name().unwrap_or("text");

                    output.push_str(replacement);
                    remaining = &remaining[var_end + 1..];
                }
                "ext" => {
                    let replacement = doc
                        .path()
                        .and_then(|p| p.extension())
                        .and_then(|e| e.to_str())
                        .unwrap_or_default();

                    output.push_str(replacement);
                    remaining = &remaining[var_end + 1..];
                }
                unknown => anyhow::bail!("unknown variable `{unknown}`"),
            }
        } else {
            output.push_str(&remaining[start..]);
            break;
        }
    }

    if found_variable {
        output.push_str(remaining);
    }

    if found_variable {
        Ok(Cow::Owned(output))
    } else {
        Ok(Cow::Borrowed(input))
    }
}

fn _process_sh_blocks(shell: &[String], input: &str) -> anyhow::Result<String> {
    if !input.contains("%sh{") {
        let result = run_cmd(shell, input);
        return result;
    }

    let mut output = String::new();
    let mut remainder = input;

    while let Some(start) = remainder.find("%sh{") {
        output.push_str(&remainder[..start]);

        let mut braces = 1;
        let mut end = start + 4; // Skip over `%sh{`

        for (idx, ch) in remainder[end..].char_indices() {
            match ch {
                '{' => braces += 1,
                '}' => {
                    braces -= 1;
                    if braces == 0 {
                        end += idx;
                        break;
                    }
                }
                _ => {}
            }
        }

        if braces != 0 {
            bail!("`%sh` block was not closed");
        }

        let inner_result = _process_sh_blocks(shell, remainder[start + 4..end].trim())?;
        output.push_str(&inner_result);

        remainder = &remainder[end + 1..]; // Skip over the closing `}`
    }

    output.push_str(remainder);
    Ok(output)
}

fn run_cmd(shell: &[String], cmd: &str) -> anyhow::Result<String> {
    tokio::task::block_in_place(move || {
        helix_lsp::block_on(async move {
            let mut command = tokio::process::Command::new(&shell[0]);
            command.args(&shell[1..]).arg(cmd);
            let output = command
                .output()
                .await
                .map_err(|_| anyhow::anyhow!("Shell command failed: {cmd}"))?;
            if output.status.success() {
                if output.stdout.is_empty() {
                    String::from_utf8(output.stderr)
                        .map_err(|_| anyhow::anyhow!("Process did not output valid UTF-8"))
                } else {
                    String::from_utf8(output.stdout)
                        .map_err(|_| anyhow::anyhow!("Process did not output valid UTF-8"))
                }
            } else {
                bail!(
                    "failed to evaluate `%sh` block: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
            }
        })
    })
}
