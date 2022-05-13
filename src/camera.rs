use crate::vec3::*;
use crate::ray::*;
use crate::utility::*;

pub struct Camera{
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, v: Vec3, w: Vec3,
    lens_radius: f64
}

impl Camera{
    pub fn new(
        lookfrom : Vec3,
        lookat : Vec3,
        vup : Vec3,
        vfov: f64, 
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64) ->Self{

        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);

        let mut return_camera = Camera
        {
            origin : lookfrom,
            horizontal : focus_dist * viewport_width * u,
            vertical : focus_dist * viewport_height * v,
            lower_left_corner : Vec3::new(0.0,0.0,0.0),
            w:w,
            u:u,
            v:v,
            lens_radius : aperture / 2.0
        };
        return_camera.lower_left_corner =  return_camera.origin - return_camera.horizontal/2.0 - return_camera.vertical/2.0 - focus_dist * w;
        return_camera
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray
    {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y(); 

        Ray::new(self.origin + offset,
             self.lower_left_corner + s * self.horizontal + t * self.vertical 
             - self.origin - offset)
    }
}
impl Copy for Camera { }
impl std::clone::Clone for Camera
{
    fn clone(&self) -> Self {
        *self
     }
}