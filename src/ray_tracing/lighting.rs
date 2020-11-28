use crate::core::color::Color;
use crate::core::vector::Vec4;
use crate::ray_tracing::material::Material;

//Light in space with no size
#[derive(Debug, PartialEq)]
pub struct PointLight {
    intensity: Color,
    position: Vec4,
}

//Computes a color given all the variables of the environment
pub fn lighting(material: &Material, point: &Vec4, light: &PointLight, e_vec: &Vec4, n_vec: &Vec4) -> Color {
    let black = Color::new(0.0, 0.0, 0.0);
    let mut diffuse = black.clone();
    let mut specular = black.clone();

    //Combines surface and light color
    let effective_color = material.get_color() * light.get_intensity();

    //Finds the direction to the light source
    let light_vec = (light.get_position() - point).normalize();

    //Computes the ambient value
    let ambient = &effective_color * material.get_ambient();

    //light_dot_normal represents the cosine between the light and normal vectors 
    let light_dot_normal = Vec4::dot(&light_vec, n_vec);

    //A negative light_dot_normal means the light is obstructed
    if light_dot_normal >= 0.0 {
        diffuse = &effective_color * material.get_diffuse() * (light_dot_normal as f32);

        //reflect_dot_eye represents the cosine of the angle between the reflection and eye vectors
        let reflect_vec = Vec4::reflect(&light_vec.negate(), &n_vec);
        let reflect_dot_eye = Vec4::dot(&reflect_vec, &e_vec);

        //A negative light_dot_normal means there is no specular lighting
        if reflect_dot_eye > 0.0 {
            let factor = f32::powf(reflect_dot_eye as f32, material.get_shininess().clone());
            specular = light.get_intensity() * material.get_specular() * (factor as f32);
        }
    }
    println!("ambient: {:?}, diffuse: {:?}, specular: {:?}", &ambient, &diffuse, &specular);
    ambient + diffuse + specular
}

impl PointLight {
    //Creates a new PointLnight
    pub fn new(intensity: Color, position: Vec4) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }

    //Gets the intensity of a PointLight
    pub fn get_intensity(&self) -> &Color {
        &self.intensity
    }

    //Gets the position of a PointLight
    pub fn get_position(&self) -> &Vec4 {
        &self.position
    }
}

impl Clone for PointLight {
    //Clones a PointLight
    fn clone(&self) -> PointLight {
        PointLight {
            intensity: self.intensity.clone(),
            position: self.position.clone(),
        } 
    }
}