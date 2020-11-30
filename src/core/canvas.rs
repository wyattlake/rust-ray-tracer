use crate::core::color::Color;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//Canvas stores the color for each pixel
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub contents: Vec<Color>,
}

impl Canvas {
    //Creates a new canvas
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            contents: vec![Color(0.0, 0.0, 0.0); width * height],
        }
    }

    //Gets a reference to a Color in the Canvas
    pub fn get(&self, x: i32, y: i32) -> Option<&Color> {
        if self.in_bounds(x, y) {
            let index = self.width * (y as usize) + (x as usize) % self.width;
            Some(&self.contents[index])
        }
        else {
            None
        }
    }

    //Sets the color of a pixel in the Canvas
    pub fn set(&mut self, color: Color, x: i32, y: i32) {
        if self.in_bounds(x, y) {
            let index = self.width * (y as usize) + (x as usize) % self.width;
            self.contents[index] = color;
        }
    }

    //Checks if given coordinates are in the canvas bounds
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        if x > (self.width - 1) as i32 || y > (self.height - 1) as i32 || x < 0 || y < 0 {
            false
        }
        else {
            true
        }
    }

    //Formats the canvas contents
    pub fn format_ppm(canvas: Canvas) -> String {
        let mut current_line_length = 0;
        let mut current_item = 0;
        let slice = &canvas.contents[..];
        let mut result = format!("P3\n{} {}\n255\n", &canvas.width, &canvas.height);
        for color in slice {
            if &current_line_length + color.ppm_length() > 70 || &current_item >= &canvas.width {
                result.push_str("\n");
                current_item = 0; 
                current_line_length = 0;
            }
            result.push_str(&*format!("{} ", color.ppm_string()));
            current_item += 1;
            current_line_length += color.ppm_length();
        };
        result.push_str("\n");
        result
    }

    //Formats the canvas contents
    pub fn format_txt(canvas: Canvas) -> String {
        let mut current_item = 0;
        let slice = &canvas.contents[..];
        let mut result = String::from("");
        for color in slice {
            if &current_item >= &canvas.width {
                result.push_str("\n");
                current_item = 0; 
            }
            result.push_str(&*format!("{}", color.txt_string()));
            current_item += 1;
        };
        result.push_str("\n");
        result
    }

    //Write the canvas to a ppm file
    pub fn write_file(canvas: Canvas, filename: &str) {
        let filename_formatted = &*format!("{}.ppm", filename);
        let path = Path::new(filename_formatted);
        let display = path.display();
        let text = Canvas::format_ppm(canvas);
        let mut file = match File::create(&path) {
            Err(error) => panic!("Failed to open {}: {}", display, error),
            Ok(file) => file,
        };
        match file.write_all(text.as_bytes()) {
            Err(error) => panic!("Failed to write to {}: {}", display, error),
            Ok(_) => println!("Wrote canvas to {}", display),
        }
    }

    //Write the canvas to a txt file
    pub fn write_file_txt(canvas: Canvas, filename: &str) {
        let filename_formatted = &*format!("{}.txt", filename);
        let path = Path::new(filename_formatted);
        let display = path.display();
        let text = Canvas::format_txt(canvas);
        let mut file = match File::create(&path) {
            Err(error) => panic!("Failed to open {}: {}", display, error),
            Ok(file) => file,
        };
        match file.write_all(text.as_bytes()) {
            Err(error) => panic!("Failed to write to {}: {}", display, error),
            Ok(_) => println!("Wrote canvas to {}", display),
        }
    }
}
