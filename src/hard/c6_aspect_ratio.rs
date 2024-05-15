/*
 * Crea un programa que se encargue de calcular el aspect ratio de una
 * imagen a partir de una url.
 * - Url de ejemplo:
 *   https://raw.githubusercontent.com/mouredevmouredev/master/mouredev_github_profile.png
 * - Por ratio hacemos referencia por ejemplo a los "16:9" de una
 *   imagen de 1920*1080px.
 */

use reqwest;
use image::io::Reader as ImageReader;
use image::GenericImageView;

pub fn aspect_ratio(){
    let url="https://www.educaciontrespuntocero.com/wp-content/uploads/2020/04/mejores-bancos-de-imagenes-gratis.jpg";

    let response = reqwest::blocking::get(url).expect("Failed to fetch URL");
    let image_bytes = response.bytes().expect("Failed to read response");

    let image_reader=ImageReader::new(std::io::Cursor::new(image_bytes))
        .with_guessed_format()
        .expect("Failed to guess image format")
        .decode()
        .expect("Failed to decode image");

    let width = image_reader.width();
    let height = image_reader.height();
    
    println!("Width: {}, Height: {}", width, height);

    let aspect_ratio = width as f64 / height as f64;
    println!("Aspect Ratio: {:.2}", aspect_ratio);
}
