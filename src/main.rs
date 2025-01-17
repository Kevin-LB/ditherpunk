use image::{*};

fn main() {
    let img_path = "image.png";
    
    let img = image::open(img_path).expect("Erreur lors de l'ouvertre de l'image");

    let mut img = img.to_rgba8();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = Rgba([255, 255, 255, 255]);
        }
    }

    img.save("question_5.png").expect("Erreur");

    println!("Image modifiée et sauvegardée avec succès !");
}
