#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Margin {
    pub horizontal: u16,
    pub vertical: u16,
}

impl Margin {
    pub fn none() -> Self {
        Self {
            horizontal: 0,
            vertical: 0,
        }
    }

    /// Set uniform margin for all sides.
    pub const fn all(value: u16) -> Self {
        Self {
            horizontal: value,
            vertical: value,
        }
    }

    /// Set the margin of left and right sides to specified value.
    pub const fn horizontal(value: u16) -> Self {
        Self {
            horizontal: value,
            vertical: 0,
        }
    }

    /// Set the margin of top and bottom sides to specified value.
    pub const fn vertical(value: u16) -> Self {
        Self {
            horizontal: 0,
            vertical: value,
        }
    }

    /// Get the total width of the margin (left + right)
    pub const fn width(&self) -> u16 {
        self.horizontal * 2
    }

    /// Get the total height of the margin (top + bottom)
    pub const fn height(&self) -> u16 {
        self.vertical * 2
    }
}
