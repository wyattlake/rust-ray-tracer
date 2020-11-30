#[cfg(test)]

mod tests {
    use rust_ray_tracer::ray_tracing::intersection::Intersection;
    use rust_ray_tracer::core::color::Color;
    use rust_ray_tracer::ray_tracing::ray::Ray;
    use rust_ray_tracer::objects::sphere::Sphere;
    use rust_ray_tracer::ray_tracing::scene::Scene;
    use rust_ray_tracer::ray_tracing::lighting::*;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::core::comp::Comp;
    use rust_ray_tracer::objects::general::ObjectMethods;
    use std::rc::Rc;
    
    //Tests creation of a default scene
    #[test]
    fn default_scene() {
        let scene = Scene::default();
        assert_eq!(scene.get_light_sources()[0], PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -10.0, 1.0)));
        let objects = scene.get_objects();
        let mut sphere1_raw = Sphere::new_raw(); 
        &sphere1_raw.get_mut_material().set_color(Color::new(0.8, 1.0, 0.6));
        &sphere1_raw.get_mut_material().set_diffuse(0.7);
        &sphere1_raw.get_mut_material().set_specular(0.2);
        let sphere1 = Rc::new(sphere1_raw);
        assert_eq!(&objects[0], &sphere1);
        let mut sphere2_raw = Sphere::new_raw(); 
        &sphere2_raw.transform(Matrix4x4::scaling(0.5, 0.5, 0.5));
        let sphere2 = Rc::new(sphere2_raw);
        assert_eq!(&objects[1], &sphere2);
    }

    //Tests intersections within a scene
    #[test]
    fn scene_intersections() {
        let scene = Scene::default();
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let intersections = Ray::intersect_scene(&scene, &ray);
        assert_eq!(intersections.len(), 4);
        assert_eq!(&intersections[0].get_t(), &4.0);
        assert_eq!(&intersections[1].get_t(), &4.5);
        assert_eq!(&intersections[2].get_t(), &5.5);
        assert_eq!(&intersections[3].get_t(), &6.0);
    }

    //Tests pre computing values
    #[test]
    fn pre_compute_values() {
       let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0)); 
       let shape = Sphere::new();
       let intersection = Intersection::new(4.0, shape);
       let vars = Comp::compute_vars(intersection, &ray);
       assert_eq!(&vars.t, &4.0);
       assert_eq!(&vars.object, &Sphere::new());
       assert_eq!(&vars.point, &Vec4::new(0.0, 0.0, -1.0, 1.0));
       assert_eq!(&vars.e_vec, &Vec4::new(0.0, 0.0, -1.0, 0.0));
       assert_eq!(&vars.n_vec, &Vec4::new(0.0, 0.0, -1.0, 0.0));
    }

    //Tests intersection outside sphere
    #[test]
    fn intersection_outside() {
       let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0)); 
       let shape = Sphere::new();
       let intersection = Intersection::new(4.0, shape);
       let vars = Comp::compute_vars(intersection, &ray);
       assert_eq!(&vars.inside, &false);
    }

    //Tests intersection inside sphere
    #[test]
    fn intersection_inside() {
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0)); 
        let shape = Sphere::new();
        let intersection = Intersection::new(1.0, shape);
        let vars = Comp::compute_vars(intersection, &ray);
        assert_eq!(&vars.inside, &true);
        assert_eq!(&vars.point, &Vec4::new(0.0, 0.0, 1.0, 1.0));
        assert_eq!(&vars.e_vec, &Vec4::new(0.0, 0.0, -1.0, 0.0));
        assert_eq!(&vars.n_vec, &Vec4::new(0.0, 0.0, -1.0, 0.0));
    }

    #[test]
    //Tests lighting an intersection
    fn scene_lighting_intersection() {
        let scene = Scene::default();
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let sphere = Rc::clone(&scene.get_objects()[0]);
        let intersection = Intersection::new(4.0, sphere);
        let comps = Comp::compute_vars(intersection, &ray);
        let color = Scene::scene_lighting(&scene, comps, 5);
        assert_eq!(color.round(), Color::new(0.38063, 0.47583, 0.2855).round());
    }

    #[test]
    //Tests lighting an intersection from the inside
    fn scene_lighting_intersection_inside() {
        let mut scene = Scene::default();
        &scene.clear_lights();
        &scene.add_light(PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(0.0, 0.25, 0.0, 1.0)));
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let sphere = Rc::clone(&scene.get_objects()[1]);
        let intersection = Intersection::new(0.5, sphere);
        let comps = Comp::compute_vars(intersection, &ray);
        let color = Scene::scene_lighting(&scene, comps, 5);
        assert_eq!(color.round(), Color::new(0.1, 0.1, 0.1).round());
    }

    //Tests color when a ray misses
    #[test]
    fn ray_misses() {
        let scene = Scene::default();
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 1.0, 0.0));
        let color = Scene::compute_color(ray, &scene, 5);
        assert_eq!(color, None);
    }

    //Tests color when a ray hits
    #[test]
    fn ray_hits() {
        let scene = Scene::default();
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let color = Scene::compute_color(ray, &scene, 5);
        assert_eq!(color.unwrap().round(), Color::new(0.3806, 0.47583, 0.2855).round());
    }

    //Tests color when a ray hits from inside a sphere
    #[test]
    fn ray_inside_hits() {
        let mut scene = Scene::new();
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -10.0, 1.0));
        &scene.add_light(light);
        let mut sphere1_raw = Sphere::new_raw(); 
        &sphere1_raw.get_mut_material().set_color(Color::new(0.8, 1.0, 0.6));
        &sphere1_raw.get_mut_material().set_diffuse(0.7);
        &sphere1_raw.get_mut_material().set_ambient(1.0);
        &sphere1_raw.get_mut_material().set_specular(0.2);
        let sphere1 = Rc::new(sphere1_raw);
        let mut sphere2_raw = Sphere::new_raw(); 
        &sphere2_raw.get_mut_material().set_ambient(1.0);
        &sphere2_raw.transform(Matrix4x4::scaling(0.5, 0.5, 0.5));
        let inner_material = sphere2_raw.get_material().clone();
        let sphere2 = Rc::new(sphere2_raw);
        &scene.add_object(sphere1); 
        &scene.add_object(sphere2); 
        let ray = Ray::new((0.0, 0.0, 0.75), (0.0, 0.0, -1.0));
        let color = Scene::compute_color(ray, &scene, 5);
        assert_eq!(color.unwrap().round(), inner_material.get_color().round());
    }

    #[test]
    //Tests lighting of an intersection in shadow
    fn intersection_in_shadow() {
        let mut scene = Scene::new();

        &scene.clear_lights();
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(0.0, 0.0, -10.0, 1.0));
        &scene.add_light(light);

        let sphere1 = Sphere::new();
        &scene.add_object(sphere1);

        let mut sphere2_raw = Sphere::new_raw(); 
        &sphere2_raw.transform(Matrix4x4::translation(0.0, 0.0, 10.0));
        let sphere2 = Rc::new(sphere2_raw);
        &scene.add_object(sphere2);

        let ray = Ray::new((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));
        let intersection = Intersection::new(4.0, Rc::clone(&scene.get_objects()[1]));

        let comps = Comp::compute_vars(intersection, &ray);
        let color = Scene::scene_lighting(&scene, comps, 5);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }

    //Tests intersections offsetting points
    #[test]

    fn point_offset() {
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));

        let mut sphere_raw = Sphere::new_raw();
        &sphere_raw.transform(Matrix4x4::translation(0.0, 0.0, 1.0));
        let sphere = Rc::new(sphere_raw);

        let intersection = Intersection::new(5.0, sphere);
        let _comps = Comp::compute_vars(intersection, &ray);
    }
}