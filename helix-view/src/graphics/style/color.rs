use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightGray,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl Color {
    /// Creates a `Color` from a hex string of the form
    /// "#RRGGBB" or "#RGB"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use helix_view::graphics::style::Color;
    ///
    /// let color1 = Color::from_hex("#c0ffee").unwrap();
    /// let color2 = Color::Rgb(192, 255, 238);
    ///
    /// assert_eq!(color1, color2);
    ///
    /// let color3 = Color::from_hex("#012").unwrap();
    /// assert_eq!(color3, Color::Rgb(0, 17, 34));
    /// ```
    pub fn from_hex(h: &str) -> Result<Self, MalformedHex> {
        let h = h.as_bytes();
        if !h.starts_with(b"#") {
            return Err(MalformedHex::NoHash);
        }

        use byte_from_hex as pair;
        use dupe_from_nibble as nibble;

        match h.len() {
            7 => match (|| {
                Some(Self::Rgb(
                    pair([h[1], h[2]])?,
                    pair([h[3], h[4]])?,
                    pair([h[5], h[6]])?,
                ))
            })() {
                Some(c) => Ok(c),
                None => Err(MalformedHex::NotANibble),
            },
            4 => match (|| Some(Self::Rgb(nibble(h[1])?, nibble(h[2])?, nibble(h[3])?)))() {
                Some(c) => Ok(c),
                None => Err(MalformedHex::NotANibble),
            },
            _ => Err(MalformedHex::LenOOB),
        }
    }
}

#[cfg(feature = "term")]
impl From<Color> for termina::style::ColorSpec {
    fn from(color: Color) -> Self {
        match color {
            Color::Reset => Self::Reset,
            Color::Black => Self::BLACK,
            Color::Red => Self::RED,
            Color::Green => Self::GREEN,
            Color::Yellow => Self::YELLOW,
            Color::Blue => Self::BLUE,
            Color::Magenta => Self::MAGENTA,
            Color::Cyan => Self::CYAN,
            Color::Gray => Self::BRIGHT_BLACK,
            Color::White => Self::BRIGHT_WHITE,
            Color::LightRed => Self::BRIGHT_RED,
            Color::LightGreen => Self::BRIGHT_GREEN,
            Color::LightBlue => Self::BRIGHT_BLUE,
            Color::LightYellow => Self::BRIGHT_YELLOW,
            Color::LightMagenta => Self::BRIGHT_MAGENTA,
            Color::LightCyan => Self::BRIGHT_CYAN,
            Color::LightGray => Self::WHITE,
            Color::Indexed(i) => Self::PaletteIndex(i),
            Color::Rgb(r, g, b) => termina::style::RgbColor::new(r, g, b).into(),
        }
    }
}

#[cfg(all(feature = "term", windows))]
impl From<Color> for crossterm::style::Color {
    fn from(color: Color) -> Self {
        use crossterm::style::Color as CColor;

        match color {
            Color::Reset => CColor::Reset,
            Color::Black => CColor::Black,
            Color::Red => CColor::DarkRed,
            Color::Green => CColor::DarkGreen,
            Color::Yellow => CColor::DarkYellow,
            Color::Blue => CColor::DarkBlue,
            Color::Magenta => CColor::DarkMagenta,
            Color::Cyan => CColor::DarkCyan,
            Color::Gray => CColor::DarkGrey,
            Color::LightRed => CColor::Red,
            Color::LightGreen => CColor::Green,
            Color::LightBlue => CColor::Blue,
            Color::LightYellow => CColor::Yellow,
            Color::LightMagenta => CColor::Magenta,
            Color::LightCyan => CColor::Cyan,
            Color::LightGray => CColor::Grey,
            Color::White => CColor::White,
            Color::Indexed(i) => CColor::AnsiValue(i),
            Color::Rgb(r, g, b) => CColor::Rgb { r, g, b },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MalformedHex {
    NoHash,
    LenOOB,
    NotANibble,
}

impl Display for MalformedHex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Malformed hex color code: {}",
            match self {
                Self::NoHash => "Missing hash prefix",
                Self::LenOOB => "Must be 12 or 24 bit RGB",
                Self::NotANibble => "One or more chars is not hex digit (nibble)",
            }
        )
    }
}

