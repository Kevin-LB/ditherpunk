use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage};
use std::path::Path;

fn main() {
    let img_path = Path::new("image.png");
    
    let img = image::open(&img_path).expect("Erreur lors de l'ouvertre de l'image");
    let rgb_img: RgbImage = img.to_rgb8();

    let output_path = Path::new("output.png");
    rgb_img.save(output_path).expect("Erreur lors de l'enregistrement de l'image");

    println!("Image convertie et sauvegardée avec succès !");
}