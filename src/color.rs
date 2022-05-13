use std::fmt;
use crate::vec3::*;
use crate::utility::*;

type ColorF = Vec3;
pub fn fcolor_to_icolor(float_color: ColorF, samples_per_pixel: u16) -> Color
{
    let scale = 1.0 / (samples_per_pixel as f64);

    let r  = (float_color.x() * scale).sqrt(); 
    let g  = (float_color.y() * scale).sqrt(); 
    let b  = (float_color.z() * scale).sqrt(); 
    Color{
        r: (clamp(r,0.0, 0.999) * 256.0) as u16,
        g: (clamp(g,0.0, 0.999) * 256.0) as u16,
        b: (clamp(b,0.0, 0.999) * 256.0) as u16,
    }
}   
pub struct Color {
        pub r : u16,
        pub g : u16,
        pub b : u16
}
impl Copy for Color{ }
impl std::clone::Clone for Color
{
    fn clone(&self) -> Self {
        *self
     }
}

impl Color
    {
        pub fn new (r:u16, g:u16, b:u16) -> Self
        {
            Color{
                r:r,
                g:g,
                b:b
            }
        }
        
    }
    impl fmt::Display for Color {
        
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {} {}\n", self.r, self.g, self.b)
        }
    }