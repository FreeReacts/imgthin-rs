use crate::bin_image::*;
use crate::common::*;
use std::convert::TryFrom;
use std::io::Error;

// Improved Version Of The Algorithm
pub fn imgthin(pixels: Vec<Vec<bool>>) -> Result<Vec<Vec<bool>>, Error> {
    let k_t = BinImage::try_from(pixels)?;
    let mut s_t = k_t.clone();

    // Making the mapping table
    let table: Vec<Vec<bool>> = vec!();

    for i in 0..16 {
        let i_bin = format!("{:04b}",i);
        let mut row = vec!();
        for j in 0..16 {
            let j_bin = format!("{:04b}", j).as_str(); 
            let bin = i_bin + j_bin;


        }
    }

    Ok(s_t.get_pixels().to_vec())

}

#[cfg(test)]
mod test {
 use super::*;


}
