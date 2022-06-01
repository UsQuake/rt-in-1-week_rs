use std::fmt;
use crate::color::*;

    pub struct PpmFormat{
        pub buffer : Vec<Vec<Color>>,
        pub width: u16,
        pub height: u16,
        pub color_max: u8
    }
    
    impl PpmFormat
    {
        pub fn new(width: u16, height: u16, color_max: u8) -> Self
        {
            let mut return_form = PpmFormat{
                width: width,
                height: height,
                color_max: color_max,
                buffer: Vec::new()
            };
            return_form.buffer.reserve(height as usize);
            
            for _ in (0..height).rev()
            {
                let mut row:Vec<Color> = Vec::with_capacity(width as usize);
                for _ in 0..width{
                    row.push(Color::new(0,0,0));
                }
                return_form.buffer.push(row);
            }
            return_form
        }

        pub fn write_color(&mut self, pixel_color: Color, pixel_x: usize, pixel_y: usize)
        {
           self.buffer[pixel_y][pixel_x] = pixel_color;
        }
        pub fn write_color_rows(&mut self, column_color_vectors:  Vec<Vec<Color>>,  row_offset: usize, row_end: usize)
        {
            for y in row_offset..row_end
            {
                for x in 0..self.width
                {
                    self.buffer[y as usize][x as usize] = column_color_vectors[(y - row_offset) as usize][x  as usize];
                }
            }

        }
    }
     impl fmt::Display for PpmFormat{
        
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut formatting_buffer = String::from(format!("P3\n{} {}\n{}\n", self.width, self.height, self.color_max));
            
            for y in self.buffer.iter().rev()
            {
                for x in y
                {
                    formatting_buffer.push_str(&format!("{}",x));
                }
            }
            return write!(f, "{}", formatting_buffer);
        }
    }

