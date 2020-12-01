#[cfg(test)]

mod tests {
    use rust_ray_tracer::ray_tracing::intersection::Intersection;
    use rust_ray_tracer::core::color::*;
    use rust_ray_tracer::ray_tracing::ray::Ray;
    use rust_ray_tracer::objects::plane::Plane;
    use rust_ray_tracer::objects::sphere::Sphere;
    use rust_ray_tracer::ray_tracing::scene::Scene;
    use rust_ray_tracer::ray_tracing::lighting::*;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::core::comp::Comp;
    use rust_ray_tracer::objects::general::ObjectMethods;
    use std::rc::Rc;

    //Tests reflection vector calculation
    #[test]
    fn reflection_vector_test() {
        let shape = Plane::new();
        let ray = Ray::new((0.0, 1.0, -1.0), (0.0, (-(2.0 as f32).sqrt()) / 2.0, ((2.0 as f32).sqrt()) / 2.0));
        let intersection = Intersection::new((2.0 as f32).sqrt(), shape);
        let comps = Comp::compute_vars(intersection, &ray);
        assert_eq!(comps.r_vec, Vec4::new(0.0, ((2.0 as f32).sqrt()) / 2.0, ((2.0 as f32).sqrt()) / 2.0, 0.0))
    }

    //Tests reflected color when the ray hits an object which isn't reflective
    #[test]
    fn non_reflective_intersection() {
        let mut scene = Scene::new();
        &scene.add_light(PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -10.0, 1.0)));
        let mut sphere1_raw = Sphere::new_raw(); 
        &sphere1_raw.get_mut_material().set_color(Color::new(0.8, 1.0, 0.6));
        &sphere1_raw.get_mut_material().set_diffuse(0.7);
        &sphere1_raw.get_mut_material().set_specular(0.2);
        &sphere1_raw.get_mut_material().set_ambient(1.0);
        let sphere1 = Rc::new(sphere1_raw);
        let mut sphere2_raw = Sphere::new_raw(); 
        &sphere2_raw.transform(Matrix4x4::scaling(0.5, 0.5, 0.5));
        let sphere2 = Rc::new(sphere2_raw);
        &scene.add_object(sphere1); 
        &scene.add_object(sphere2); 
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let intersection = Intersection::new(1.0, scene.get_objects()[0].clone());
        let comps = Comp::compute_vars(intersection, &ray);
        let color = reflected_color(&scene, &comps, 5);
        assert_eq!(color, BLACK);
    }

    #[test]
    //Tests reflected color when the ray hits an object which is reflective
    fn reflective_intersection() {
        let mut scene = Scene::default();
        let mut shape_raw = Plane::new_raw();
        &shape_raw.transform(Matrix4x4::translation(0.0, -1.0, 0.0));
        &shape_raw.get_mut_material().set_reflectivity(0.5);
        let shape = Rc::new(shape_raw);
        scene.add_object(shape);
        let ray = Ray::new((0.0, 0.0, -3.0), (0.0, (-(2.0 as f32).sqrt()) / 2.0, ((2.0 as f32).sqrt()) / 2.0)); 
        let intersection = Intersection::new((2 as f32).sqrt(), Rc::clone(&scene.get_objects()[2]));
        let comps = Comp::compute_vars(intersection, &ray);
        let color = reflected_color(&scene, &comps, 5);
        assert_eq!(color.round(), Color::new(0.1905, 0.2381, 0.1429).round());
    }

    #[test]
    //Tests reflected color when the ray hits an object which is reflective
    fn reflection_shading() {
        let mut scene = Scene::default();
        let mut shape_raw = Plane::new_raw();
        &shape_raw.transform(Matrix4x4::translation(0.0, -1.0, 0.0));
        &shape_raw.get_mut_material().set_reflectivity(0.5);
        let shape = Rc::new(shape_raw);
        scene.add_object(shape);
        let ray = Ray::new((0.0, 0.0, -3.0), (0.0, (-(2.0 as f32).sqrt()) / 2.0, ((2.0 as f32).sqrt()) / 2.0)); 
        let intersection = Intersection::new((2 as f32).sqrt(), Rc::clone(&scene.get_objects()[2]));
        let comps = Comp::compute_vars(intersection, &ray);
        let color = Scene::scene_lighting(&scene, &comps, 5);
        assert_eq!(color.round(), Color::new(0.87687, 0.92453, 0.8293).round());
    }

    #[test]
    //Tests reflection limits on recursion
    fn recursion_limits() {
        let mut scene = Scene::default();
        let mut shape_raw = Plane::new_raw();
        &shape_raw.transform(Matrix4x4::translation(0.0, -1.0, 0.0));
        &shape_raw.get_mut_material().set_reflectivity(0.5);
        let shape = Rc::new(shape_raw);
        scene.add_object(shape);
        let ray = Ray::new((0.0, 0.0, -3.0), (0.0, (-(2.0 as f32).sqrt()) / 2.0, ((2.0 as f32).sqrt()) / 2.0)); 
        let intersection = Intersection::new((2 as f32).sqrt(), Rc::clone(&scene.get_objects()[2]));
        let comps = Comp::compute_vars(intersection, &ray);
        let color = reflected_color(&scene, &comps, 0);
        assert_eq!(color.round(), BLACK);
    }
}