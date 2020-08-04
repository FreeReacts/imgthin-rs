use image::open;
use image::Rgb;
use image::RgbImage;
use imgthin::imgthin;
use std::path::PathBuf;

fn main() {
    let img = open(PathBuf::from("./image/original.png")).unwrap();

    let rgb_img = img.to_rgb();
    let (width, height) = rgb_img.dimensions();

    let rows = rgb_img.rows();

    let thinned = imgthin(
        rows.map(|r| r.map(|c|{
            let color: [u8;3] = c.0;

            color[0] < 200 && color[1] < 200 && color[2] < 200
        }).collect())
            .collect(),
    )
    .unwrap();

    let mut img = RgbImage::new(width, height);

    for (y, row) in thinned.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if val.to_owned() {
                img.put_pixel(x as u32, y as u32, Rgb::from([0, 0, 0]));
            } else {
                img.put_pixel(x as u32, y as u32, Rgb::from([255, 255, 255]));
            }
        }
    }

    img.save(PathBuf::from("./image/thinned.png")).expect("Can not save the image");
}
