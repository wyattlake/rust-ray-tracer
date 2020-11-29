#[cfg(test)]

mod tests {
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::core::color::Color;
    use rust_ray_tracer::ray_tracing::lighting::*;
    use rust_ray_tracer::ray_tracing::scene::Scene;
    use rust_ray_tracer::ray_tracing::material::Material;


    //Tests lighting when directly facing at a sphere
    #[test]
    fn sphere_front() {
        let m = Material::default();
        let position = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let e_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let n_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(0.0, 0.0, -10.0, 1.0));
        let result = lighting(&m, &light, &position, &e_vec, &n_vec, false);
        assert_eq!(result.round(), Color::new(1.9, 1.9, 1.9).round());
    }

    //Tests lighting when above a sphere at a 45 degree angle
    #[test]
    fn sphere_above_45() {
        let m = Material::default();
        let position = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let e_vec = Vec4::new(0.0, ((2.0 as f64).sqrt())/2.0, -((2.0 as f64).sqrt())/2.0, 0.0);
        let n_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(0.0, 0.0, -10.0, 1.0));
        let result = lighting(&m, &light, &position, &e_vec, &n_vec, false);
        assert_eq!(result.round(), Color::new(1.0, 1.0, 1.0).round());
    }

    //Tests lighting when directly facing a sphere with a light 45 degrees above
    #[test]
    fn sphere_front_light_45() {
        let m = Material::default();
        let position = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let e_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let n_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(0.0, 10.0, -10.0, 1.0));
        let result = lighting(&m, &light, &position, &e_vec, &n_vec, false);
        assert_eq!(result.round(), Color::new(0.7364, 0.7364, 0.7364).round());
    }

    //Tests lighting when facing a sphere from below at a 45 degree angle with a light above at a 45 degrees above
    #[test]
    fn sphere_45_light_45() {
        let m = Material::default();
        let position = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let e_vec = Vec4::new(0.0, -((2.0 as f64).sqrt())/2.0, -((2.0 as f64).sqrt())/2.0, 0.0);
        let n_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(0.0, 10.0, -10.0, 1.0));
        let result = lighting(&m, &light, &position, &e_vec, &n_vec, false);
        assert_eq!(result.round(), Color::new(1.6364, 1.6364, 1.6364).round());
    }

    //Tests lighting when the light source is obstructed
    #[test]
    fn sphere_obstructed() {
        let m = Material::default();
        let position = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let e_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let n_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(0.0, 10.0, 10.0, 1.0));
        let result = lighting(&m, &light, &position, &e_vec, &n_vec, false);
        assert_eq!(result.round(), Color::new(0.1, 0.1, 0.1).round());
    }

    //Tests shadows when sphere does not block the light source from the point
    #[test]
    fn point_not_blocked() {
        let scene = Scene::default();
        let light = &scene.get_light_sources()[0];
        let target = Vec4::new(0.0, 10.0, 0.0, 1.0);
        assert_eq!(in_shadow(&light, &target, &scene), false);
    }

    //Tests shadows when sphere blocks point from light source
    #[test]
    fn shadow_blocked_sphere() {
        let scene = Scene::default();
        let light = &scene.get_light_sources()[0];
        let target = Vec4::new(10.0, -10.0, 10.0, 1.0);
        assert_eq!(in_shadow(&light, &target, &scene), true);
    }

    //Tests shadows when the point is in front of the light source
    #[test]
    fn point_past_light() {
        let scene = Scene::default();
        let light = &scene.get_light_sources()[0];
        let target = Vec4::new(-20.0, 20.0, -20.0, 1.0);
        assert_eq!(in_shadow(&light, &target, &scene), false);
    }
}