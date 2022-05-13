mod image_process;
mod vec3;
mod color;
mod ray;
mod hitable;
mod hitable_list;
mod utility;
mod sphere;
mod camera;
mod material;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::{Lambertian, Metal, Dielectric};
use crate::color::Color;
use crate::hitable_list::HitableList;
use crate::sphere::Sphere;
use crate::camera::Camera;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::{Mutex, Arc};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;


fn random_scene(rand_seed: &mut ChaCha8Rng) -> HitableList{
    let mut world = HitableList::default();
    world.add(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, Lambertian::new(Vec3::new(0.5, 0.5, 0.5))));

    for a  in -11..11 {
        for b in -11..11 {
            let choose_mat = utility::random_double_from_seed(rand_seed);
             let center = Vec3::new(a as f64 + 0.9*utility::random_double_from_seed(rand_seed), 0.2, b as f64 + 0.9*utility::random_double_from_seed(rand_seed));

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {

              
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random_from_seed(rand_seed) * Vec3::random_from_seed(rand_seed);
                    world.add(Sphere::new(center, 0.2,  Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo =  Vec3::random_range_from_seed(0.5, 1.0,rand_seed);
                    let fuzz = utility::random_double_range_from_seed(0.0, 0.5, rand_seed);
                    world.add(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    // glass
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));

    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0,  Lambertian::new(Vec3::new(0.4, 0.2, 0.1))));

    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)));

    world
}
fn ray_color(r: Ray,  world: &mut HitableList, depth: i32)-> Vec3
{
    
    if depth <= 0
    {
        return Vec3::new(0.0, 0.0, 0.0);
    }


    if let Some(hit_anything) = world.hit(r,0.001, std::f64::INFINITY)
    {
         let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0),Vec3::new(0.0, 0.0, 0.0));
         let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if hit_anything.material_pointer.scatter(r,hit_anything,&mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(scattered, world, depth - 1);
        }

         return Vec3::new(0.0,0.0,0.0);

    }
    let unit_dir = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3:: new(0.53, 0.46, 0.67)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::create(&args[1])
    .expect("Could not create file");

    //Image
    let aspect_ratio = 2.0;
    let image_width = 600;
    let image_height = (image_width as f32 / aspect_ratio)as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    //Camera
    let lookfrom = Vec3::new(13.0,2.0,3.0);
    let lookat= Vec3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio as f64, aperture, dist_to_focus);

    let image_shared_pointer = Arc::new(Mutex::new(image_process::PpmFormat::new(image_width,image_height,255)));
    

    let is_multi_thread = true;
    if !is_multi_thread
    {
        let mut rand_seed = ChaCha8Rng::seed_from_u64(45);
        let mut world = random_scene(&mut rand_seed);
        for  j in (0..image_height).rev(){
            
            
            //World 
            for i in 0..image_width{
                print!("스캔라인 남은 수: {}, {}\r", j, i);
                std::io::stdout().flush().unwrap();
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel
                {
                    let u = (i as f64 + utility::random_double()) / (image_width-1) as f64;
                    let v = (j as f64 + utility::random_double()) / (image_height-1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(r, &mut world, max_depth);
                    
                }
                let mut target_image = image_shared_pointer.lock().unwrap();
                target_image.write_color(color::fcolor_to_icolor(pixel_color, samples_per_pixel),i as usize,j as usize);
            }
        }
    }
    else{
    //1.스레드 별로 쪼갠다
    //
    let thread_count = 12;
    let draw_part_per_thread = image_height / thread_count;
    let residue_count = image_height % thread_count;
    println!("잔여: {}", residue_count);
    let mut handles = vec![];
    for thread_index in 0..thread_count
    {
        let result_vector = Arc::clone(&image_shared_pointer);
        let handle = thread::spawn(move || {
            let mut color_vector_per_thread:Vec<Vec<color::Color>> = Vec::new();
            let mut rand_seed = ChaCha8Rng::seed_from_u64(45);
            //World
            let mut world = random_scene(&mut rand_seed);
            println!("thread_id[{}]: [{},{})", thread_index, thread_index * draw_part_per_thread, (thread_index + 1) * draw_part_per_thread);
            for  j in thread_index * draw_part_per_thread.. (thread_index + 1) * draw_part_per_thread
            {
                let mut column_vec:Vec<Color> = Vec::new();

                for i in 0..image_width{
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel
                    {
                        let u = (i as f64 + utility::random_double()) / (image_width-1) as f64;
                        let v = (j as f64 + utility::random_double()) / (image_height-1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(r, &mut world, max_depth);
                        
                    }
                    column_vec.push(color::fcolor_to_icolor(pixel_color, samples_per_pixel));
                }

                color_vector_per_thread.push(column_vec);

            }
            let mut target_image = result_vector.lock().unwrap();
            target_image.write_color_rows(color_vector_per_thread, (thread_index * draw_part_per_thread) as usize, ((thread_index + 1) * draw_part_per_thread) as usize);


   });
    handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    
    }
    let target_image = image_shared_pointer.lock().unwrap();
    file.write_all(&format!("{}",target_image).into_bytes())
    .expect("파일에 쓸 수 없습니다!");
}

    