/// Decodes big-endian nibble-pair.
///
/// # Errors
/// If any byte isn't a nibble
const fn byte_from_hex(mut h: [u8; 2]) -> Option<u8> {
    // reuse memory
    h[0] = from_nibble(h[0]);
    h[1] = from_nibble(h[1]);
    // we could split this in 2 `if`s,
    // to avoid calling `from_nibble`,
    // but that might be slower
    if h[0] > 0xf || h[1] > 0xf {
        return None;
    }
    Some((h[0] << 4) | h[1])
}

#[must_use]
const fn from_nibble(h: u8) -> u8 {
    match h {
        b'A'..=b'F' => h - b'A' + 10,
        b'a'..=b'f' => h - b'a' + 10,
        b'0'..=b'9' => h - b'0',
        _ => 0xff, // Err
    }
}

/// Decodes nibble, repeating its value on each half,
/// i.e. the value is its own padding.
///
/// # Errors
/// If `h` isn't a nibble
#[must_use]
const fn dupe_from_nibble(mut h: u8) -> Option<u8> {
    h = from_nibble(h);
    if h > 0xf {
        return None;
    }
    Some((h << 4) | h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_nibble_lowercase() {
        for i in 0..0x10_u8 {
            let c = format!("{:x}", i);
            assert_eq!(c.len(), 1);
            assert_eq!(
                u8::from_str_radix(&c, 0x10).unwrap(),
                from_nibble(c.as_bytes()[0])
            );
        }
    }
    #[test]
    fn sanity_nibble_uppercase() {
        for i in 0..0x10_u8 {
            let c = format!("{:X}", i);
            assert_eq!(c.len(), 1);
            assert_eq!(
                u8::from_str_radix(&c, 0x10).unwrap(),
                from_nibble(c.as_bytes()[0])
            );
        }
    }

    #[test]
    fn sanity_nibble2() {
        assert_eq!(dupe_from_nibble(b'0'), Some(0));
        assert_eq!(dupe_from_nibble(b'1'), Some(0x11));
        assert_eq!(dupe_from_nibble(b'7'), Some(0x77));
        assert_eq!(dupe_from_nibble(b'a'), Some(0xaa));
        assert_eq!(dupe_from_nibble(b'f'), Some(0xff));
    }

    #[test]
    fn invalid_nibble() {
        for c in *b"gGzZ+-" {
            assert_eq!(from_nibble(c), 0xff);
        }
    }

    #[test]
    fn pair_endian() {
        assert_eq!(byte_from_hex(*b"00"), Some(0));
        assert_eq!(byte_from_hex(*b"fF"), Some(0xff));
        assert_eq!(byte_from_hex(*b"c3"), Some(0xc3));
    }
    #[test]
    fn invalid_pair() {
        assert!(byte_from_hex(*b"+1").is_none());
        assert!(byte_from_hex(*b"-1").is_none());
        assert!(byte_from_hex(*b"Gg").is_none());
        assert!(byte_from_hex(*b"0x").is_none());
    }

    #[test]
    fn hex_color_no_regress() {
        assert_eq!(Color::from_hex("#+a+b+c"), Err(MalformedHex::NotANibble));
        assert_eq!(Color::from_hex("#+0+1+2"), Err(MalformedHex::NotANibble));
    }
    #[test]
    fn hex_color_sanity() {
        assert_eq!(Color::from_hex("#01fe3a"), Ok(Color::Rgb(0x01, 0xfe, 0x3a)));
        assert_eq!(Color::from_hex("#abc"), Ok(Color::Rgb(0xaa, 0xbb, 0xcc)));
    }
    #[test]
    fn hex_color_invalid_len() {
        for h in [
            "#0",
            "#00",
            "#0000",
            "#00000",
            "#0000000",
            "#00000000",
            "#000000000",
            "#0000000000",
        ] {
            assert_eq!(Color::from_hex(h), Err(MalformedHex::LenOOB));
        }
    }
}
