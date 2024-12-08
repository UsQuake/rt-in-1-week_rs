# Raytracing in one weekend - MT-Patched

 ## Introduction

  Multithreaded patched version of rt-in-one-weekend-rs.

 ## Setup and Requirements
 
 - Rust version **1.71.1** [2021 edition]

 ## Explanation
 ![image](https://user-images.githubusercontent.com/24998577/168279934-45e2c5d2-aad8-45a2-8b97-fc671ac2a9db.png)
  
  ### 1. Seperate each part of tasks in parrellel.
  - I used *Atomic-Reference-counter* to release resources with atomic operation.
  - Also I used *Mutex* to update critical section safely.
  - ```Rust
    let image_shared_pointer = Arc::new(Mutex::new(image_process::PpmFormat::new(image_width,image_height,255)));
    let progress_shared_pointer = Arc::new(Mutex::new(indicate_bar::IndicateBar::new(image_height as u32)));
    ```
  ### 2. Seperate each part of tasks in parrellel.
  - I used *Atomic-Reference-counter* to release resources with atomic operation.
  - Also I used *Mutex* to update critical section safely.
  - ```Rust
    let image_shared_pointer = Arc::new(Mutex::new(image_process::PpmFormat::new(image_width,image_height,255)));
    let progress_shared_pointer = Arc::new(Mutex::new(indicate_bar::IndicateBar::new(image_height as u32)));
    ```
  ### 1. Seperate each part of tasks in parrellel.
  - I used *Atomic-Reference-counter* to release resources with atomic operation.
  - Also I used *Mutex* to update critical section safely.
  - ```Rust
    let image_shared_pointer = Arc::new(Mutex::new(image_process::PpmFormat::new(image_width,image_height,255)));
    let progress_shared_pointer = Arc::new(Mutex::new(indicate_bar::IndicateBar::new(image_height as u32)));
    ```
 ## Acknowledge

 - [rt-in-one-weekend in rust](https://github.com/fralken/ray-tracing-in-one-weekend)


