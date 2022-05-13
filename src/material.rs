use crate::ray::*;
use crate::hitable::*;
use crate::vec3::*;
use crate::utility::*;

pub trait Material: Sync
{
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;   
}

pub struct Lambertian{
    pub albedo: Vec3
}


impl Lambertian
{
    pub fn new(albedo: Vec3) ->Self
    {
        Self{
            albedo:albedo
        }
    }
}
impl Material for Lambertian
{
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool
    {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero()
        {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }    
}

pub struct Metal{
    pub albedo: Vec3,
    pub fuzz: f64
}
impl Metal
{
    pub fn new(albedo: Vec3, fuzz:f64) ->Self
    {
        Self{
            albedo:albedo,
            fuzz: 
            if fuzz < 1.0{
                fuzz
            }
            else 
            {
                1.0
            }
        }
    }
}
impl Material for Metal
{
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool
    {
       let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
       *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
       *attenuation = self.albedo;
       return dot(&scattered.direction(), &rec.normal) > 0.0;
    }    
}

pub struct Dielectric{

    pub ir:f64, // Index of Refraction
}

impl Dielectric
{
   pub fn new(index_of_refraction: f64)->Self{
        Self{
            ir: index_of_refraction
        }
   }
   pub fn reflectance(cosine: f64, ref_idx: f64) -> f64{
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    return r0 + (1.0-r0) * (1.0 - cosine).powf(5.0);
}
}

impl Material for Dielectric
{
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) ->bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face
        {
            1.0/self.ir
        }
        else{
            self.ir
        };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = if dot(&-unit_direction, &rec.normal)> 1.0
        {
            1.0
        }else{
            dot(&-unit_direction, &rec.normal)
        };
    
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            let direction;

            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double()
            {
                direction = reflect(unit_direction, rec.normal);
            }
            else
            {
                direction = refract(unit_direction, rec.normal, refraction_ratio);
            }
            *scattered = Ray::new(rec.p, direction);
        return true;
    }

}