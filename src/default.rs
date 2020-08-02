use crate::bin_image::*;
use crate::common::*;
use std::convert::TryFrom;
use std::io::Error;
#[cfg(test)]
use std::path::PathBuf;

fn recursive(image: BinImage) -> BinImage {
    let mut image_mut = image;
    let sub_iters = vec![SubIter::First, SubIter::Second];

    for sub_iter in sub_iters {
        let mut c = 0;
        let mut m = BinImage::new(image_mut.get_width(), image_mut.get_height(), false);
        let img_iter = image_mut.clone().into_iter();

        for (x, y, _) in img_iter {
            if image_mut.sub_iter(sub_iter.clone(), x, y) && image_mut.get_value(x, y).unwrap() {
                let _result = m.set_value(x, y, true);
                c += 1;
            }
        }

        image_mut = image_mut - m;

        if c == 0 {
            return image_mut;
        }
    }

    return recursive(image_mut);
}

pub fn imgthin(pixels: Vec<Vec<bool>>) -> Result<Vec<Vec<bool>>, Error> {
    let bin_image_result = BinImage::try_from(pixels);

    match bin_image_result {
        Ok(bin_image) => {
            let thinned = recursive(bin_image);
            Ok(thinned.get_pixels().to_vec())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_subiter() {
        let img = BinImage::try_from(PathBuf::from("./test_data/test_subiter.txt")).unwrap();

        assert_eq!(img.sub_iter(SubIter::First, 4, 4), false);
        assert_eq!(img.sub_iter(SubIter::First, 3, 1), true);
        assert_eq!(img.sub_iter(SubIter::Second, 3, 7), true);
        assert_eq!(img.sub_iter(SubIter::Second, 3, 6), false);
    }

    #[test]
    fn test_char_b() {
        let img = BinImage::try_from(PathBuf::from("./test_data/b_char.txt")).unwrap();

        let thinned = recursive(img);

        let expect_img =
            BinImage::try_from(PathBuf::from("./test_data/b_char_thinned.txt")).unwrap();
        assert_eq!(expect_img.get_pixels(), thinned.get_pixels());
    }
}
