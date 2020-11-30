use crate::core::color::Color;
use crate::core::vector::Vec4;
use crate::core::matrix::Matrix4x4;
use crate::objects::general::*;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
//Generic enum pattern which matches to specific patterns
pub enum Pattern {
    StripePattern(StripePattern),
    GradientPattern(GradientPattern),
    RingPattern(RingPattern),
    CheckerboardPattern(CheckerboardPattern),
}

impl Pattern {
    //Calls color_at with the correct pattern
    pub fn color_at(&self, point: &Vec4) -> Color {
        match self {
            Pattern::StripePattern(stripe_pattern) => {
                StripePattern::color_at(stripe_pattern, point) 
            }
            Pattern::GradientPattern(gradient_pattern) => {
                GradientPattern::color_at(gradient_pattern, point) 
            }
            Pattern::RingPattern(ring_pattern) => {
                RingPattern::color_at(ring_pattern, point) 
            }
            Pattern::CheckerboardPattern(checkerboard_pattern) => {
                CheckerboardPattern::color_at(checkerboard_pattern, point) 
            }
        }
    }

    //Calls color_at_object with the correct pattern
    pub fn color_at_object(&self, object: &Rc<Object>, point: &Vec4) -> Color {
        match self {
            Pattern::StripePattern(stripe_pattern) => {
                StripePattern::color_at_object(stripe_pattern, object, point)
            } 
            Pattern::GradientPattern(gradient_pattern) => {
                GradientPattern::color_at_object(gradient_pattern, object, point)
            }
            Pattern::RingPattern(ring_pattern) => {
                RingPattern::color_at_object(ring_pattern, object, point)
            }
            Pattern::CheckerboardPattern(checkerboard_pattern) => {
                CheckerboardPattern::color_at_object(checkerboard_pattern, object, point)
            }
        }
    }

    //Calls transform with the correct pattern
    pub fn transform(&mut self, matrix: Matrix4x4) {
        match self {
            Pattern::StripePattern(stripe_pattern) => {
                stripe_pattern.transform(matrix);
            } 
            Pattern::GradientPattern(gradient_pattern) => {
                gradient_pattern.transform(matrix);
            } 
            Pattern::RingPattern(ring_pattern) => {
                ring_pattern.transform(matrix);
            } 
            Pattern::CheckerboardPattern(checkerboard_pattern) => {
                checkerboard_pattern.transform(matrix);
            } 
        }
    }
}

pub trait PatternMethods {
    fn new(color1: Color, color2: Color) -> Pattern; 
    fn color_at(pattern: &Self, point: &Vec4) -> Color;
    fn transform(&mut self, matrix: Matrix4x4);
    fn color_at_object(pattern: &Self, object: &Rc<Object>, point: &Vec4) -> Color;
}

//A striped pattern
#[derive(Debug, PartialEq, Clone)]
pub struct StripePattern {
    colors: (Color, Color),
    transform: Matrix4x4,
}

impl PatternMethods for StripePattern  {
    //Creates a new StripePattern
    fn new(color1: Color, color2: Color) -> Pattern {
        Pattern::StripePattern(StripePattern {
            colors: (color1, color2),
            transform: Matrix4x4::identity(),
        })
    }

    //Gets the color at a specific point
    fn color_at(pattern: & StripePattern, point: &Vec4) -> Color {
        if point.0.floor() % 2.0 == 0.0 {
            pattern.colors.0.clone()
        }
        else {
            pattern.colors.1.clone()
        }
    }

    //Transforms the pattern
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Gets the color at a specific point taking into account pattern and object transformations
    fn color_at_object(pattern: &StripePattern, object: &Rc<Object>, point: &Vec4) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * point;
        let pattern_point = pattern.transform.inverse().unwrap() * object_point;
        StripePattern::color_at(pattern, &pattern_point)
    }
}

//A gradient pattern
#[derive(Debug, PartialEq, Clone)]
pub struct GradientPattern {
    colors: (Color, Color),
    transform: Matrix4x4,
}

impl PatternMethods for GradientPattern  {
    //Creates a new StripePattern
    fn new(color1: Color, color2: Color) -> Pattern {
        Pattern::GradientPattern(GradientPattern {
            colors: (color1, color2),
            transform: Matrix4x4::identity(),
        })
    }

    //Gets the color at a specific point
    fn color_at(pattern: &GradientPattern, point: &Vec4) -> Color {
        if point.0.floor() % 2.0 == 0.0 {
            let distance = &pattern.colors.1 - &pattern.colors.0;
            let fraction = point.0 - point.0.floor();
            let result = &pattern.colors.0 + ((fraction as f32) * distance);
            result
        }
        else {
            let distance = &pattern.colors.0 - &pattern.colors.1;
            let fraction = point.0 - point.0.floor();
            let result = &pattern.colors.1 + ((fraction as f32) * distance);
            result
        }

    }

    //Transforms the pattern
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Gets the color at a specific point taking into account pattern and object transformations
    fn color_at_object(pattern: &GradientPattern, object: &Rc<Object>, point: &Vec4) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * point;
        let pattern_point = pattern.transform.inverse().unwrap() * object_point;
        GradientPattern::color_at(pattern, &pattern_point)
    }
}

//A ring pattern
#[derive(Debug, PartialEq, Clone)]
pub struct RingPattern {
    colors: (Color, Color),
    transform: Matrix4x4,
}

impl PatternMethods for RingPattern  {
    //Creates a new StripePattern
    fn new(color1: Color, color2: Color) -> Pattern {
        Pattern::RingPattern(RingPattern {
            colors: (color1, color2),
            transform: Matrix4x4::identity(),
        })
    }

    //Gets the color at a specific point
    fn color_at(pattern: &RingPattern, point: &Vec4) -> Color {
        if (((point.0 * point.0) + (point.2 * point.2)).sqrt()).floor() % 2.0 == 0.0 {
            pattern.colors.0.clone()
        }
        else {
            pattern.colors.1.clone()
        }
    }

    //Transforms the pattern
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Gets the color at a specific point taking into account pattern and object transformations
    fn color_at_object(pattern: &RingPattern, object: &Rc<Object>, point: &Vec4) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * point;
        let pattern_point = pattern.transform.inverse().unwrap() * object_point;
        RingPattern::color_at(pattern, &pattern_point)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CheckerboardPattern {
    colors: (Color, Color),
    transform: Matrix4x4,
}

impl PatternMethods for CheckerboardPattern  {
    //Creates a new StripePattern
    fn new(color1: Color, color2: Color) -> Pattern {
        Pattern::CheckerboardPattern(CheckerboardPattern {
            colors: (color1, color2),
            transform: Matrix4x4::identity(),
        })
    }

    //Gets the color at a specific point
    fn color_at(pattern: &CheckerboardPattern, point: &Vec4) -> Color {
        if (point.0.floor() + point.1.floor() + point.2.floor()) % 2.0 == 0.0 {
            pattern.colors.0.clone()
        }
        else {
            pattern.colors.1.clone()
        }
    }

    //Transforms the pattern
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Gets the color at a specific point taking into account pattern and object transformations
    fn color_at_object(pattern: &CheckerboardPattern, object: &Rc<Object>, point: &Vec4) -> Color {
        let object_point = object.get_transform().inverse().unwrap() * point;
        let pattern_point = pattern.transform.inverse().unwrap() * object_point;
        CheckerboardPattern::color_at(pattern, &pattern_point)
    }
}