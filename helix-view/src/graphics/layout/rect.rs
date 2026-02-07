use std::cmp::{max, min};

use super::margin::Margin;

/// A simple rectangle used in the computation of the layout and to give widgets an hint about the
/// area they are supposed to render to. (x, y) = (0, 0) is at the top left corner of the screen.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    /// Creates a new rect, with width and height
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    pub fn area(self) -> usize {
        (self.width as usize) * (self.height as usize)
    }

    #[inline]
    pub fn left(self) -> u16 {
        self.x
    }

    #[inline]
    pub fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    #[inline]
    pub fn top(self) -> u16 {
        self.y
    }

    #[inline]
    pub fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    // Returns a new Rect with width reduced from the left side.
    // This changes the `x` coordinate and clamps it to the right
    // edge of the original Rect.
    pub fn clip_left(self, width: u16) -> Rect {
        let width = std::cmp::min(width, self.width);
        Rect {
            x: self.x.saturating_add(width),
            width: self.width.saturating_sub(width),
            ..self
        }
    }

    // Returns a new Rect with width reduced from the right side.
    // This does _not_ change the `x` coordinate.
    pub fn clip_right(self, width: u16) -> Rect {
        Rect {
            width: self.width.saturating_sub(width),
            ..self
        }
    }

    // Returns a new Rect with height reduced from the top.
    // This changes the `y` coordinate and clamps it to the bottom
    // edge of the original Rect.
    pub fn clip_top(self, height: u16) -> Rect {
        let height = std::cmp::min(height, self.height);
        Rect {
            y: self.y.saturating_add(height),
            height: self.height.saturating_sub(height),
            ..self
        }
    }

    // Returns a new Rect with height reduced from the bottom.
    // This does _not_ change the `y` coordinate.
    pub fn clip_bottom(self, height: u16) -> Rect {
        Rect {
            height: self.height.saturating_sub(height),
            ..self
        }
    }

    pub fn with_height(self, height: u16) -> Rect {
        // new height may make area > u16::max_value, so use new()
        Self::new(self.x, self.y, self.width, height)
    }

    pub fn with_width(self, width: u16) -> Rect {
        Self::new(self.x, self.y, width, self.height)
    }

    pub fn inner(self, margin: Margin) -> Rect {
        if self.width < margin.width() || self.height < margin.height() {
            Rect::default()
        } else {
            Rect {
                x: self.x + margin.horizontal,
                y: self.y + margin.vertical,
                width: self.width - margin.width(),
                height: self.height - margin.height(),
            }
        }
    }

    /// Calculate the union between two [`Rect`]s.
    pub fn union(self, other: Rect) -> Rect {
        // Example:
        //
        // If `Rect` A is positioned at `(0, 0)` with a width and height of `5`,
        // and `Rect` B is positioned at `(5, 0)` with a width and height of `2`,
        // then this is the resulting union:
        //
        // x1 = min(0, 5) => x1 = 0
        // y1 = min(0, 0) => y1 = 0
        // x2 = max(0 + 5, 5 + 2) => x2 = 7
        // y2 = max(0 + 5, 0 + 2) => y2 = 5
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.x + self.width, other.x + other.width);
        let y2 = max(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    /// Calculate the intersection between two [`Rect`]s.
    pub fn intersection(self, other: Rect) -> Rect {
        // Example:
        //
        // If `Rect` A is positioned at `(0, 0)` with a width and height of `5`,
        // and `Rect` B is positioned at `(5, 0)` with a width and height of `2`,
        // then this is the resulting intersection:
        //
        // x1 = max(0, 5) => x1 = 5
        // y1 = max(0, 0) => y1 = 0
        // x2 = min(0 + 5, 5 + 2) => x2 = 5
        // y2 = min(0 + 5, 0 + 2) => y2 = 2
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.x + self.width, other.x + other.width);
        let y2 = min(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    pub fn intersects(self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_size_preservation() {
        for width in 0..256u16 {
            for height in 0..256u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area(); // Should not panic.
                assert_eq!(rect.width, width);
                assert_eq!(rect.height, height);
            }
        }

        // One dimension below 255, one above. Area below max u16.
        let rect = Rect::new(0, 0, 300, 100);
        assert_eq!(rect.width, 300);
        assert_eq!(rect.height, 100);
    }

    #[test]
    fn test_rect_chop_from_left() {
        let rect = Rect::new(0, 0, 20, 30);
        assert_eq!(Rect::new(10, 0, 10, 30), rect.clip_left(10));
        assert_eq!(
            Rect::new(20, 0, 0, 30),
            rect.clip_left(40),
            "x should be clamped to original width if new width is bigger"
        );
    }

    #[test]
    fn test_rect_chop_from_right() {
        let rect = Rect::new(0, 0, 20, 30);
        assert_eq!(Rect::new(0, 0, 10, 30), rect.clip_right(10));
    }

    #[test]
    fn test_rect_chop_from_top() {
        let rect = Rect::new(0, 0, 20, 30);
        assert_eq!(Rect::new(0, 10, 20, 20), rect.clip_top(10));
        assert_eq!(
            Rect::new(0, 30, 20, 0),
            rect.clip_top(50),
            "y should be clamped to original height if new height is bigger"
        );
    }

    #[test]
    fn test_rect_chop_from_bottom() {
        let rect = Rect::new(0, 0, 20, 30);
        assert_eq!(Rect::new(0, 0, 20, 20), rect.clip_bottom(10));
    }
}
