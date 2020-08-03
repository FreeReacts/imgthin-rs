use crate::bin_image::*;
use crate::common::*;
use std::convert::TryFrom;
use std::io::Error;
#[cfg(test)]
use std::path::PathBuf;

pub fn imgthin(pixels: Vec<Vec<bool>>) -> Result<Vec<Vec<bool>>, Error> {
    let bin_image_result = BinImage::try_from(pixels);

    match bin_image_result {
        Ok(mut image_mut) => {
            let mut c = 0;
            let mut first = true;
            let width = image_mut.get_width();
            let height = image_mut.get_height();

            while c > 0 || first {
                first = false;
                let sub_iters = vec![SubIter::First, SubIter::Second];

                for sub_iter in sub_iters {
                    if let SubIter::Second = sub_iter {
                        c = 0;
                    }

                    let mut m = BinImage::new(width, height, false);
                    let img_iter = image_mut.clone().into_iter();

                    for (x, y, _) in img_iter {
                        if image_mut.sub_iter(sub_iter.clone(), x, y)
                            && image_mut.get_value(x, y).unwrap()
                        {
                            let _result = m.set_value(x, y, true);
                            c += 1;
                        }
                    }

                    image_mut = image_mut - m;

                    if c == 0 {
                        break;
                    }
                }
            }
            Ok(image_mut.get_pixels().to_vec())
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

        let thinned = imgthin(img.get_pixels().to_vec()).unwrap();

        let expect_img =
            BinImage::try_from(PathBuf::from("./test_data/b_char_thinned.txt")).unwrap();
        assert_eq!(expect_img.get_pixels(), &thinned);
    }
}
