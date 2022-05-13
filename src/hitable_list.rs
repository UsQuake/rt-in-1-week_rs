use crate::hitable::*;
use crate::ray::*;

#[derive(Default)]
pub struct HitableList
{
    pub objects: Vec<Box<dyn Hitable>>
}
unsafe impl Send for HitableList
{

}

unsafe impl Sync for HitableList
{

}
impl HitableList
{
    pub fn new() -> Self
    {
        Self{
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: impl Hitable + 'static)
    {
        self.objects.push(Box::new(object));
    }
    pub fn hit(&mut self, r: Ray, t_min:f64, t_max: f64) -> Option<HitRecord>
    {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for object in self.objects.iter_mut()
        {
            if let Some(hit) = object.hit(r, t_min, closest_so_far)
            {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
       hit_anything
    }
}
// impl hitable for hitable_list
// {

// }