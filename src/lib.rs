use std::io::{Error, ErrorKind};

#[derive(Clone)]
pub struct BinImage {
    pixels: Vec<Vec<bool>>
}

impl BinImage {
    pub fn new(pixels: Vec<Vec<bool>>)->Result<BinImage,Error> {
        let mut first = true;
        let mut width = 0;
        for row in pixels.iter() {
            if first {
                width = row.len();
            } else {
                if width != row.len() {
                    return Err(Error::new(ErrorKind::InvalidInput, "Invalid image"))
                }
            }
        }

        Ok(BinImage {
            pixels
        })
    }

    pub fn get_width(&self)->usize {
        match self.pixels.get(0) {
            Some(row)=>{
                row.len()
            }
            None=>0
        } 
    }

    pub fn get_height(&self)->usize {
        self.pixels.len()
    }

    pub fn get_value(&self, x: usize, y: usize)->Result<bool, Error> {
        match self.pixels.get(y) {
            Some(row)=>{
                match row.get(x){
                    Some(val)=>{
                        Ok(val.clone())
                    }
                    None=>Err(Error::new(ErrorKind::InvalidInput, "Invalid X coordinate"))
                }
            }
            None=>Err(Error::new(ErrorKind::InvalidInput, "Invalid Y coordinate"))
        }
    }

    pub fn set_value(&mut self, x:usize, y:usize, value: bool)->Result<(), Error> {
        match self.pixels.get(y) {
            Some(row)=>{
                match row.get(x) {
                    Some(val)=>{
                        self.pixels[y][x] = value;
                        Ok(())
                    }
                    None=>Err(Error::new(ErrorKind::InvalidInput, "Invalid X coordinate"))
                }
            }
            None=>Err(Error::new(ErrorKind::InvalidInput, "Invalid Y coordinate"))
        }
    }

    pub fn get_pixels(self)->Vec<Vec<bool>> {
        self.pixels
    }

}


pub fn imgthin(pixels: Vec<Vec<bool>> )-> Vec<Vec<bool>>{
    let mut c =0;

    let img = BinImage::new(pixels);

    unimplemented!();
}

