use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage, Pixel};
use std::path::Path;

fn main() {
    let img_path = Path::new("image.png");
    
    let img = image::open(&img_path).expect("Erreur lors de l'ouvertre de l'image");
    let pixel = img.get_pixel(32, 52);

    println!("La couleur du pixel (32, 52) est : {:?}", pixel.to_rgb());
}
