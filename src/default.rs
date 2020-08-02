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
        let img = BinImage::try_from(PathBuf::from("./test_data/test_get_neighbors.txt")).unwrap();

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
        let img = BinImage::try_from(PathBuf::from("./test_data/test_sub.txt")).unwrap();

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

        let expect_img = BinImage::try_from(PathBuf::from("./test_data/b_char_thinned.txt")).unwrap();
        assert_eq!(expect_img.get_pixels(), thinned.get_pixels());
        
    }
}
