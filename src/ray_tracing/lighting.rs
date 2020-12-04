use crate::core::color::*;
use crate::core::comp::Comp;
use crate::core::vector::Vec4;
use crate::core::matrix::Matrix4x4;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::scene::Scene;
use crate::ray_tracing::intersection::Intersection;

//A Light is either a PointLight or an AreaLight
pub trait Light {
    fn get_intensity(&self) -> &Color;

    fn get_position(&self) -> &Vec4;

    fn light_intensity(&self, point: &Vec4, scene: &Scene) -> f32;
}

//An area light is an array of lights which produce soft shadows
#[derive(Debug, PartialEq)]
pub struct AreaLight {
    pub corner: Vec4, //Position of the bottom left corner
    pub uvec: Vec4, //Vector of the u edge
    pub usteps: usize, //Width separation of lights on the u edge
    pub vvec: Vec4, //Vector of the v edge
    pub vsteps: usize, //Width separation of lights on the v edge
    pub samples: usize,
    pub intensity: Color,
}

impl Light for AreaLight {
    fn get_intensity(&self) -> &Color {
        &self.intensity
    }

    fn get_position(&self) -> &Vec4 {
        &self.corner
    }

    fn light_intensity(&self, _point: &Vec4, _scene: &Scene) -> f32 {
        0.0
    }
}

impl AreaLight {
    //Creates a new AreaLight
    pub fn new(corner: Vec4, full_uvec: Vec4, usteps: i32, full_vvec: Vec4, vsteps: i32, intensity: Color) -> AreaLight {
        AreaLight {
            corner,
            uvec: (1.0 / &(usteps as f32)) * full_uvec,
            usteps: usteps as usize,
            vvec: (1.0 / &(vsteps as f32)) * full_vvec,
            vsteps: vsteps as usize,
            samples: (&vsteps * &usteps) as usize,
            intensity,
        }
    }

    pub fn point_on_light(&self, u: i32, v: i32) -> Vec4 {
        &self.corner + (&self.uvec * ((u as f32) + 0.5)) + (&self.vvec * ((v as f32) + 0.5))
    }   
}
//Light in space with no size
#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Vec4,
}

impl Light for PointLight {
    fn get_intensity(&self) -> &Color {
        &self.intensity
    }

    fn get_position(&self) -> &Vec4 {
        &self.position
    }

    //Finds the intensity of a PointLight at a given point
    fn light_intensity(&self, point: &Vec4, scene: &Scene) -> f32 {
        if in_shadow(&self.position, point, scene) == true {
            0.0
        }
        else {
            1.0
        }
    }
}

impl PointLight {
    //Creates a new PointLnight
    pub fn new(intensity: Color, position: Vec4) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
}

//Creates a ray reflected off of a surface
pub fn reflected_color(scene: &Scene, comps: &Comp, remaining: i32) -> Color {
    if comps.material.reflectivity == 0.0 || remaining <= 0 {
        Color::new(0.0, 0.0, 0.0)
    }
    else {
        let reflected_ray = Ray::new_from_vec(Vec4::new(comps.over_point.0, comps.over_point.1, comps.over_point.2, 1.0), Vec4::new(comps.r_vec.0, comps.r_vec.1, comps.r_vec.2, 0.0));
        let color = Scene::compute_color(reflected_ray, scene, remaining - 1);
        if color != None {
            color.unwrap() * comps.material.reflectivity
        }
        else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}

//Computes a color given all the variables of the environment
pub fn lighting(material: &Material, object_inverse: &Matrix4x4, light: &Box<dyn Light>, point: &Vec4, e_vec: &Vec4, n_vec: &Vec4, light_intensity: f32) -> Color {
    let mut color = material.color.clone();
    let material_pattern = &material.pattern;
    if !material_pattern.is_none() {
        color = material_pattern.as_ref().unwrap().color_at_object(object_inverse, point);
    }
    let mut diffuse = BLACK;
    let mut specular = BLACK;

    //Combines surface and light color
    let effective_color = color * light.get_intensity();

    //Finds the direction to the light source
    let light_vec = (light.get_position() - point).normalize();

    //Computes the ambient value
    let ambient = &effective_color * material.ambient;

    //light_dot_normal represents the cosine between the light and normal vectors 
    let light_dot_normal = Vec4::dot(&light_vec, n_vec);

    //A negative light_dot_normal means the light is obstructed
    if light_dot_normal >= 0.0 {
        diffuse = &effective_color * material.diffuse * light_dot_normal * light_intensity;

        //reflect_dot_eye represents the cosine of the angle between the reflection and eye vectors
        let reflect_vec = Vec4::reflect(&light_vec.negate(), &n_vec);
        let reflect_dot_eye = Vec4::dot(&reflect_vec, &e_vec);
        //A negative light_dot_normal means there is no specular lighting
        if reflect_dot_eye > 0.0 {
            let factor = f32::powf(reflect_dot_eye as f32, material.shininess);
            specular = light.get_intensity() * &material.specular * factor * light_intensity;
        }
    }
    ambient + diffuse + specular
}

//Creates a vector from a point to a given light and tests for intersections within that distance
pub fn in_shadow(light_position: &Vec4, point: &Vec4, scene: &Scene) -> bool {
    let vector = light_position - point;
    let distance = Vec4::magnitude(&vector);
    let direction = (&vector).normalize();
    let shadow_ray = Ray::new_from_vec(Vec4::new(point.0, point.1, point.2, 1.0), direction);
    let intersections = Ray::intersect_scene(scene, shadow_ray);
    let ray_hit = Intersection::hit(intersections);
    let mut result = false;
    if !ray_hit.is_none() {
        if ray_hit.unwrap().t < distance {
            result = true;
        }
    }
    result
}
