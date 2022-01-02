use text_io::scan;
use crate::complex::Complex;

pub struct UserInput {
    pub image_width: usize,
    pub image_height: usize,
    pub center: Complex,
    pub zoom: f64,
    pub max_iterations: u32,
    pub num_samples: u32,
}

pub fn get_user_input() -> UserInput {
    let mut image_width: usize = 0;
    let mut image_height: usize = 0;
    let mut center_real: f64 = 0.0;
    let mut center_imag: f64 = 0.0;
    let mut zoom: f64 = 0.0;
    let mut max_iterations: u32 = 0;
    let mut num_samples: u32 = 0;

    println!("Input the desired image width and height in pixels in the format: width,height");
    { scan!("{},{}\n", image_width, image_height); }
    println!("You input width: {}, height: {}", image_width, image_height);

    println!("Input the desired image center complex number in the format: real,imaginary");
    { scan!("{},{}\n", center_real, center_imag); }

    let center = Complex {
        real: center_real,
        imag: center_imag,
    };

    println!("You input {}", center);

    println!("Input the desired image zoom");
    { scan!("{}\n", zoom); }
    println!("You input {}", zoom);

    println!("Input the desired max iterations");
    { scan!("{}\n", max_iterations); }
    println!("You input {}", max_iterations);

    println!("Input the desired number of samples");
    { scan!("{}\n", num_samples); }
    println!("You input {}", num_samples);

    return UserInput {
        image_width,
        image_height,
        center,
        zoom,
        max_iterations,
        num_samples,
    };
}