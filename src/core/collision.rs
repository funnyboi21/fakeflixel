//! Axis-aligned overlap helpers for framework sprites.

use super::{math::Rect, render::Sprite};

/// Returns the area where two sprite bounds overlap.
///
/// Edges that merely touch do not count as an overlap. Negative rectangle
/// dimensions are normalized, while non-finite coordinates are rejected.
pub fn intersection(a: &Sprite, b: &Sprite) -> Option<Rect> {
    intersection_rects(a.bounds(), b.bounds())
}

/// Returns whether two sprites have a positive-area axis-aligned overlap.
pub fn overlaps(a: &Sprite, b: &Sprite) -> bool {
    intersection(a, b).is_some()
}

/// Returns the positive-area intersection of two rectangles, if any.
///
/// The rectangle endpoints are normalized, so rectangles with negative width
/// or height are handled consistently. Rectangles that only touch at an edge
/// or corner do not intersect.
pub fn intersection_rects(a: Rect, b: Rect) -> Option<Rect> {
    let (a_left, a_right) = ordered(a.min.x, a.max.x)?;
    let (a_top, a_bottom) = ordered(a.min.y, a.max.y)?;
    let (b_left, b_right) = ordered(b.min.x, b.max.x)?;
    let (b_top, b_bottom) = ordered(b.min.y, b.max.y)?;

    let left = a_left.max(b_left);
    let top = a_top.max(b_top);
    let right = a_right.min(b_right);
    let bottom = a_bottom.min(b_bottom);
    (right > left && bottom > top).then(|| Rect::new(left, top, right - left, bottom - top))
}

/// Returns whether two rectangles have a positive-area intersection.
pub fn rects_overlap(a: Rect, b: Rect) -> bool {
    intersection_rects(a, b).is_some()
}

fn ordered(first: f32, second: f32) -> Option<(f32, f32)> {
    (first.is_finite() && second.is_finite()).then(|| {
        if first <= second {
            (first, second)
        } else {
            (second, first)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Vec2;

    #[test]
    fn sprite_overlap_returns_intersection_bounds() {
        let mut first = Sprite::new(20.0, 10.0);
        first.position = Vec2::new(5.0, 4.0);
        let mut second = Sprite::new(8.0, 12.0);
        second.position = Vec2::new(15.0, 8.0);

        assert!(overlaps(&first, &second));
        assert_eq!(
            intersection(&first, &second),
            Some(Rect::new(15.0, 8.0, 10.0, 6.0))
        );
    }

    #[test]
    fn touching_edges_and_separated_rectangles_do_not_overlap() {
        assert!(!rects_overlap(
            Rect::new(0.0, 0.0, 10.0, 10.0),
            Rect::new(10.0, 0.0, 5.0, 5.0)
        ));
        assert!(!rects_overlap(
            Rect::new(0.0, 0.0, 10.0, 10.0),
            Rect::new(20.0, 20.0, 5.0, 5.0)
        ));
    }

    #[test]
    fn negative_dimensions_are_normalized_and_non_finite_bounds_rejected() {
        assert_eq!(
            intersection_rects(
                Rect::new(10.0, 10.0, -10.0, -10.0),
                Rect::new(5.0, 5.0, 10.0, 10.0)
            ),
            Some(Rect::new(5.0, 5.0, 5.0, 5.0))
        );
        assert!(intersection_rects(
            Rect::new(f32::NAN, 0.0, 1.0, 1.0),
            Rect::new(0.0, 0.0, 1.0, 1.0)
        )
        .is_none());
    }
}
