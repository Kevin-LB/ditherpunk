use image::GenericImageView;

fn main() {
    let img = image::open("test.png").expect("Erreur");
    println!("Dimensions: {:?}", img.dimensions());
    println!("Color type: {:?}", img.color());
}