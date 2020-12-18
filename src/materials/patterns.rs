use crate::core::color::Color;
use crate::core::vector::Vec4;
use crate::core::matrix::Matrix4x4;
use std::fmt::Debug;
use std::any::Any;

//Generic enum pattern which matches to specific patterns
pub trait Pattern: Debug + PatternClone {
    //Gets the color at a point on the pattern
    fn color_at(&self, point: &Vec4) -> Color;
    
    //Transforms a pattern
    fn transform(&mut self, matrix: Matrix4x4);

    //Finds the color on a pattern given the transformations of the matrix the pattern is on
    fn color_at_object(&self, object_inverse: &Matrix4x4, point: &Vec4) -> Color;

    //Methods used to allow PartialEq between objects
    fn eq(&self, other: &dyn Pattern) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<'a, 'b> PartialEq<dyn Pattern+'b> for dyn Pattern+'a {
    fn eq(&self, other: &(dyn Pattern+'b)) -> bool {
        Pattern::eq(self, other)
    }
}

pub trait PatternClone {
    fn clone_box(&self) -> Box<dyn Pattern>;
}

impl<T> PatternClone for T where T: 'static + Pattern + Clone, {
    fn clone_box(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Pattern> {
    fn clone(&self) -> Box<dyn Pattern> {
        self.clone_box()
    }
}

//A striped pattern
#[derive(Debug, PartialEq, Clone)]
pub struct StripePattern {
    colors: (Color, Color),
    transform: Matrix4x4,
    inverse: Matrix4x4,
}

impl StripePattern {
    pub fn new(color1: Color, color2: Color, transform: Matrix4x4) -> StripePattern {
        StripePattern {
            inverse: transform.inverse().unwrap(),
            colors: (color1, color2),
            transform,
        }
    }
}

impl Pattern for StripePattern  {
    //Creates a new StripePattern}
    //Gets the color at a specific point
    fn color_at(&self, point: &Vec4) -> Color {
        if point.0.floor() % 2.0 == 0.0 {
            self.colors.0.clone()
        }
        else {
            self.colors.1.clone()
        }
    }

    //Transforms the pattern
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Gets the color at a specific point taking into account pattern and object transformations
    fn color_at_object(&self, object_inverse: &Matrix4x4, point: &Vec4) -> Color {
        let object_point = object_inverse * point;
        let pattern_point = &self.inverse * object_point;
        self.color_at(&pattern_point)
    }

    fn eq(&self, other: &dyn Pattern) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any { self }
}

//A gradient pattern
#[derive(Debug, PartialEq, Clone)]
pub struct GradientPattern {
    colors: (Color, Color),
    transform: Matrix4x4,
    inverse: Matrix4x4,
}

impl GradientPattern {
    //Creates a new StripePattern
    pub fn new(color1: Color, color2: Color, transform: Matrix4x4) -> GradientPattern {
        GradientPattern {
            inverse: transform.inverse().unwrap(),
            colors: (color1, color2),
            transform,
        }
    }
}

impl Pattern for GradientPattern  {
    //Gets the color at a specific point
    fn color_at(&self, point: &Vec4) -> Color {
        if point.0.floor() % 2.0 == 0.0 {
            let distance = &self.colors.1 - &self.colors.0;
            let fraction = point.0 - point.0.floor();
            let result = &self.colors.0 + ((fraction as f32) * distance);
            result
        }
        else {
            let distance = &self.colors.0 - &self.colors.1;
            let fraction = point.0 - point.0.floor();
            let result = &self.colors.1 + ((fraction as f32) * distance);
            result
        }

    }

    //Transforms the pattern
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Gets the color at a specific point taking into account pattern and object transformations
    fn color_at_object(&self, object_inverse: &Matrix4x4, point: &Vec4) -> Color {
        let object_point = object_inverse * point;
        let pattern_point = &self.inverse * object_point;
        self.color_at(&pattern_point)
    }

    fn eq(&self, other: &dyn Pattern) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any { self }
}

//A ring pattern
#[derive(Debug, PartialEq, Clone)]
pub struct RingPattern {
    colors: (Color, Color),
    transform: Matrix4x4,
    inverse: Matrix4x4,
}

impl RingPattern {
    //Creates a new StripePattern
    pub fn new(color1: Color, color2: Color, transform: Matrix4x4) -> RingPattern {
        RingPattern {
            inverse: transform.inverse().unwrap(),
            colors: (color1, color2),
            transform,
        }
    }
}

impl Pattern for RingPattern  {
    //Gets the color at a specific point
    fn color_at(&self, point: &Vec4) -> Color {
        if (((point.0 * point.0) + (point.2 * point.2)).sqrt()).floor() % 2.0 == 0.0 {
            self.colors.0.clone()
        }
        else {
            self.colors.1.clone()
        }
    }

    //Transforms the pattern
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Gets the color at a specific point taking into account pattern and object transformations
    fn color_at_object(&self, object_inverse: &Matrix4x4, point: &Vec4) -> Color {
        let object_point = object_inverse * point;
        let pattern_point = &self.inverse * object_point;
        self.color_at(&pattern_point)
    }

    fn eq(&self, other: &dyn Pattern) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CheckerboardPattern {
    colors: (Color, Color),
    transform: Matrix4x4,
    inverse: Matrix4x4,
}

impl CheckerboardPattern {
    //Creates a new StripePattern
    pub fn new(color1: Color, color2: Color, transform: Matrix4x4) -> CheckerboardPattern {
        CheckerboardPattern {
            inverse: transform.inverse().unwrap(),
            colors: (color1, color2),
            transform,
        }
    }
}

impl Pattern for CheckerboardPattern  {

    //Gets the color at a specific point
    fn color_at(&self, point: &Vec4) -> Color {
        if (point.0.floor() + point.1.floor() + point.2.floor()) % 2.0 == 0.0 {
            self.colors.0.clone()
        }
        else {
            self.colors.1.clone()
        }
    }

    //Transforms the pattern
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Gets the color at a specific point taking into account pattern and object transformations
    fn color_at_object(&self, object_inverse: &Matrix4x4, point: &Vec4) -> Color {
        let object_point = object_inverse * point;
        let pattern_point = &self.inverse * object_point;
        self.color_at(&pattern_point)
    }

    fn eq(&self, other: &dyn Pattern) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any { self }
}