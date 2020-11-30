#[cfg(test)]

mod tests {
    use rust_ray_tracer::ray_tracing::patterns::*;
    use rust_ray_tracer::ray_tracing::material::Material;
    use rust_ray_tracer::core::color::*;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::objects::sphere::Sphere;
    use rust_ray_tracer::objects::general::*;
    use rust_ray_tracer::ray_tracing::lighting::*;
    use std::rc::Rc;

    //Tests if the  stripe pattern is constant for z
    #[test]
    fn stripe_pattern_z() {
        let pattern = StripePattern::new(WHITE.clone(), BLACK.clone());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 0.0, 1.0, 1.0)));
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 1.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 0.0, 2.0, 1.0)));
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 2.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 0.0, 100.0, 1.0)));
    }

    //Tests if the stripe pattern is constant for y
    #[test]
    fn stripe_pattern_y() {
        let pattern = StripePattern::new(WHITE.clone(), BLACK.clone());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 1.0, 0.0, 1.0)));
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 1.0, 0.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 2.0, 0.0, 1.0)));
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 2.0, 0.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 100.0, 0.0, 1.0)));
    }

    //Tests if the stripe pattern alternates for x
    #[test]
    fn stripe_pattern_x() {
        let pattern = StripePattern::new(WHITE.clone(), BLACK.clone());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(1.0, 0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern.color_at(&Vec4::new(2.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(3.0, 0.0, 0.0, 1.0)), BLACK);
    }

    //Tests the stripe pattern with the lighting function
    #[test]
    fn stripe_pattern_lighting() {
        let object = Sphere::new();
        let mut material = Material::default();
        let pattern = StripePattern::new(WHITE, BLACK);
        &material.set_ambient(1.0);
        &material.set_diffuse(0.0);
        &material.set_specular(0.0);
        &material.set_pattern(pattern);
        let e_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let n_vec = Vec4::new(0.0, 0.0, -1.0, 0.0);
        let light = PointLight::new(WHITE, Vec4::new(0.0, 0.0, -10.0, 1.0));
        let color1 = lighting(&material, &object, &light, &Vec4::new(0.9, 0.0, 0.0, 1.0), &e_vec, &n_vec, false);
        let color2 = lighting(&material, &object, &light, &Vec4::new(1.1, 0.0, 0.0, 1.0), &e_vec, &n_vec, false);
        assert_eq!(color1, WHITE);
        assert_eq!(color2, BLACK);
    }

    //Tests color on a transformed object
    #[test]
    fn color_object_transformation() {
        let pattern = StripePattern::new(WHITE, BLACK);
        let mut s = Sphere::new_raw();
        &s.transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
        let sphere = Rc::new(s);
        let color = pattern.color_at_object(&sphere, &Vec4::new(1.5, 0.0, 0.0, 1.0));
        assert_eq!(color, WHITE);
    }

    #[test]
    //Tests color on a transformed pattern
    fn color_pattern_transformation() {
        let mut pattern = StripePattern::new(WHITE, BLACK);
        &pattern.transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
        let sphere = Sphere::new();
        let color = pattern.color_at_object(&sphere, &Vec4::new(1.5, 0.0, 0.0, 1.0));
        assert_eq!(color, WHITE);
    }

    #[test]
    //Tests color on a transformed pattern on a transformed sphere
    fn color_mixed_tranformation_pattern() {
        let mut pattern = StripePattern::new(WHITE, BLACK);
        &pattern.transform(Matrix4x4::translation(0.5, 0.0, 0.0));
        let mut s = Sphere::new_raw();
        &s.transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
        let sphere = Rc::new(s);
        let color = pattern.color_at_object(&sphere, &Vec4::new(2.5, 0.0, 0.0, 1.0));
        assert_eq!(color, WHITE);
    }

    #[test]
    //Tests the gradient pattern
    fn gradient_pattern() {
        let pattern = GradientPattern::new(WHITE, BLACK);
        let sphere = Sphere::new();
        assert_eq!(pattern.color_at_object(&sphere, &Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at_object(&sphere, &Vec4::new(0.25, 0.0, 0.0, 1.0)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.color_at_object(&sphere, &Vec4::new(0.5, 0.0, 0.0, 1.0)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.color_at_object(&sphere, &Vec4::new(0.75, 0.0, 0.0, 1.0)), Color::new(0.25, 0.25, 0.25));
    }

    #[test]
    //Tests the ring pattern
    fn ring_pattern() {
        let pattern = RingPattern::new(WHITE, BLACK);
        let sphere = Sphere::new();
        assert_eq!(pattern.color_at_object(&sphere, &Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at_object(&sphere, &Vec4::new(1.0, 0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern.color_at_object(&sphere, &Vec4::new(0.0, 0.0, 1.0, 1.0)), BLACK);
    }

    #[test]
    //Tests the checkerboard pattern repeating on the x axis
    fn checkerboard_pattern_x() {
        let pattern = CheckerboardPattern::new(WHITE, BLACK);
        let sphere = Sphere::new();
        assert_eq!(pattern.color_at_object(&sphere, &Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at_object(&sphere, &Vec4::new(0.99, 0.0, 0.0, 1.0)), WHITE);
    }

    #[test]
    //Tests the checkerboard pattern repeating on the y axis
    fn checkerboard_pattern_y() {
        let pattern = CheckerboardPattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.99, 0.0, 1.0)), WHITE);
    }

    #[test]
    //Tests the checkerboard pattern repeating on the y axis
    fn checkerboard_pattern_z() {
        let pattern = CheckerboardPattern::new(WHITE, BLACK);
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.99, 1.0)), WHITE);
    }
}