use crate::misc::utils::clamp_float;
use std::ops::*;

//Color is a wrapper for Tuple
#[derive(Debug, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    //Creates a new Color
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(r, g, b)
    }

    //clamp_floats colors between 0 and 1
    fn clamp(&self) -> Color {
        Color(clamp_float(self.0, 0.0, 1.0), clamp_float(self.1, 0.0, 1.0), clamp_float(self.2, 0.0, 1.0))
    }

    //Converts the color values to a 0-255 scale
    fn convert(&self) -> Color {
        Color(self.0 * 255.0, self.1 * 255.0, self.2 * 255.0)
    }

    //Takes a color and fully converts it to a string valid for a ppm file
    pub fn ppm_string(&self) -> String {
        let fixed_color = self.clamp().convert();
        format!("{} {} {}", fixed_color.0.round() as i32, fixed_color.1.round() as i32, fixed_color.2.round() as i32)
    }
    
    //Gets the length of a color's ppm string
    pub fn ppm_length(&self) -> i32 {
        (self.ppm_string().len() as i32) + 1
    }

    //Rounds colors for testing
    pub fn round(&self) -> Color {
        Color::new(((self.0 * 10000.0).round())/10000.0, ((self.1 * 10000.0).round())/10000.0, ((self.2 * 10000.0).round())/10000.0)
    }
}

//Color + Color
impl Add for Color {
    type Output = Color;
    
    fn add(self, other: Color) -> Color {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
//&Color + &Color
impl<'a, 'b> Add<&'b Color> for &'a Color {
    type Output = Color;
    
    fn add(self, other: &'b Color) -> Color {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
//&Color + Color
impl<'a> Add<Color> for &'a Color {
    type Output = Color;
    
    fn add(self, other: Color) -> Color {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
//Color + &Color
impl<'a> Add<&'a Color> for Color {
    type Output = Color;
    
    fn add(self, other: &'a Color) -> Color {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

//Color - Color
impl Sub for Color {
    type Output = Color;
    
    fn sub(self, other: Color) -> Color {
        Color(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
//&Color - &Colors
impl<'a, 'b> Sub<&'b Color> for &'a Color {
    type Output = Color;
    
    fn sub(self, other: &'b Color) -> Color {
        Color(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
//&Color - Color
impl<'a> Sub<Color> for &'a Color {
    type Output = Color;
    
    fn sub(self, other: Color) -> Color {
        Color(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
//Color - &Color
impl<'a> Sub<&'a Color> for Color {
    type Output = Color;
    
    fn sub(self, other: &'a Color) -> Color {
        Color(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

//Color * f32
impl Mul<f32> for Color {
    type Output = Color;
    
    fn mul(self, other: f32) -> Color {
        Color(self.0 * other, self.1 * other, self.2 * other)
    }
}
//&Color * &f32
impl<'a, 'b> Mul<&'b f32> for &'a Color {
    type Output = Color;
    
    fn mul(self, other: &'b f32) -> Color {
        Color(self.0 * other, self.1 * other, self.2 * other)
    }
}
//&Color * f32
impl<'a> Mul<f32> for &'a Color {
    type Output = Color;
    
    fn mul(self, other: f32) -> Color {
        Color(self.0 * other, self.1 * other, self.2 * other)
    }
}
//Color * &f32
impl<'a> Mul<&'a f32> for Color {
    type Output = Color;
    
    fn mul(self, other: &'a f32) -> Color {
        Color(self.0 * other, self.1 * other, self.2 * other)
    }
}

//f32 * Color
impl Mul<Color> for f32 {
    type Output = Color;
    
    fn mul(self, other: Color) -> Color {
        Color(other.0 * self, other.1 * self, other.2 * self)
    }
}
//&f32 * &Color
impl<'a, 'b> Mul<&'b Color> for &'a f32 {
    type Output = Color;
    
    fn mul(self, other: &'b Color) -> Color {
        Color(other.0 * self, other.1 * self, other.2 * self)
    }
}
//&f32 * &Color
impl<'a> Mul<Color> for &'a f32 {
    type Output = Color;
    
    fn mul(self, other: Color) -> Color {
        Color(other.0 * self, other.1 * self, other.2 * self)
    }
}
//f32 * &Color
impl<'a> Mul<&'a Color> for f32 {
    type Output = Color;
    
    fn mul(self, other: &'a Color) -> Color {
        Color(other.0 * self, other.1 * self, other.2 * self)
    }
}

//Color * Color
impl Mul for Color {
    type Output = Color;
    
    fn mul(self, other: Color) -> Color {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}
//&Color * &Color
impl<'a, 'b> Mul<&'b Color> for &'a Color {
    type Output = Color;
    
    fn mul(self, other: &'b Color) -> Color {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}
//&Color * Color
impl<'a> Mul<Color> for &'a Color {
    type Output = Color;
    
    fn mul(self, other: Color) -> Color {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}
//Color * &Color
impl<'a> Mul<&'a Color> for Color {
    type Output = Color;
    
    fn mul(self, other: &'a Color) -> Color {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

//Clones Color 
impl Clone for Color {
    fn clone(&self) -> Color {
        Color(self.0, self.1, self.2)
    }
}