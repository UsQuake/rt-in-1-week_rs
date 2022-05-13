use crate::vec3::*;
use crate::ray::*;
use crate::material::*;

#[derive(Clone, Copy)]
pub struct HitRecord<'a>
{ 
    
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material_pointer: &'a dyn Material
}
impl<'a> HitRecord<'a>
{
    pub fn new(material_pointer: &'a mut dyn Material) -> Self
    {
        HitRecord
        { 
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            material_pointer: material_pointer,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal : Vec3)
    {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        
        self.normal = if self.front_face
         {
            outward_normal
         }
         else
         {
            -outward_normal
         }
    }
}


pub trait Hitable: Sync
{
    fn hit(&mut self, r: Ray, t_min:f64, t_max: f64) -> Option<HitRecord>;
}