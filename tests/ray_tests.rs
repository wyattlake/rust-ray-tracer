#[cfg(test)]

mod tests {
    use rust_ray_tracer::ray::*;
    use rust_ray_tracer::vector::*;
    use rust_ray_tracer::sphere::*;
    use rust_ray_tracer::matrix::*;
    use std::rc::Rc;

    //Tests the ray position function
    #[test]
    fn ray_position() {
        let ray = Ray::new((2.0, 3.0, 4.0), (1.0, 0.0, 0.0));
        assert_eq!(Ray::position(&ray, &1.0), Vec4::new(3.0, 3.0, 4.0, 1.0))
    }

    //Tests ray intersection with 2 points on unit sphere
    #[test]
    fn ray_intersect_sphere() {
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let result = Ray::intersect(sphere, &ray);
        assert_eq!(result.unwrap().len(), 2);
    }

    //Tests ray intersection with 1 point on unit sphere
    #[test]
    fn ray_tangent_to_sphere() {
       let ray = Ray::new((0.0, 1.0, -5.0), (0.0, 0.0, 1.0));
       let sphere = Sphere::new();
       let result = Ray::intersect(sphere, &ray);
       assert_eq!(result.unwrap().len(), 2);
    }

    //Tests ray missing unit sphere
    #[test]
    fn ray_avoids_sphere() {
        let ray = Ray::new((0.0, 2.0, -5.0), (0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let result = Ray::intersect(sphere, &ray);
        assert_eq!(result, None);
    }

    //Tests ray intersection from inside the unit sphere
    #[test]
    fn ray_inside_sphere() {
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let result = Ray::intersect(sphere, &ray);
        assert_eq!(result.unwrap().len(), 2);
    }

    //Tests ray intersection from in front of the unit sphere
    #[test]
    fn ray_in_front_of_sphere() {
       let ray = Ray::new((0.0, 1.0, 5.0), (0.0, 0.0, 1.0));
       let sphere = Sphere::new();
       let result = Ray::intersect(sphere, &ray);
       assert_eq!(result.unwrap().len(), 2);
    }

    //Tests ray translation
    #[test]
    fn ray_translation() {
        let ray = Ray::new((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));
        let matrix = Matrix4x4::translation(3.0, 4.0, 5.0);
        let new_ray = Ray::transform(&ray, &matrix);
        assert_eq!(new_ray.origin, Vec4::new(4.0, 6.0, 8.0, 1.0));
    }

    //Tests ray scaling
    #[test]
    fn ray_scaling() {
        let ray = Ray::new((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));
        let matrix = Matrix4x4::scaling(2.0, 3.0, 4.0);
        let new_ray = Ray::transform(&ray, &matrix);
        assert_eq!(&new_ray.origin, &Vec4::new(2.0, 6.0, 12.0, 1.0));
        assert_eq!(&new_ray.direction, &Vec4::new(0.0, 3.0, 0.0, 0.0));
    }

    //Tests ray intersection with a scaled sphere
    #[test]
    fn ray_intersect_scaled_sphere() {
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let mut sphere = Sphere::new_raw();
        sphere.transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
        let sphere_ref = Rc::new(sphere);
        let result = Ray::intersect(sphere_ref, &ray).unwrap();
        assert_eq!(result.clone().len(), (2 as usize));
        assert_eq!(result.clone()[0].get_t(), 3.0);
        assert_eq!(result.clone()[1].get_t(), 7.0);
    }

    //Tests ray intersection with a translated sphere
    #[test]
    fn ray_intersect_translated_sphere() {
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let mut sphere = Sphere::new_raw();
        sphere.transform(Matrix4x4::translation(5.0, 0.0, 0.0));
        let sphere_ref = Rc::new(sphere);
        let result = Ray::intersect(sphere_ref, &ray);
        assert_eq!(result, None);
    }

    #[test]
    //Tests surface normals on the x axis
    fn surface_normal_x() {
        let s = Sphere::new();
        let vector = Vec4::normal(&s, &Vec4::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(vector, Vec4::new(1.0, 0.0, 0.0, 0.0))
    }
}