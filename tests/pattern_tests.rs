#[cfg(test)]

mod tests {
    use rust_ray_tracer::materials::patterns::*;
    use rust_ray_tracer::core::color::*;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    
    //Tests if the  stripe pattern is constant for z
    #[test]
    fn stripe_pattern_z() {
        let pattern = StripePattern::new(WHITE.clone(), BLACK.clone(), Matrix4x4::identity());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 0.0, 1.0, 1.0)));
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 1.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 0.0, 2.0, 1.0)));
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 2.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 0.0, 100.0, 1.0)));
    }

    //Tests if the stripe pattern is constant for y
    #[test]
    fn stripe_pattern_y() {
        let pattern = StripePattern::new(WHITE.clone(), BLACK.clone(), Matrix4x4::identity());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 1.0, 0.0, 1.0)));
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 1.0, 0.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 2.0, 0.0, 1.0)));
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 2.0, 0.0, 1.0)), pattern.color_at(&Vec4::new(0.0, 100.0, 0.0, 1.0)));
    }

    //Tests if the stripe pattern alternates for x
    #[test]
    fn stripe_pattern_x() {
        let pattern = StripePattern::new(WHITE.clone(), BLACK.clone(), Matrix4x4::identity());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(1.0, 0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern.color_at(&Vec4::new(2.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(3.0, 0.0, 0.0, 1.0)), BLACK);
    }

    #[test]
    //Tests the gradient pattern
    fn gradient_pattern() {
        let pattern = GradientPattern::new(WHITE, BLACK, Matrix4x4::identity());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(0.25, 0.0, 0.0, 1.0)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.color_at(&Vec4::new(0.5, 0.0, 0.0, 1.0)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.color_at(&Vec4::new(0.75, 0.0, 0.0, 1.0)), Color::new(0.25, 0.25, 0.25));
    }

    #[test]
    //Tests the ring pattern
    fn ring_pattern() {
        let pattern = RingPattern::new(WHITE, BLACK, Matrix4x4::identity());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(1.0, 0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 1.0, 1.0)), BLACK);
    }

    #[test]
    //Tests the checkerboard pattern repeating on the x axis
    fn checkerboard_pattern_x() {
        let pattern = CheckerboardPattern::new(WHITE, BLACK, Matrix4x4::identity());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(0.99, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(1.01, 0.0, 0.0, 1.0)), BLACK);
    }

    #[test]
    //Tests the checkerboard pattern repeating on the y axis
    fn checkerboard_pattern_y() {
        let pattern = CheckerboardPattern::new(WHITE, BLACK, Matrix4x4::identity());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.99, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 1.01, 0.0, 1.0)), BLACK);
    }

    #[test]
    //Tests the checkerboard pattern repeating on the y axis
    fn checkerboard_pattern_z() {
        let pattern = CheckerboardPattern::new(WHITE, BLACK, Matrix4x4::identity());
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 0.99, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Vec4::new(0.0, 0.0, 1.01, 1.0)), BLACK);
    }
}