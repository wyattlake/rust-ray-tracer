use crate::core::color::Color;
use crate::misc::utils::clamp_float;
use crate::ray_tracing::patterns::*;

#[derive(Debug, PartialEq)]
//A Material holds a bunch of properties for an object
//Lighting properties are based on the Phong Reflection Model
pub struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    pattern: Option<Pattern>,
}

impl Material {
    //Creates a new Material and clamps all the values
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32, pattern: Option<Pattern>) -> Material {
        Material {
            color: color,
            ambient: clamp_float(ambient, 0.0, 1.0),
            diffuse: clamp_float(diffuse, 0.0, 1.0),
            specular: clamp_float(specular, 0.0, 1.0),
            shininess: clamp_float(shininess, 1.0, 200.0),
            pattern,
        }
    }

    //Sets a material
    pub fn set(&mut self, color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32, pattern: Option<Pattern>) {
        self.color = color;
        self.ambient = clamp_float(ambient, 0.0, 1.0); 
        self.diffuse = clamp_float(diffuse, 0.0, 1.0);
        self.specular = clamp_float(specular, 0.0, 1.0);
        self.shininess = clamp_float(shininess, 1.0, 200.0);
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
            pattern: None,
        } 
    }

    //Gets the color of a Material
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    //Sets the color of a Material
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    //Gets the ambient value of a Material
    pub fn get_ambient(&self) -> &f32 {
        &self.ambient
    }

    //Sets the ambient of a Material
    pub fn set_ambient(&mut self, ambient: f32) {
        self.ambient = ambient;
    }

    //Gets the diffuse value of a Material
    pub fn get_diffuse(&self) -> &f32 {
        &self.diffuse
    }

    //Sets the diffuse of a Material
    pub fn set_diffuse(&mut self, diffuse: f32) {
        self.diffuse = diffuse;
    }

    //Gets the specular value of a Material
    pub fn get_specular(&self) -> &f32 {
        &self.specular
    }

    //Sets the specular of a Material
    pub fn set_specular(&mut self, specular: f32) {
        self.specular = specular;
    }

    //Gets the shininess value of a Material
    pub fn get_shininess(&self) -> &f32 {
        &self.shininess
    }

    //Sets the shininess of a Material
       pub fn set_shininess(&mut self, shininess: f32) {
        self.specular = shininess;
    } 

    //Gets the pattern of a Material
    pub fn get_pattern(&self) -> &Option<Pattern> {
        &self.pattern
    }

    //Sets the pattern of a Material
    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = Some(pattern);
    } 
}

impl Clone for Material {
    fn clone(&self) -> Material {
        Material::new(self.color.clone(), self.ambient.clone(), self.diffuse.clone(), self.specular.clone(), self.shininess.clone(), self.pattern.clone())
    }
}