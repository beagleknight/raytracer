use colors::Color;
use std::fmt::Write;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pixels: Vec<Color>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        canvas(width, height)
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        write_pixel(self, x, y, color);
    }

    // pub fn pixel_at(&self, x: usize, y: usize) -> Option<&Color> {
    //     pixel_at(self, x, y)
    // }

    pub fn color_at(&self, x: usize, y: usize) -> Color {
        match self.pixels.get(x + y * self.width as usize) {
            Some(color) => *color,
            None => Color::new(0.0, 0.0, 0.0),
        }
    }
}

pub fn canvas(width: i32, height: i32) -> Canvas {
    Canvas {
        width,
        height,
        pixels: vec![Color::new(0.0, 0.0, 0.0); (width * height) as usize],
    }
}

pub fn write_pixel(canvas: &mut Canvas, x: usize, y: usize, color: Color) {
    if let Some(color_in_canvas) = canvas.pixels.get_mut(x + y * canvas.width as usize) {
        *color_in_canvas = color;
    }
}

pub fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<&Color> {
    canvas.pixels.get(x + y * canvas.width as usize)
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value > max {
        return max;
    } else if value < min {
        return min;
    }

    value
}

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut result: String = format!("P3\n{} {}\n255\n", canvas.width, canvas.height);
    for y in 0..canvas.height {
        let mut row = vec![];
        for x in 0..canvas.width {
            let color = pixel_at(&canvas, x as usize, y as usize).unwrap();
            let red = (clamp(color.red, 0.0, 1.0) * 255.0).round();
            let green = (clamp(color.green, 0.0, 1.0) * 255.0).round();
            let blue = (clamp(color.blue, 0.0, 1.0) * 255.0).round();
            row.push(format!("{}", red));
            row.push(format!("{}", green));
            row.push(format!("{}", blue));
        }
        let mut row_as_string = String::from("");
        for color_component in row {
            if row_as_string.len() + color_component.len() + 1 > 70 {
                writeln!(&mut result, "{}", row_as_string).unwrap();
                row_as_string = String::from("");
            }
            let mut row_as_vec = row_as_string
                .split(" ")
                .filter(|&str| str != "")
                .collect::<Vec<_>>();
            row_as_vec.push(&color_component);
            row_as_string = row_as_vec.join(" ");
        }
        writeln!(&mut result, "{}", row_as_string).unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn creating_a_canvas() {
        let c = canvas(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.pixels.len(), 200);
        for pixel in c.pixels {
            assert_eq!(pixel, Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = canvas(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        write_pixel(&mut c, 2, 3, red);
        assert_eq!(pixel_at(&c, 2, 3).unwrap(), &red);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = canvas(5, 3);
        let ppm = canvas_to_ppm(&c);
        assert_eq!(
            ppm.split_terminator("\n")
                .take(3)
                .collect::<Vec<&str>>()
                .join("\n"),
            r#"P3
5 3
255"#
        )
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        write_pixel(&mut c, 0, 0, c1);
        write_pixel(&mut c, 2, 1, c2);
        write_pixel(&mut c, 4, 2, c3);
        let ppm = canvas_to_ppm(&c);
        assert_eq!(
            ppm.split_terminator("\n")
                .skip(3)
                .take(3)
                .collect::<Vec<&str>>()
                .join("\n"),
            r#"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"#
        )
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = canvas(10, 2);
        for pixel in c.pixels.iter_mut() {
            *pixel = Color::new(1.0, 0.8, 0.6);
        }
        let ppm = canvas_to_ppm(&c);
        assert_eq!(
            ppm.split_terminator("\n")
                .skip(3)
                .take(4)
                .collect::<Vec<&str>>()
                .join("\n"),
            r#"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153"#
        )
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = canvas(5, 3);
        let ppm = canvas_to_ppm(&c);
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}
