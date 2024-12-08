# Raytracing in one weekend - MT-Patched

 ## Introduction

  Multithreaded patched version of rt-in-one-weekend-rs.

 ## Setup and Requirements
 
 - Rust version **1.71.1** [2021 edition]

 ## Explanation
 ![image](https://user-images.githubusercontent.com/24998577/168279934-45e2c5d2-aad8-45a2-8b97-fc671ac2a9db.png)
  
  ### 1. Select shared resources to do tasks in parallel.
  - I used *Atomic-Reference-counter* to release resources with atomic operation.
  - Also I used *Mutex* to update critical section safely.
  - ```Rust
    let image_shared_pointer = Arc::new(Mutex::new(image_process::PpmFormat::new(image_width,image_height,255)));
    let progress_shared_pointer = Arc::new(Mutex::new(indicate_bar::IndicateBar::new(image_height as u32)));
    ```
  ### 2. Write a single threaded version to debug with *differential test*.
  - *Differential test* is easy. Just compare output between codes which should emit same results.
  - To do differential test, We always retain single threaded version of parallel codes.
  - ```Rust
    if !is_multi_thread
    {
        let mut rand_seed = ChaCha8Rng::seed_from_u64(45);
        let mut world = random_scene(&mut rand_seed);
        let progress_bar = Arc::clone(&progress_shared_pointer);
        for  j in (0..image_height).rev(){
            for i in 0..image_width{
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
            let mut progress_indicator = progress_bar.lock().unwrap();
            progress_indicator.progress_one();
        }
    }
    ```
  ### 3. Seperate atomic tasks in parallel.
  - I simply separated draw part with column in parallel.
  - ```Rust
       let thread_count = 12;
        let draw_part_per_thread = image_height / thread_count;
        let residue_count = image_height % thread_count;
        //Draw Residue By Single Thread
        {
            let result_vector = Arc::clone(&image_shared_pointer);
            let mut rand_seed = ChaCha8Rng::seed_from_u64(45);
            //World
            let mut world = random_scene(&mut rand_seed);
            let mut color_vector_per_thread:Vec<Vec<color::Color>> = Vec::new();
            let progress_bar = Arc::clone(&progress_shared_pointer);
            for  j in 0.. residue_count
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
                let mut progress_indicator = progress_bar.lock().unwrap();
                progress_indicator.progress_one();
            }
            let mut target_image = result_vector.lock().unwrap();
            target_image.write_color_rows(color_vector_per_thread, 0 , residue_count as usize);
    ```
 ## Acknowledge

 - [rt-in-one-weekend in rust](https://github.com/fralken/ray-tracing-in-one-weekend)


