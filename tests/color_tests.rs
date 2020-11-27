#[cfg(test)]
mod tests {
    use rust_ray_tracer::color::*;

    //Tests color addition
    #[test]
    fn addition() {
        let clr1 = Color::new(0.9, 0.6, 0.75);
        let clr2 = Color::new(0.7, 0.1, 0.25);
        let clr3 = clr1 + clr2;
        assert_eq!(&Color::new(1.6, 0.7, 1.0), &clr3);
    }

    //Tests color subtraction 
    #[test]
    fn subtraction() {
        let clr1 = Color::new(0.8, 0.6, 0.75);
        let clr2 = Color::new(0.3, 0.1, 0.25);
        let clr3 = clr1 - clr2;
        assert_eq!(Color::new(0.5, 0.5, 0.5), clr3);
    }

    //Tests scalar multiplication 
    #[test]
    fn scalar_mult() {
        let clr1 = Color::new(0.2, 0.3, 0.4);
        let clr2 = clr1 * 2.0;
        let clr3 = 0.5 * clr2;
        assert_eq!(&Color::new(0.2, 0.3, 0.4), &clr3);
    }

    //Tests cross products 
    #[test]
    fn mult() {
        let clr1 = Color::new(1.0, 0.2, 1.0);
        let clr2 = Color::new(0.9, 1.0, 0.3);
        let clr_mult = &clr1 * &clr2;
        assert_eq!(Color::new(0.9, 0.2, 0.3), clr_mult);
    }

    //Tests color ppm conversion
    #[test]
    fn convert_to_ppm_string() {
        let clr = Color::new(1.0, 0.5, 0.0);
        assert_eq!(clr.ppm_string(), String::from("255 128 0"));
    }
}

