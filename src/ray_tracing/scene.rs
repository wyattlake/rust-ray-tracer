use crate::core::matrix::Matrix4x4;
use crate::core::color::Color;
use crate::core::vector::Vec4;
use crate::objects::sphere::Sphere;
use crate::ray_tracing::lighting::*;
use crate::core::comp::Comp;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;
use crate::objects::general::{ObjectMethods, Object};
use std::rc::Rc;

pub struct Scene {
    light_sources: Vec<PointLight>,
    objects: Vec<Rc<Object>>,
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
    pub fn add_light(&mut self, light: PointLight) {
        self.light_sources.push(light); 
    }

    //Pushes a light source to the Scene
    pub fn clear_lights(&mut self) {
        self.light_sources.clear(); 
    }

    //Gets the Scene's light source
    pub fn get_light_sources(&self) -> &Vec<PointLight> {
       &self.light_sources
    }

    //Pushes an object to the scene
    pub fn add_object(&mut self, object: Rc<Object>) {
        self.objects.push(object);
    }

    //Gets the Scene's objects
    pub fn get_objects(&self) -> &Vec<Rc<Object>> {
        &self.objects
    }

    //Clears the objects in a scene
    pub fn clear_objects(&mut self) {
        self.objects.clear(); 
    }

    //Creates a default Scene
    pub fn default() -> Scene {
        let mut scene = Scene {
            light_sources: vec![PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -10.0, 1.0))],
            objects: vec![],
        };
        let mut sphere1_raw = Sphere::new_raw(); 
        &sphere1_raw.get_mut_material().set_color(Color::new(0.8, 1.0, 0.6));
        &sphere1_raw.get_mut_material().set_diffuse(0.7);
        &sphere1_raw.get_mut_material().set_specular(0.2);
        let sphere1 = Rc::new(sphere1_raw);
        let mut sphere2_raw = Sphere::new_raw(); 
        &sphere2_raw.transform(Matrix4x4::scaling(0.5, 0.5, 0.5));
        let sphere2 = Rc::new(sphere2_raw);
        &scene.add_object(sphere1); 
        &scene.add_object(sphere2); 
        scene
    }

    //Lights a pixel in the scene
    pub fn scene_lighting(scene: &Scene, comps: Comp) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for light in scene.get_light_sources() {
            let shadow = in_shadow(&light, &comps.over_point, scene);
            color = color + lighting(comps.object.get_material(), &comps.object, &light, &comps.over_point, &comps.e_vec, &comps.n_vec, shadow);
        }
        color
    }

    //Computes the color at a given point
    pub fn compute_color(ray: Ray, scene: &Scene) -> Option<Color> {
        let intersections = Ray::intersect_scene(scene, &ray);
        let hit = Intersection::hit(&intersections);
        if hit != None {
            let comps = Comp::compute_vars(hit.unwrap(), &ray);
            let color = Scene::scene_lighting(&scene, comps);
            Some(color)
        }
        else {
            None
        }
    }

    //Gets the color without any lighting calculations
    pub fn compute_color_quick(ray: Ray, scene: &Scene) -> Option<Color> {
        let intersections = Ray::intersect_scene(scene, &ray);
        let hit = Intersection::hit(&intersections);
        if hit != None {
            let object = hit.unwrap().get_object();
            let color = object.get_material().get_color();
            Some(color.clone())
        }
        else {
            None
        }
    }
}
