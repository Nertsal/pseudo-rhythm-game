use geng::prelude::*;

mod report;

pub use report::*;

pub fn smooth_step<T: Float>(t: T) -> T {
    T::from_f32(3.0) * t * t - T::from_f32(2.0) * t * t * t
}

pub fn king_distance<T: Num + Ord>(vec: vec2<T>) -> T {
    vec.x.abs().max(vec.y.abs())
}

/// Normalizes arbitrary vector to one of 9 possible directions from the grid.
pub fn vec_to_dir(vec: vec2<f32>) -> vec2<i64> {
    let tangent = vec.y.abs() / vec.x.abs().max(1e-2);
    if tangent < 0.41 {
        // Horizontal
        vec2::UNIT_X * vec.x.signum() as i64
    } else if tangent < 2.41 {
        // Diagonal
        vec.map(|x| x.signum() as i64)
    } else {
        // Vertical
        vec2::UNIT_Y * vec.y.signum() as i64
    }
}

/// Calculates distance from a point to a segment.
pub fn dist_to_segment<T: Float>(p: vec2<T>, s: Segment<T>) -> T {
    let dir = s.1 - s.0;
    if dir == vec2::ZERO {
        // Segment is a point
        return (p - s.0).len();
    }

    let t = vec2::dot(p - s.0, p - s.1) / dir.len_sqr();

    if t < T::ZERO {
        // Point is closer to `s.0`
        return (p - s.0).len();
    } else if t > T::ONE {
        // Point is closer to `s.1`
        return (p - s.1).len();
    }

    // Point's projection falls on the segment
    let proj = s.0 + dir * t;
    (p - proj).len()
}
