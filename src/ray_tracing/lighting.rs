use crate::core::color::*;
use crate::core::comp::Comp;
use crate::core::vector::Vec4;
use crate::core::matrix::Matrix4x4;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::scene::Scene;
use crate::ray_tracing::intersection::Intersection;

//A Light is either a PointLight or an AreaLight
#[derive(Debug, PartialEq)]
pub enum Light {
    PointLight(PointLight),
    AreaLight(AreaLight),
}

impl Light {
    pub fn get_intensity(&self) -> &Color {
        match self {
            Light::PointLight(point_light) => {
                point_light.get_intensity()
            }
            Light::AreaLight(area_light) => {
                area_light.get_intensity() 
            }
        }
    }

    pub fn get_position(&self) -> &Vec4 {
        match self {
            Light::PointLight(point_light) => {
                point_light.get_position()
            }
            Light::AreaLight(_) => {
                panic!("Cannot call get_position() on an Area Light");
            }
        }
    }

    pub fn get_corner(&self) -> &Vec4 {
        match self {
            Light::PointLight(_) => {
                panic!("Cannot call get_corner() on a Point Light");
            }
            Light::AreaLight(area_light) => {
                area_light.get_corner()
            }
        }
    }

    pub fn get_uvec(&self) -> &Vec4 {
        match self {
            Light::PointLight(_) => {
                panic!("Cannot call get_uvec() on a Point Light");
            }
            Light::AreaLight(area_light) => {
                area_light.get_uvec()
            }
        }
    }

    pub fn get_vvec(&self) -> &Vec4 {
        match self {
            Light::PointLight(_) => {
                panic!("Cannot call get_vvec() on a Point Light");
            }
            Light::AreaLight(area_light) => {
                area_light.get_vvec()
            }
        }
    }

    pub fn get_usteps(&self) -> &usize {
        match self {
            Light::PointLight(_) => {
                panic!("Cannot call get_usteps() on a Point Light");
            }
            Light::AreaLight(area_light) => {
                area_light.get_usteps()
            }
        }
    }

    pub fn get_vsteps(&self) -> &usize {
        match self {
            Light::PointLight(_) => {
                panic!("Cannot call get_vsteps() on a Point Light");
            }
            Light::AreaLight(area_light) => {
                area_light.get_vsteps()
            }
        }
    }

    pub fn get_samples(&self) -> &usize {
        match self {
            Light::PointLight(_) => {
                panic!("Cannot call get_samples() on a Point Light");
            }
            Light::AreaLight(area_light) => {
                area_light.get_samples()
            }
        }
    }
}

//An area light is an array of lights which produce soft shadows
#[derive(Debug, PartialEq)]
pub struct AreaLight {
    corner: Vec4, //Position of the bottom left corner
    uvec: Vec4, //Vector of the u edge
    usteps: usize, //Width separation of lights on the u edge
    vvec: Vec4, //Vector of the v edge
    vsteps: usize, //Width separation of lights on the v edge
    samples: usize,
    intensity: Color,
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

    //Gets the AreaLight corner
    pub fn get_corner(&self) -> &Vec4 {
        &self.corner
    }

    //Gets the AreaLight uvec
    pub fn get_uvec(&self) -> &Vec4 {
        &self.uvec
    }

    //Gets the AreaLight usteps
    pub fn get_usteps(&self) -> &usize {
        &self.usteps
    }

    //Gets the AreaLight vvec
    pub fn get_vvec(&self) -> &Vec4 {
        &self.vvec
    }

    //Gets the AreaLight vsteps
    pub fn get_vsteps(&self) -> &usize {
        &self.vsteps
    }

    //Gets the AreaLight intensity
    pub fn get_intensity(&self) -> &Color {
        &self.intensity
    }

    //Gets the AreaLight intensity
    pub fn get_samples(&self) -> &usize {
        &self.samples
    }
}

//Light in space with no size
#[derive(Debug, PartialEq)]
pub struct PointLight {
    intensity: Color,
    position: Vec4,
}

impl PointLight {
    //Creates a new PointLnight
    pub fn new(intensity: Color, position: Vec4) -> Light {
        Light::PointLight(PointLight {
            intensity,
            position,
        })
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

//Creates a ray reflected off of a surface
pub fn reflected_color(scene: &'static Scene, comps: &Comp, remaining: i32) -> Color {
    if comps.material.get_reflectivity() == &0.0 || remaining <= 0 {
        Color::new(0.0, 0.0, 0.0)
    }
    else {
        let reflected_ray = Ray::new_from_vec(Vec4::new(comps.over_point.0, comps.over_point.1, comps.over_point.2, 1.0), Vec4::new(comps.r_vec.0, comps.r_vec.1, comps.r_vec.2, 0.0));
        let color = Scene::compute_color(reflected_ray, scene, remaining - 1);
        if color != None {
            color.unwrap() * comps.material.get_reflectivity()
        }
        else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}

//Computes a color given all the variables of the environment
pub fn lighting(material: &Material, object_inverse: &Matrix4x4, light: &Light, point: &Vec4, e_vec: &Vec4, n_vec: &Vec4, light_intensity: f32) -> Color {
    let mut color = material.get_color().clone();
    let material_pattern = &material.pattern;
    if !material_pattern.is_none() {
        color = material.get_pattern().as_ref().unwrap().color_at_object(object_inverse, point);
    }
    let mut diffuse = BLACK;
    let mut specular = BLACK;

    //Combines surface and light color
    let effective_color = color * light.get_intensity();

    //Finds the direction to the light source
    let light_vec = (light.get_position() - point).normalize();

    //Computes the ambient value
    let ambient = &effective_color * material.get_ambient();

    //light_dot_normal represents the cosine between the light and normal vectors 
    let light_dot_normal = Vec4::dot(&light_vec, n_vec);

    //A negative light_dot_normal means the light is obstructed
    if light_dot_normal >= 0.0 {
        diffuse = &effective_color * material.get_diffuse() * light_dot_normal * light_intensity;

        //reflect_dot_eye represents the cosine of the angle between the reflection and eye vectors
        let reflect_vec = Vec4::reflect(&light_vec.negate(), &n_vec);
        let reflect_dot_eye = Vec4::dot(&reflect_vec, &e_vec);

        //A negative light_dot_normal means there is no specular lighting
        if reflect_dot_eye > 0.0 {
            let factor = f32::powf(reflect_dot_eye as f32, material.get_shininess().clone());
            specular = light.get_intensity() * material.get_specular() * factor * light_intensity;
        }
    }
    ambient + diffuse + specular
}

//Creates a vector from a point to a given light and tests for intersections within that distance
pub fn in_shadow(light_position: &Vec4, point: &Vec4, scene: &'static Scene) -> bool {
    let vector = light_position - point;
    let distance = Vec4::magnitude(&vector);
    let direction = (&vector).normalize();
    let shadow_ray = Ray::new_from_vec(Vec4::new(point.0, point.1, point.2, 1.0), direction);
    let intersections = Ray::intersect_scene(scene, shadow_ray);
    let ray_hit = Intersection::hit(intersections);
    let mut result = false;
    if !ray_hit.is_none() {
        if ray_hit.unwrap().get_t() < distance {
            result = true;
        }
    }
    result
}

//Finds the intensity of a light at a given point
pub fn light_intensity(light: &Light, point: &Vec4, scene: &'static Scene) -> f32 {
    match light {
        Light::PointLight(point_light) => {
            if in_shadow(point_light.get_position(), point, scene) == true {
                0.0
            }
            else {
                1.0
            }
        }
        _ => 0.0
    }
}