use crate::complex::Complex;

mod input;
mod buddhabrot;
mod complex;

fn main() {
    let user_input = input::get_user_input();
    let image = buddhabrot::render_buddhabrot(&user_input);
    image.save("buddhabrot.jpg").expect("Image saves successfully");
}


