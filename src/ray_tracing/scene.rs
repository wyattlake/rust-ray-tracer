use crate::ray_tracing::lighting::PointLight;
use crate::core::matrix::Matrix4x4;
use crate::core::color::Color;
use crate::core::vector::Vec4;
use crate::objects::sphere::Sphere;
use std::rc::Rc;

pub struct Scene {
    light_source: Option<PointLight>,
    objects: Vec<Rc<Sphere>>,
}

impl Scene {
    //Creates a new Scene
    pub fn new() -> Scene {
        Scene {
            light_source: None,
            objects: vec![],
        }
    }

    //Sets the Scene's light source
    pub fn set_light(&mut self, light: &PointLight) {
        self.light_source = Some(light.clone());
    }

    //Pushes an object to the scene
    pub fn push_object(&mut self, object: Rc<Sphere>) {
        self.objects.push(object);
    }

    //Creates a default Scene
    pub fn default() -> Scene {
        let mut scene = Scene {
            light_source: Some(PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -10.0, 0.0))),
            objects: vec![],
        };
        let sphere1 = Sphere::new(); 
        let mut sphere2_raw = Sphere::new_raw(); 
        &sphere2_raw.transform(Matrix4x4::scaling(0.5, 0.5, 0.5));
        let sphere2 = Rc::new(sphere2_raw);
        &scene.push_object(sphere1); 
        &scene.push_object(sphere2); 
        scene
    }
}
