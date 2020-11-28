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