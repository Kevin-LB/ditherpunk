use image::{*};
use std::env;
use std::path::Path;

/// Convertit une couleur hexadécimale en un tuple (R, G, B)
fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), &'static str> {
    if hex.len() != 6 {
        return Err("La couleur doit être au format hexadécimal RRGGBB");
    }
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex value")?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex value")?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex value")?;
    Ok((r, g, b))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <color_black> <color_white>", args[0]);
        std::process::exit(1);
    }

    let color_black = &args[1];
    let color_white = &args[2];

    let black_rgb = hex_to_rgb(color_black).expect("Invalid black color value");
    let white_rgb = hex_to_rgb(color_white).expect("Invalid white color value");

    let img_path = "image.png";
    let img = image::open(img_path).expect("Erreur lors de l'ouvertre de l'image");

    let mut img = img.to_rgba8();

    for pixel in img.pixels_mut() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        // Calculer la luminosité
        let luminosity = 0.299 * r + 0.587 * g + 0.114 * b;

        // Appliquer les couleurs
        if luminosity > 128.0 {
            *pixel = Rgba([white_rgb.0, white_rgb.1, white_rgb.2, 255]);
        } else {
            *pixel = Rgba([black_rgb.0, black_rgb.1, black_rgb.2, 255]);
        }
    }

    img.save("question_8.png").expect("Erreur");

    println!("Image modifiée et sauvegardée avec succès !");
}
