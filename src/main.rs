use std::path::Path;
use image::{self, GenericImageView};

fn read_image(path: &std::path::Path) -> Result<image::DynamicImage, std::io::Error> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let img_reader = image::ImageReader::new(reader).with_guessed_format()?;
    match img_reader.decode() {
        Ok(img) => Ok(img),
        _ => { Err(std::io::Error::new(std::io::ErrorKind::Other, "")) }
    }
}

fn merge(a: image::DynamicImage, b: image::DynamicImage) -> image::DynamicImage {
    // both images seem to be rotated 90 degrees counter-clockwise, so adjust
    let a = a.rotate90();
    let b = b.rotate90();
    // the hard part
    // find the combination (rotation-offset, x-offset, y-offset) that
    // has lowest error when one image is super imposed on the other
    a
}

fn main() {
    let left = read_image(Path::new("../testing/left.png")).unwrap();
    let right = read_image(Path::new("../testing/right.png")).unwrap();
    let merged = merge(left, right);
    merged.save_with_format(Path::new("merged.png"), image::ImageFormat::Png).unwrap();
}
