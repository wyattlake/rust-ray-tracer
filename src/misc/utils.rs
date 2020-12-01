//Increases epsilon slightly to account for rounding errors
pub const EPSILON_BUMP: f32 = f32::EPSILON + 0.001;

//Clamps a given float
pub fn clamp_float(num: f32, min: f32, max: f32) -> f32 {
    if num > max {
        max
    }
    else if num < min {
        min
    }
    else {
        num
    }
}