use crate::core::color::Color;
use crate::misc::utils::clamp_float;
use crate::ray_tracing::patterns::*;

//A Material holds a bunch of properties for an object
//Lighting properties are based on the Phong Reflection Model
#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectivity: f32,
    pub pattern: Option<Box<dyn Pattern>>,
}

impl Material {
    //Creates a new Material and clamps all the values
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32, reflectivity: f32, pattern: Option<Box<dyn Pattern>>) -> Material {
        Material {
            color: color,
            ambient: clamp_float(ambient, 0.0, 1.0),
            diffuse: clamp_float(diffuse, 0.0, 1.0),
            specular: clamp_float(specular, 0.0, 1.0),
            shininess: clamp_float(shininess, 1.0, 200.0),
            reflectivity: reflectivity,
            pattern,
        }
    }

    //Sets a material
    pub fn set(&mut self, color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32, reflectivity: f32, pattern: Option<Box<dyn Pattern>>) {
        self.color = color;
        self.ambient = clamp_float(ambient, 0.0, 1.0); 
        self.diffuse = clamp_float(diffuse, 0.0, 1.0);
        self.specular = clamp_float(specular, 0.0, 1.0);
        self.shininess = clamp_float(shininess, 1.0, 200.0);
        self.reflectivity = reflectivity;
        self.pattern = pattern;
    }

    //Creates a material with default values
    pub fn default() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectivity: 0.0,
            pattern: None,
        } 
    }
}