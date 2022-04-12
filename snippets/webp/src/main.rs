fn main() {
    let file_name = std::env::args().nth(1).expect("Need a image name");
    let _img = image::open(file_name).unwrap();
}
