use std::ops;
use crate::utility::*;
use rand_chacha::ChaCha8Rng;
pub struct Vec3
{
    elements : [f64;3]
}
type Point = Vec3;


impl Vec3
{
    pub fn new(x: f64,y: f64,z: f64)->Self{
        Vec3{ 
            elements : [x,y,z]
        }
    }
    pub fn x(&self) ->f64
    {
        self.elements[0]
    }
    pub fn y(&self) ->f64
    {
        self.elements[1]
    }
    pub fn z(&self) ->f64
    {
        self.elements[2]
    }

    pub fn length_squared(&self) -> f64{
        self.elements[0] * self.elements[0]
        + self.elements[1] * self.elements[1]
        + self.elements[2] * self.elements[2]
    }
    pub fn length(&self) -> f64{
        self.length_squared().sqrt()
    }

    pub fn random() -> Self{
        Self
        {
            elements :[
                random_double(),
                random_double(),
                random_double()
            ]
            
        }

    }
    pub fn random_from_seed(seed: &mut ChaCha8Rng) -> Self{
        Self
        {
            elements :[
                random_double_from_seed(seed),
                random_double_from_seed(seed),
                random_double_from_seed(seed)
            ]
            
        }

    }
    pub fn random_range(min: f64, max: f64) -> Self{
        Self{
            elements: [
                random_double_range(min,max),
                random_double_range(min,max),
                random_double_range(min,max)
            ]

        }
    }

    pub fn random_range_from_seed(min: f64, max: f64, seed: &mut ChaCha8Rng) -> Self{
        Self{
            elements: [
                random_double_range_from_seed(min,max,seed),
                random_double_range_from_seed(min,max,seed),
                random_double_range_from_seed(min,max,seed)
            ]

        }
    }

    pub fn near_zero(&self) -> bool
    {
        let s = 1e-8;
        return 
        (self.elements[0].abs() < s) &&
        (self.elements[1].abs() < s) &&
        (self.elements[2].abs() < s);
    }

}
pub fn dot(a: &Vec3, b: &Vec3) -> f64
{
    a.elements[0] * b.elements[0] 
    + a.elements[1] * b.elements[1] 
    + a.elements[2] * b.elements[2] 
}
pub fn cross(a: &Vec3, b: &Vec3) -> Vec3
{
    Vec3{
    elements : 
    [
        a.elements[1] *  b.elements[2] - a.elements[2] *  b.elements[1],
        a.elements[2] *  b.elements[0] - a.elements[0] *  b.elements[2],
        a.elements[0] *  b.elements[1] - a.elements[1] *  b.elements[0],
        ]

    }
}
pub fn unit_vector(a: Vec3) -> Vec3
{
    a / a.clone().length()
}
pub fn reflect(v: Vec3, n: Vec3) -> Vec3
{
    v - 2.0 * dot(&v,&n) * n
}
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3{
    let cos_theta = if dot(&-uv, &n) > 1.0
    {
        1.0
    }
    else{
        dot(&-uv, &n)
    };
    let r_out_perp =  etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    return r_out_perp + r_out_parallel;
}
pub fn random_unit_vector() -> Vec3
{
    unit_vector(random_in_unit_sphere())
}
pub fn random_in_unit_disk() -> Vec3{
    loop{
        let p = Vec3::new(random_double_range(-1.0,1.0), random_double_range(-1.0,1.0), 0.0);
        if p.length_squared() >= 1.0
        {
            continue;
        }
        return p;
    }
}
pub fn random_in_unit_sphere() -> Vec3{
    loop
    {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0
        {
            return p;
        }
    }
}
pub fn random_in_hemisphere(normal: Vec3) -> Vec3
{
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, &normal) > 0.0
    {
        return in_unit_sphere;
    }
    else{
        return -in_unit_sphere;
    }
}
impl Copy for Vec3{ }
impl std::clone::Clone for Vec3
{
    fn clone(&self) -> Self {
        *self
    }
}
impl ops::Add<Vec3> for Vec3
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            elements :  [
                self.elements[0] + other.elements[0],
                self.elements[1] + other.elements[1],
                self.elements[2] + other.elements[2]
            ]

        }
    }
}

impl ops::Sub<Vec3> for Vec3
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            elements : [
                self.elements[0] - other.elements[0],
                self.elements[1] - other.elements[1],
                self.elements[2] - other.elements[2],
            ]

        }
    }
}

impl ops::Mul<Vec3> for Vec3
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            elements :[
                self.elements[0] * other.elements[0],
                self.elements[1] * other.elements[1],
                self.elements[2] * other.elements[2],
            ]

        }
    }
}
impl ops::Mul<f64> for Vec3
{
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            elements :[
                self.elements[0] * other,
                self.elements[1] * other,
                self.elements[2] * other,
            ]

        }
    }
}
impl ops::Mul<Vec3> for f64
{
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3{
        Vec3 {
            elements :[
                other.elements[0] *  self,
                other.elements[1] *  self,
                other.elements[2] * self
            ]

        }
    }
}
impl ops::Div<f64> for Vec3
{
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3{
        Vec3 {
            elements :[
                self.elements[0] *  (1.0 / other),
                self.elements[1] *  (1.0 / other),
                self.elements[2] *  (1.0 / other)
            ]

        }
    }
}
impl ops::AddAssign<Vec3> for Vec3
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            elements: [
                self.elements[0] + other.elements[0],
                self.elements[1] + other.elements[1],
                self.elements[2] + other.elements[2]
            ]

        };
    }
}

impl ops::SubAssign<Vec3> for Vec3
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            elements: [
                self.elements[0] - other.elements[0],
                self.elements[1] - other.elements[1],
                self.elements[2] - other.elements[2]
            ]

        };
    }
}
impl ops::Neg for Vec3
{
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{
            elements :[
                -self.elements[0],
                -self.elements[1],
                -self.elements[2],
            ]

        }
    }
}

impl ops::MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, _rhs: f64){
        self.elements[0] *= _rhs;
        self.elements[1] *= _rhs;
        self.elements[2] *= _rhs;
    }
}

impl ops::DivAssign<f64> for Vec3{
    fn div_assign(&mut self, _rhs: f64){
        self.elements[0] *= 1.0 / _rhs;
        self.elements[1] *= 1.0 / _rhs;
        self.elements[2] *= 1.0 / _rhs;
    }
}

impl ops::Index<usize> for Vec3
{
    type Output = f64;
    fn index(&self, _rhs: usize) -> &f64
    {
        &self.elements[_rhs]
    }
}

impl ops::IndexMut<usize> for Vec3
{
    fn index_mut(&mut self, _rhs: usize) -> &mut f64
    {
        &mut self.elements[_rhs]
    }
}