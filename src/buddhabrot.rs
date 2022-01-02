use crate::input::UserInput;
use ndarray::prelude::*;
use ndarray::{Array3, Array2};
use image::{ImageBuffer, Rgb};
use crate::complex::{Complex, random_complex};

pub fn render_buddhabrot(input: &UserInput) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = Array3::<u8>::zeros(
        (3 as usize, input.image_width, input.image_height).f()
    );

    for channel in 2u8..3 {
        run_buddhabrot_on_channel(
            &mut img,
            channel,
            input.center,
            input.zoom,
            input.max_iterations,
            input.num_samples,
        );
    }

    let img_vec = img.into_raw_vec();

    return ImageBuffer::from_raw(
        input.image_width as u32,
        input.image_height as u32,
        img_vec,
    ).expect("Container should have the right size for the image dimensions");
}

fn run_buddhabrot_on_channel(
    img: &mut Array3::<u8>,
    img_channel: u8,
    center: Complex,
    zoom: f64,
    max_iteration: u32,
    num_samples: u32,
) {
    let pixel_dist = f64::powf(2.0, -zoom);
    let (_, width, height) = img.dim();

    let mut buddhabrot_distribution = Array2::<u32>::zeros((width, height).f());

    for _ in 0..num_samples {
        let c = random_complex(
            -2.0,
            1.0,
            -1.5,
            1.5,
        );
        let mut z = c;

        let mut trajectory = Vec::new();
        for _ in 0..max_iteration {
            z = z.square() + c;
            trajectory.push(z);

            if has_escaped_to_infinity(z) {
                break;
            }
        }

        if has_escaped_to_infinity(z) {
            for coordinate in &trajectory {
                let row = ((((coordinate.imag + center.imag) + (2.0 * pixel_dist)) / (4.0 * pixel_dist)) * (height as f64)) as i32;
                let column = ((((coordinate.real + center.real) + (2.0 * pixel_dist)) / (4.0 * pixel_dist)) * (width as f64)) as i32;
                let row_usize = row as usize;
                let col_usize = column as usize;

                if row >= 0 && column >= 0 && row_usize < height && col_usize < width {
                    buddhabrot_distribution[[col_usize,row_usize]] += 1;
                }
            }
        }
    }

    let mut normalising_constant = 1.0;
    for i in 0..height {
        for j in 0..width {
            normalising_constant = f64::max(normalising_constant, buddhabrot_distribution[[j,i]] as f64);
        }
    }

    for i in 0..height {
        for j in 0..width {
            img[[img_channel.into(),j,i]] = ((buddhabrot_distribution[[j,i]] as f64) / normalising_constant * 255.0) as u8;
        }
    }
}

fn has_escaped_to_infinity(num: Complex) -> bool {
    return ((num.real*num.real) + (num.imag*num.imag)) > 4.0;
}
