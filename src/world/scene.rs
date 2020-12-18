use crate::core::color::Color;
use crate::core::comp::Comp;
use crate::core::matrix::Matrix4x4;
use crate::core::sequence::Sequence;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::objects::sphere::Sphere;
use crate::ray_tracing::intersection::Intersection;
use crate::world::lighting::*;
use crate::materials::material::Material;
use crate::ray_tracing::ray::Ray;

pub struct Scene {
    pub light_sources: Vec<Box<dyn Light>>,
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

    //Lights a pixel in the scene
    pub fn scene_lighting(
        scene: &Scene,
        comps: &Comp,
        remaining: i32,
        offset: &mut Sequence,
    ) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for light in &scene.light_sources {
            let light_intensity = light.light_intensity(&comps.over_point, &scene, offset);
            color = color
                + lighting(
                    &comps.material,
                    &comps.object_inverse,
                    &light,
                    &comps.over_point,
                    &comps.e_vec,
                    &comps.n_vec,
                    light_intensity,
                );
        }
        let reflected = reflected_color(&scene, comps, remaining, offset);
        color + reflected
    }

    //Computes the color at a given point
    pub fn compute_color(
        ray: Ray,
        scene: &Scene,
        remaining: i32,
        offset: &mut Sequence,
    ) -> Option<Color> {
        let intersections = Ray::intersect_scene(&scene, ray.clone());
        let hit = Intersection::hit(&intersections);
        if !hit.is_none() {
            let comps = Comp::compute_vars(hit.unwrap(), &ray, &intersections);
            let color = Scene::scene_lighting(scene, &comps, remaining, offset);
            Some(color)
        } else {
            None
        }
    }

    //Creates a default Scene
    pub fn default() -> Scene {
        let scene = Scene {
            light_sources: vec![Box::new(PointLight::new(
                Color::new(1.0, 1.0, 1.0),
                Vec4::new(-10.0, 10.0, -10.0, 1.0),
            ))],
            objects: vec![
                Box::new(Sphere::new(
                    Matrix4x4::identity(),
                    Material::new(
                        Color::new(0.8, 1.0, 0.6),
                        0.1,
                        0.7,
                        0.2,
                        200.0,
                        0.0,
                        0.0,
                        0.0,
                        true,
                        None,
                    ),
                )),
                Box::new(Sphere::new(
                    Matrix4x4::scaling(0.5, 0.5, 0.5),
                    Material::default(),
                )),
            ],
        };
        scene
    }

    //Gets the color without any lighting calculations
    pub fn compute_color_quick(ray: Ray, scene: &Scene) -> Option<Color> {
        let intersections = Ray::intersect_scene(scene, ray);
        let hit = Intersection::hit(&intersections);
        if !hit.is_none() {
            let unwrapped = hit.unwrap();
            let color = &unwrapped.object.get_material().color;
            Some(color.clone())
        } else {
            None
        }
    }
}
