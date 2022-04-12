use webp::*;

fn main() {
    let file_name = std::env::args().nth(1).expect("Need a image name");
    let img = image::open(file_name).unwrap();

    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    let webp: WebPMemory = encoder.encode(75_f32);
    std::fs::write("new.webp", &*webp).unwrap();
}
