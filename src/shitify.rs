use image::{codecs::jpeg::JpegEncoder, ImageReader};
use rand::Rng;

pub fn shitify(path: &String, out: &String) -> Result<(), image::ImageError> {

    let img = ImageReader::open(path)?.decode()?;

    let img = img.blur(2.5);
    let mut img = img.huerotate(360/64);

    if rand::random_bool(1.0/4.0) {
        let width = img.width();
        let height = img.height();
        img = img.thumbnail(256, 128);
        img = img.resize(width, height, image::imageops::FilterType::CatmullRom)
    }

    let mut img = img.adjust_contrast(75.0);
    
    if rand::random_bool(1.0/5.0) {
        img.invert();
    }
    
    let mut quality = 100;

    if !rand::random_bool(1.0/85.0) {
        if rand::random_bool(1.0/2.0) {
            quality = 1;
        } else {
            quality = rand::rng().random_range(1..=25);
        }
    }

    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);
        encoder.encode_image(&img)?;
    }
    
    std::fs::write(out, buffer)?;

    Ok(())
}
