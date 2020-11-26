#[cfg(test)]
mod tests {
    use rust_raytracer::canvas::*;
    use rust_raytracer::color::*;

    //Tests reading and writing to the canvas
    #[test]
    fn read_and_write() {
        let mut canvas = Canvas::new(10, 10);
        let green = Color::new(0.0, 1.0, 0.0);
        canvas.set(green.clone(), 5, 5);
        assert_eq!(canvas.get(5, 5), &green);
    }

    //Tests line length of generated ppm
    #[test]
    fn ppm_line_length() {
        let mut canvas = Canvas::new(20, 2);
        for y in 0..2 {
            for x in 0..20 {
                canvas.set(Color::new(1.0, 0.0, 0.0), x, y)
            }
        }
        let result = Canvas::format_ppm(canvas);
        let lines = result.lines();
        for line in lines {
            if line.len() > 70 {
                panic!("Line in ppm file is longer than 70");
            }
        }
    }

    //Tests if the formatted ppm ends with a newline
    #[test]
    fn ppm_ends_newline() {
        let canvas = Canvas::new(10, 10);
        let result = Canvas::format_ppm(canvas);
        assert_eq!(result.chars().last().unwrap(), '\n')
    }
}