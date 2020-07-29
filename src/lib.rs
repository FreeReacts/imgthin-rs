use std::io::{Error, ErrorKind};
use std::ops::Sub;
use std::convert::TryFrom;

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

struct BinImageIntoIter {
    bin_image: BinImage,
    x: usize,
    y: usize
}

impl IntoIterator for BinImage {
    type Item = (usize, usize, bool);
    type IntoIter = BinImageIntoIter;

    fn into_iter(self)->BinImageIntoIter {
        BinImageIntoIter {
            bin_image: self,
            x: 0,
            y: 0
        }
    }
}

impl Iterator for BinImageIntoIter {
   type Item = (usize, usize, bool);

   fn next(&mut self)->Option<(usize, usize, bool)> {
        
       let height = self.bin_image.get_height();
       let width = self.bin_image.get_width();

       if height>=self.y && width >= self.x {
            return None;
       }

       let ret = (self.x, self.y, self.bin_image.get_value(self.x, self.y).unwrap());

       if self.x>= width {
        self.y +=1;
        self.x =0;
       } else {
        self. x += 1;
       }

       Some(ret)
   }
}

impl TryFrom<Vec<Vec<bool>>> for BinImage {
    type Error = Error;

    fn try_from(pixels: Vec<Vec<bool>>)->Result<BinImage, Error>{
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

impl Sub<BinImage> for BinImage {
    type Output = BinImage;

    fn sub(self, rhs: BinImage)->BinImage {
        let mut new_img = self.clone();
        
        let iter = rhs.into_iter();

        for (x,y,val) in iter {
            if val {
                let _result = new_img.set_value(x,y, false); 
            }
        }

        new_img
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
    let sub_iters = vec!(SubIter::First, SubIter::Second);

    for sub_iter in sub_iters {
    
        let mut m = BinImage::new(image_mut.get_width(), image_mut.get_height(), false);
        let img_iter = image_mut.clone().into_iter();

        for (x,y,_) in img_iter {
            if image_mut.sub_iter(sub_iter.clone(), x, y) {
                let _result = m.set_value(x, y, true);
                c +=1;
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
        Ok(bin_image)=>{
            let (thinned, _) = recursive(bin_image, 0);

            Ok(thinned.get_pixels())

        }
        Err(e)=>Err(e)
    }
}
