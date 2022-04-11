use image::GenericImageView;

fn main() {
    let img = image::open("wallpaper.png").unwrap();

    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    img.save("wallpaper.jpg").unwrap();
}
