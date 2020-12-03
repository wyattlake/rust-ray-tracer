#[cfg(test)]

mod tests {
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::core::color::*;
    use rust_ray_tracer::ray_tracing::lighting::*;
    use rust_ray_tracer::ray_tracing::scene::Scene;

    //Tests shadows when sphere does not block the light source from the point
    #[test]
    fn point_not_blocked() {
        let scene = Scene::default();
        let light = &scene.light_sources[0];
        let target = Vec4::new(0.0, 10.0, 0.0, 1.0);
        assert_eq!(in_shadow(&light.get_position(), &target, &scene), false);
    }

    //Tests shadows when sphere blocks point from light source
    #[test]
    fn shadow_blocked_sphere() {
        let scene = Scene::default();
        let light = &scene.light_sources[0];
        let target = Vec4::new(10.0, -10.0, 10.0, 1.0);
        assert_eq!(in_shadow(&light.get_position(), &target, &scene), true);
    }

    //Tests shadows when the point is in front of the light source
    #[test]
    fn point_past_light() {
        let scene = Scene::default();
        let light = &scene.light_sources[0];
        let target = Vec4::new(-20.0, 20.0, -20.0, 1.0);
        assert_eq!(in_shadow(&light.get_position(), &target, &scene), false);
    }

    //Tests creating a new AreaLight
    #[test]
    fn create_area_light() {
        let corner = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let v1 = Vec4::new(2.0, 0.0, 0.0, 0.0);
        let v2 = Vec4::new(0.0, 0.0, 1.0, 0.0);
        let light = AreaLight::new(corner.clone(), v1, 4, v2.clone(), 2, WHITE);
        assert_eq!(light.corner, corner);
        assert_eq!(light.uvec, Vec4::new(0.5, 0.0, 0.0, 0.0));
        assert_eq!(light.vvec, Vec4::new(0.0, 0.0, 0.5, 0.0));
        assert_eq!(light.vsteps, 2);
        assert_eq!(light.usteps, 4);
        assert_eq!(light.samples, (8 as usize));
        assert_eq!(light.get_intensity(), &Color::new(1.0, 1.0, 1.0));
    }
}