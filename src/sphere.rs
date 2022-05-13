use crate::hitable::*;
use crate::vec3::*;
use crate::ray::*;
use crate::material::*;

pub struct Sphere<Mat: Material>
{
    pub center: Vec3,
    pub radius: f64,
    pub material_pointer: Mat
}

impl<Mat: Material> Sphere<Mat>
{
    pub fn new(center: Vec3, radius: f64, material_pointer:  Mat) -> Self{
        Sphere
        {
            center: center,
            radius: radius,
            material_pointer: material_pointer
        }
    }
}
impl<Mat: Material> Hitable for Sphere<Mat>
{
    fn hit(&mut self, r: Ray, t_min:f64, t_max: f64) -> Option<HitRecord>{
        let oc = r.origin() - self.center;
        let a= r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius  * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-half_b  - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let mut return_rec = 
                HitRecord { 
                    t:t,
                     p:p, 
                     normal:Vec3::new(0.0,0.0,0.0),
                     front_face: false,
                      material_pointer: &self.material_pointer 
                    };
                return_rec.set_face_normal(r,  (p - self.center) / self.radius);

                return Some(return_rec)
            }
            let t = (-half_b  + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let mut return_rec = 
                HitRecord { 
                    t:t,
                     p:p, 
                     normal:Vec3::new(0.0,0.0,0.0),
                     front_face: false,
                      material_pointer: &self.material_pointer 
                    };
                return_rec.set_face_normal(r,  (p - self.center) / self.radius);

                return Some(return_rec)
            }
        }
        None
    }
}