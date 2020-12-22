#[cfg(test)]

mod tests {
    use rust_ray_tracer::objects::cone::Cone;
    use rust_ray_tracer::objects::object::*;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::materials::material::*;
    use rust_ray_tracer::ray_tracing::ray::Ray;

    #[test]
    //Tests intersecting rays with a cone
    fn rays_hit_cone() {
        let cone = Cone::default();

        assert_eq!(cone.intersect(&Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0))).unwrap()[0].t, 5.0);
        assert_eq!(cone.intersect(&Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0))).unwrap()[1].t, 5.0);
    }
}