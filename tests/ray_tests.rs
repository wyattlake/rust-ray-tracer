#[cfg(test)]

mod tests {
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::ray_tracing::ray::Ray;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::objects::sphere::Sphere;
    use rust_ray_tracer::materials::material::Material;
    use rust_ray_tracer::objects::object::*;

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
        assert_eq!(new_ray.origin, Vec4::new(2.0, 6.0, 12.0, 1.0));
        assert_eq!(new_ray.direction, Vec4::new(0.0, 3.0, 0.0, 0.0));
    }

    #[test]
    //Tests surface normals on the x axis
    fn surface_normal_x() {
        let s = Sphere::new(Matrix4x4::identity(), Material::default());
        let vector = Object::normal(&s, &Vec4::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(vector, Vec4::new(1.0, 0.0, 0.0, 0.0))
    }
}