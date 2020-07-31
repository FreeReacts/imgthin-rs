use std::convert::TryFrom;
use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::ops::Sub;

#[derive(Clone)]
struct BinImage {
    pixels: Vec<Vec<bool>>,
}

#[derive(Clone)]
enum SubIter {
    First,
    Second,
}

impl BinImage {
    /// Creating a new image for a given width and height
    pub fn new(width: usize, height: usize, fill_color: bool) -> BinImage {
        let mut pixels: Vec<Vec<bool>> = vec![];

        for _ in 0..height {
            let mut row: Vec<bool> = vec![];

            for _ in 0..width {
                row.push(fill_color);
            }

            pixels.push(row);
        }

        BinImage { pixels }
    }

    pub fn get_width(&self) -> usize {
        match self.pixels.get(0) {
            Some(row) => row.len(),
            None => 0,
        }
    }

    pub fn get_height(&self) -> usize {
        self.pixels.len()
    }

    pub fn get_value(&self, x: usize, y: usize) -> Result<bool, Error> {
        match self.pixels.get(y) {
            Some(row) => match row.get(x) {
                Some(val) => Ok(val.to_owned()),
                None => Err(Error::new(ErrorKind::InvalidInput, "Invalid X coordinate")),
            },
            None => Err(Error::new(ErrorKind::InvalidInput, "Invalid Y coordinate")),
        }
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: bool) -> Result<(), Error> {
        match self.pixels.get(y) {
            Some(row) => match row.get(x) {
                Some(_) => {
                    self.pixels[y][x] = value;
                    Ok(())
                }
                None => Err(Error::new(ErrorKind::InvalidInput, "Invalid X coordinate")),
            },
            None => Err(Error::new(ErrorKind::InvalidInput, "Invalid Y coordinate")),
        }
    }

    pub fn get_pixels(&self) -> &Vec<Vec<bool>> {
        &self.pixels
    }

    pub fn get_neighbors(
        &self,
        x: usize,
        y: usize,
    ) -> (bool, bool, bool, bool, bool, bool, bool, bool, bool) {
        (
            self.get_value(x, y).unwrap_or(false),
            if y == 0 {
                false
            } else {
                self.get_value(x, y - 1).unwrap_or(false)
            },
            if y == 0 {
                false
            } else {
                self.get_value(x + 1, y - 1).unwrap_or(false)
            },
            self.get_value(x + 1, y).unwrap_or(false),
            self.get_value(x + 1, y + 1).unwrap_or(false),
            self.get_value(x, y + 1).unwrap_or(false),
            if x == 0 {
                false
            } else {
                self.get_value(x - 1, y + 1).unwrap_or(false)
            },
            if x == 0 {
                false
            } else {
                self.get_value(x - 1, y).unwrap_or(false)
            },
            if x == 0 || y == 0 {
                false
            } else {
                self.get_value(x - 1, y - 1).unwrap_or(false)
            },
        )
    }

    pub fn sub_iter(&self, mode: SubIter, x: usize, y: usize) -> bool {
        let (_, p2, p3, p4, p5, p6, p7, p8, p9) = self.get_neighbors(x, y);
        let (a_p, b_p) = calculate_ap_and_bp(p2, p3, p4, p5, p6, p7, p8, p9);

        let a = 2 <= b_p && b_p <= 6;

        let b = a_p == 1;

        let (c, d) = match mode {
            SubIter::First => (!(p2 && p4 && p6), !(p4 && p6 && p8)),
            SubIter::Second => (!(p2 && p4 && p8), !(p2 && p6 && p8)),
        };

        a && b && c && d
    }
}

// Iterator
struct BinImageIntoIter {
    bin_image: BinImage,
    x: usize,
    y: usize,
}

impl IntoIterator for BinImage {
    type Item = (usize, usize, bool);
    type IntoIter = BinImageIntoIter;

