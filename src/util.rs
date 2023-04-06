use geng::prelude::*;

pub fn smooth_step<T: Float>(t: T) -> T {
    T::from_f32(3.0) * t * t - T::from_f32(2.0) * t * t * t
}

pub fn king_distance<T: Num + Ord>(vec: vec2<T>) -> T {
    vec.x.abs().max(vec.y.abs())
}

pub fn report_err<T, E: Display>(result: Result<T, E>) {
    if let Err(error) = result {
        error!("Error: {error}");
    }
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
