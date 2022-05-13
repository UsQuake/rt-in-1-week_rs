use crate::vec3::*;

pub struct Ray {
      orig : Vec3,
      dir : Vec3
}
impl Copy for Ray { }
impl std::clone::Clone for Ray
{
    fn clone(&self) -> Self {
        *self
     }
}
impl Ray{
    pub fn new(origin : Vec3, direction : Vec3) ->Self
    {
       Self{
        orig: origin,
        dir: direction
       } 
    }
    pub fn origin(&self) -> Vec3 { 
        self.orig
    }
    pub fn direction(&self) -> Vec3 { 
        self.dir
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
    pub fn point_at_parameter(&self, t: f64) -> Vec3 { 
        self.orig + t * self.dir 
    }
}