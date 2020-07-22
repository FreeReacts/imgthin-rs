use std::io::{Error, ErrorKind};
use std::ops::Sub;

#[derive(Clone)]
struct BinImage {
    pixels: Vec<Vec<bool>>,
}

enum SubIter {
    First,
    Second,
}

impl BinImage {
    pub fn new(width: usize, height: usize) -> BinImage {
        let mut pixels: Vec<Vec<bool>> = vec![];

        for i in 0..height {
            let mut row: Vec<bool> = vec![];

            for j in 0..width {
                row.push(false);
            }

            pixels.push(row);
        }

        BinImage { pixels }
    }

    pub fn from_pixels(pixels: Vec<Vec<bool>>) -> Result<BinImage, Error> {
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
        }

        Ok(BinImage { pixels })
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
                Some(val) => {
                    self.pixels[y][x] = value;
                    Ok(())
                }
                None => Err(Error::new(ErrorKind::InvalidInput, "Invalid X coordinate")),
            },
            None => Err(Error::new(ErrorKind::InvalidInput, "Invalid Y coordinate")),
        }
    }

    pub fn get_pixels(self) -> Vec<Vec<bool>> {
        self.pixels
    }

    fn get_neighbors(
        &self,
        x: usize,
        y: usize,
    ) -> (bool, bool, bool, bool, bool, bool, bool, bool, bool) {
        (
            self.get_value(x, y).unwrap_or(false),
            self.get_value(x, y + 1).unwrap_or(false),
            self.get_value(x + 1, y + 1).unwrap_or(false),
            self.get_value(x + 1, y).unwrap_or(false),
            self.get_value(x + 1, y - 1).unwrap_or(false),
            self.get_value(x, y - 1).unwrap_or(false),
            self.get_value(x - 1, y - 1).unwrap_or(false),
            self.get_value(x - 1, y).unwrap_or(false),
            self.get_value(x - 1, y + 1).unwrap_or(false),
        )
    }

    fn sub_iter(&self, mode: SubIter, x: usize, y: usize) -> bool {
        let (_, p2, p3, p4, p5, p6, p7, p8, p9) = self.get_neighbors(x, y);
        let (a_p, b_p) = calculate_ap_and_bp(p2, p3, p4, p5, p6, p7, p8, p9);

        let a = 2 <= b_p && b_p <= 6;
        let b = a_p == 1;
        let (c, d) = match mode {
            SubIter::First => (p2 && p4 && p6, p4 && p6 && p8),
            SubIter::Second => (p2 && p4 && p8, p2 && p6 && p8),
        };

        a && b && c && d
    }
}

impl Sub<BinImage> for BinImage {
    type Output = BinImage;

    fn sub(self, rhs: BinImage)->BinImage {
        unimplemented!()
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

pub fn recursive(pixels: Vec<Vec<bool>>, c: usize) -> (Vec<Vec<bool>>, usize) {
    unimplemented!()
}

pub fn imgthin(pixels: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let (thinned, _) = recursive(pixels, 0);

    unimplemented!();
}