    fn into_iter(self) -> BinImageIntoIter {
        BinImageIntoIter {
            bin_image: self,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for BinImageIntoIter {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<(usize, usize, bool)> {
        let height = self.bin_image.get_height();
        let width = self.bin_image.get_width();

        if height - 1 < self.y && self.x == 0 {
            return None;
        }

        let ret = (
            self.x,
            self.y,
            self.bin_image.get_value(self.x, self.y).unwrap(),
        );

        if self.x >= width - 1 {
            self.y += 1;
            self.x = 0;
        } else {
            self.x += 1;
        }

        Some(ret)
    }
}

// Creating a binary image from binary data
impl TryFrom<Vec<Vec<bool>>> for BinImage {
    type Error = Error;

    fn try_from(pixels: Vec<Vec<bool>>) -> Result<BinImage, Error> {
        let mut first = true;
        let mut width = 0;
        for row in pixels.iter() {
            if first {
                width = row.len();
            } else {
                if width != row.len() {
                    return Err(Error::new(ErrorKind::InvalidInput, "Invalid image"));
                }
            }
            first = false;
        }

        Ok(BinImage { pixels })
    }
}

// Substracting a binary image from another image
impl Sub<BinImage> for BinImage {
    type Output = BinImage;

    fn sub(self, rhs: BinImage) -> BinImage {
        let mut new_img = self.clone();

        let iter = rhs.into_iter();

        for (x, y, val) in iter {
            if val {
                new_img.set_value(x, y, false).unwrap();
            }
        }

        new_img
    }
}

impl Display for BinImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_str = String::new();

        let pixels = self.get_pixels();
        let width = self.get_width();
        let height = self.get_height();

        fn count_digit(number: usize) -> usize {
            let mut count = 0;
            let mut n = number;

            while n != 0 {
                n = n / 10;
                count += 1;
            }
            return count;
        }

        fn getdigit(number: usize, n: usize) -> usize {
            let base: usize = 10;
            let mut r = number / (base.pow(n as u32));
            r = r % 10;
            r
        }

        let x_dig = count_digit(width - 1);
        let y_dig = count_digit(height - 1);
        let base: usize = 10;

        for dig in 0..x_dig {
            for _ in 0..y_dig {
                display_str.push(' ');
            }
            display_str.push('|');
            for i in 0..width {
                let digit = if i >= base.pow((x_dig - dig - 1) as u32) {
                    let digi = getdigit(i, x_dig - dig - 1);
                    digi.to_string()
                } else {
                    String::from(if x_dig - dig == 1 && i == 0 { "0" } else { " " })
                };

                display_str.push_str(&digit);
                display_str.push('|');
            }
            display_str.push('\n');
        }

        for (y, row) in pixels.iter().enumerate() {
            for dig in 0..y_dig {
                let digit = if y >= base.pow((x_dig - dig - 1) as u32) {
                    let digi = getdigit(y, x_dig - dig - 1);
                    digi.to_string()
                } else {
                    String::from(if x_dig - dig == 1 && y == 0 { "0" } else { " " })
                };

                display_str.push_str(&digit);
            }
            display_str.push('|');

            for col in row.iter() {
                if col.to_owned() {
                    display_str.push('X');
                } else {
                    display_str.push('_');
                }
                display_str.push('|')
            }
            display_str.push('\n');
        }
        write!(f, "{}", display_str)
    }
}

fn calculate_ap_and_bp(
    p2: bool,
    p3: bool,
    p4: bool,
    p5: bool,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: bool,
) -> (usize, usize) {
    let arr = [p2, p3, p4, p5, p6, p7, p8, p9];

    let mut a_p = 0;
    let mut prev_p = true;

    for p in arr.iter() {
        if !prev_p && p.to_owned() {
            a_p += 1;
        }
        prev_p = p.to_owned();
    }

    let b_p = arr.iter().map(|p| if *p { 1 } else { 0 }).sum::<usize>();

    (a_p, b_p)
}

fn recursive(image: BinImage, c: usize) -> (BinImage, usize) {
    // Searching for M that satisfied the first subiteration
    let mut c = c;

    let mut image_mut = image;
    let sub_iters = vec![SubIter::First, SubIter::Second];

    for sub_iter in sub_iters {
        if let SubIter::Second = sub_iter {
            c = 0;
        }
        let mut m = BinImage::new(image_mut.get_width(), image_mut.get_height(), false);
        let img_iter = image_mut.clone().into_iter();
        println!("{}", image_mut);
        for (x, y, _) in img_iter {
            if image_mut.sub_iter(sub_iter.clone(), x, y) && image_mut.get_value(x, y).unwrap() {
                let _result = m.set_value(x, y, true);
                c += 1;
            }
        }

        image_mut = image_mut - m;
        if c == 0 {
            return (image_mut, c);
        }
    }

    return recursive(image_mut, c);
}

pub fn imgthin(pixels: Vec<Vec<bool>>) -> Result<Vec<Vec<bool>>, Error> {
    let bin_image_result = BinImage::try_from(pixels);

    match bin_image_result {
        Ok(bin_image) => {
            println!("{}", bin_image);

            let (thinned, _) = recursive(bin_image, 0);

            println!("{}", thinned);
            Ok(thinned.get_pixels().to_vec())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_calculate_ap_bp() {
        assert_eq!(
            calculate_ap_and_bp(true, true, true, true, true, true, true, true),
            (0, 8)
        );
        assert_eq!(
            calculate_ap_and_bp(true, true, false, false, true, false, true, true),
            (2, 5)
        );
        assert_eq!(
            calculate_ap_and_bp(false, false, false, false, false, false, false, false),
            (0, 0)
        );
    }

    #[test]
    fn test_get_neighbors() {
        let img = BinImage::try_from(vec![
            vec![false, true, false, true, false, true, false],
            vec![false, true, false, true, false, true, false],
            vec![false, true, false, true, false, true, false],
            vec![false, false, false, true, false, true, true],
            vec![false, true, false, true, false, true, false],
            vec![true, true, false, true, true, true, false],
            vec![false, true, false, true, false, true, false],
            vec![false, true, false, true, false, true, false],
        ])
        .unwrap();

        assert_eq!(
            img.get_neighbors(4, 5),
            (true, false, true, true, true, false, true, true, true)
        );
        assert_eq!(
            img.get_neighbors(0, 5),
            (true, false, true, true, true, false, false, false, false)
        );
        assert_eq!(
            img.get_neighbors(1, 3),
            (false, true, false, false, false, true, false, false, false)
        );
    }

    #[test]
    fn test_sub() {
        let img = BinImage::try_from(vec![
            vec![true, false, true, true],
            vec![false, true, false, true],
            vec![true, true, false, true],
        ])
        .unwrap();

        let empty_img = BinImage::new(4, 3, false);

        let mut test_1_img = empty_img.clone();
        test_1_img.set_value(1, 1, true).unwrap();
        test_1_img.set_value(3, 2, true).unwrap();

        let sub_1 = img.clone() - test_1_img;

        assert_eq!(
            sub_1.get_pixels().to_vec(),
            vec!(
                vec!(true, false, true, true),
                vec!(false, false, false, true),
                vec!(true, true, false, false)
            )
        );
    }

    #[test]
    fn test_subiter() {
        let img = BinImage::try_from(vec![
            vec![false, true, false, true, false, true, false],
            vec![false, true, false, true, false, true, false],
            vec![false, true, false, true, false, true, false],
            vec![false, false, false, true, false, true, true],
            vec![false, true, false, true, false, true, false],
            vec![true, true, false, false, true, true, false],
            vec![false, true, false, true, true, true, false],
            vec![false, true, false, true, false, true, false],
        ])
        .unwrap();

        println!("{}", img);

        assert_eq!(img.sub_iter(SubIter::First, 4, 5), false);
    }
}
