use crate::core::matrix::Matrix4x4;
use crate::core::color::Color;
use crate::core::vector::Vec4;
use crate::objects::sphere::Sphere;
use crate::ray_tracing::lighting::*;
use crate::core::comp::Comp;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::intersection::Intersection;
use crate::objects::object::*;

pub struct Scene {
    pub light_sources: Vec<Light>,
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    //Creates a new Scene
    pub fn new() -> Scene {
        Scene {
            light_sources: vec![],
            objects: vec![],
        }
    }

    //Pushes a light source to the Scene
    pub fn add_light(&mut self, light: Light) {
        self.light_sources.push(light); 
    }

    //Pushes a light source to the Scene
    pub fn clear_lights(&mut self) {
        self.light_sources.clear(); 
    }

    //Gets the Scene's light source
    pub fn get_light_sources(&self) -> &Vec<Light> {
       &self.light_sources
    }
    
    //Gets the Scene's objects
    pub fn get_objects(&self) -> &[Box<dyn Object>] {
        &self.objects
    }

    //Clears the objects in a scene
    pub fn clear_objects(&mut self) {
        self.objects.clear(); 
    }

    //Creates a default Scene
    pub fn default() -> Scene {
        let scene = Scene {
            light_sources: vec![PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -10.0, 1.0))],
            objects: vec![
                Box::new(Sphere::new(Matrix4x4::identity(), Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, 0.0, None))),
                Box::new(Sphere::new(Matrix4x4::scaling(0.5, 0.5, 0.5), Material::default()))
            ],
        };
        scene
    }

    //Lights a pixel in the scene
    pub fn scene_lighting(scene: Scene, comps: &Comp, remaining: i32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for light in scene.get_light_sources() {
            let light_intensity = light_intensity(&light, &comps.over_point, &scene);
            color = color + lighting(comps.material, &comps.object_inverse, &light, &comps.over_point, &comps.e_vec, &comps.n_vec, light_intensity);
        }
        let reflected = reflected_color(&scene, comps, remaining);
        color + reflected
    }

    //Computes the color at a given point
    pub fn compute_color(ray: Ray, scene: Scene, remaining: i32) -> Option<Color> {
        let intersections = Ray::intersect_scene(&scene, ray.clone());
        let hit = Intersection::hit(intersections);
        if !hit.is_none() {
            let comps = Comp::compute_vars(hit.unwrap(), &ray);
            let color = Scene::scene_lighting(scene, &comps, remaining);
            Some(color)
        }
        else {
            None
        }
    }

    //Gets the color without any lighting calculations
    pub fn compute_color_quick(ray: Ray, scene: &Scene) -> Option<Color> {
        let intersections = Ray::intersect_scene(scene, ray);
        let hit = Intersection::hit(intersections);
        if !hit.is_none() {
            let unwrapped = hit.unwrap();
            let color = unwrapped.get_material().get_color();
            Some(color.clone())
        }
        else {
            None
        }
    }
}
