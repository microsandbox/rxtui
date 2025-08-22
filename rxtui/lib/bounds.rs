//! Bounds and rectangle operations for dirty region tracking.
//!
//! This module provides types and operations for tracking rectangular regions
//! on the terminal screen, used for efficient dirty region tracking and clipping.

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A rectangular region defined by position and dimensions.
///
/// Used for:
/// - Tracking dirty regions that need redrawing
/// - Clipping child elements to parent bounds
/// - Hit testing for mouse events
///
/// ## Coordinate System
///
/// ```text
/// (0,0) ─────────────▶ x
///   │   ┌─────────┐
///   │   │ (x,y)   │
///   │   │ ┌─────┐ │
///   │   │ │     │ │ height
///   │   │ └─────┘ │
///   │   └─────────┘
///   ▼      width
///   y
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// X coordinate of the top-left corner
    pub x: u16,

    /// Y coordinate of the top-left corner
    pub y: u16,

    /// Width in terminal columns
    pub width: u16,

    /// Height in terminal rows
    pub height: u16,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Rect {
    /// Creates a new rectangle with the given position and dimensions.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates an empty rectangle at origin.
    pub fn empty() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    /// Returns the right edge coordinate (exclusive).
    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the bottom edge coordinate (exclusive).
    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Checks if this rectangle contains the given point.
    ///
    /// ## Example
    ///
    /// ```text
    /// Rect { x: 10, y: 5, width: 20, height: 10 }
    /// contains(15, 7) → true
    /// contains(5, 7) → false
    /// ```
    pub fn contains_point(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    /// Checks if this rectangle is empty (zero width or height).
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Calculates the intersection of two rectangles.
    ///
    /// Returns the overlapping region, or an empty rectangle if no overlap.
    ///
    /// ## Example
    ///
    /// ```text
    /// ┌─────┐
    /// │  A  │──┐
    /// └─────┘  │ intersection
    ///    │  ┌──▼──┐
    ///    └──│     │
    ///       │  B  │
    ///       └─────┘
    /// ```
    pub fn intersection(&self, other: &Rect) -> Rect {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        if x < right && y < bottom {
            Rect::new(x, y, right - x, bottom - y)
        } else {
            Rect::empty()
        }
    }

    /// Checks if two rectangles intersect.
    pub fn intersects(&self, other: &Rect) -> bool {
        !self.intersection(other).is_empty()
    }

    /// Calculates the union of two rectangles.
    ///
    /// Returns the smallest rectangle that contains both rectangles.
    ///
    /// ## Example
    ///
    /// ```text
    /// ┌─────────────┐ union
    /// │ ┌─────┐     │
    /// │ │  A  │  B  │
    /// │ └─────┘     │
    /// └─────────────┘
    /// ```
    pub fn union(&self, other: &Rect) -> Rect {
        if self.is_empty() {
            return *other;
        }
        if other.is_empty() {
            return *self;
        }

        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());

        Rect::new(x, y, right - x, bottom - y)
    }

    /// Clips this rectangle to fit within the given bounds.
    ///
    /// Returns the intersection of this rectangle with the bounds.
    pub fn clip_to(&self, bounds: &Rect) -> Rect {
        self.intersection(bounds)
    }

    /// Expands the rectangle by the given amount in all directions.
    ///
    /// Useful for adding padding or margins.
    pub fn expand(&self, amount: u16) -> Rect {
        Rect::new(
            self.x.saturating_sub(amount),
            self.y.saturating_sub(amount),
            self.width.saturating_add(amount * 2),
            self.height.saturating_add(amount * 2),
        )
    }

    /// Contracts the rectangle by the given amount in all directions.
    ///
    /// Useful for applying padding inward.
    pub fn contract(&self, amount: u16) -> Rect {
        if amount * 2 >= self.width || amount * 2 >= self.height {
            Rect::empty()
        } else {
            Rect::new(
                self.x + amount,
                self.y + amount,
                self.width - amount * 2,
                self.height - amount * 2,
            )
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_edges() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.right(), 40);
        assert_eq!(rect.bottom(), 60);
    }

    #[test]
    fn test_contains_point() {
        let rect = Rect::new(10, 10, 20, 20);
        assert!(rect.contains_point(10, 10)); // top-left
        assert!(rect.contains_point(29, 29)); // bottom-right - 1
        assert!(!rect.contains_point(30, 30)); // outside
        assert!(!rect.contains_point(9, 15)); // left of rect
    }

    #[test]
    fn test_intersection() {
        let rect1 = Rect::new(10, 10, 20, 20);
        let rect2 = Rect::new(20, 20, 20, 20);
        let result = rect1.intersection(&rect2);
        assert_eq!(result, Rect::new(20, 20, 10, 10));

        // No intersection
        let rect3 = Rect::new(50, 50, 10, 10);
        assert!(rect1.intersection(&rect3).is_empty());
    }

    #[test]
    fn test_union() {
        let rect1 = Rect::new(10, 10, 10, 10);
        let rect2 = Rect::new(30, 30, 10, 10);
        let result = rect1.union(&rect2);
        assert_eq!(result, Rect::new(10, 10, 30, 30));
    }
}
