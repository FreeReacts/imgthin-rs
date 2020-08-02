use crate::bin_image::*;
use crate::common::*;
use std::convert::TryFrom;
use std::io::Error;
#[cfg(test)]
use std::path::PathBuf;

fn sub_iter(
    mode: &SubIter,
    p2: bool,
    p3: bool,
    p4: bool,
    p5: bool,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: bool,
) -> bool {
    let (a_p, b_p) = calculate_ap_and_bp(p2, p3, p4, p5, p6, p7, p8, p9);

    let a = 2 <= b_p && b_p <= 7;

    match mode {
        SubIter::First => {
            if a_p == 1 {
                a && !(p2 && p4 && p6) && !(p4 && p6 && p8)
            } else if a_p == 2 {
                a && ((p2 && p4) && !(p6 || p7 || p8)) || ((p4 && p6) && !(p2 || p8 || p9))
            } else {
                false
            }
        }
        SubIter::Second => {
            if a_p == 1 {
                a && !(p2 && p4 && p8) && !(p2 && p6 && p8)
            } else if a_p == 2 {
                a && ((p2 && p8) && !(p4 || p5 || p6)) || ((p6 && p8) && !(p2 || p3 || p4))
            } else {
                false
            }
        }
    }
}

fn make_table(sub_iter_type: SubIter) -> Vec<Vec<bool>> {
    // Making the mapping table
    let mut table: Vec<Vec<bool>> = vec![];

    for i in 0..16 {
        let i_bin = format!("{:04b}", i);
        let mut row: Vec<bool> = vec![];
        for j in 0..16 {
            let j_bin = format!("{:04b}", j);
            let i_bin = i_bin.as_str();
            let bin: String = j_bin.clone() + i_bin;

            let neighbors: Vec<bool> = bin
                .chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!("Can not parse char to bool"),
                })
                .collect();

            let sub_iter_result = !sub_iter(
                &sub_iter_type,
                neighbors[3],
                neighbors[2],
                neighbors[1],
                neighbors[0],
                neighbors[7],
                neighbors[6],
                neighbors[5],
                neighbors[4],
            );

            row.push(sub_iter_result);
        }

        table.push(row);
    }

    table
}

fn bin_to_dec(bin: [bool; 4]) -> usize {
    let val: Vec<usize> = bin
        .iter()
        .map(|v| if *v { 1 as usize } else { 0 as usize })
        .collect();

    (val[3] * 1) + (val[2] * 2) + (val[1] * 4) + (val[0] * 8)
}

pub fn imgthin(pixels: Vec<Vec<bool>>) -> Result<Vec<Vec<bool>>, Error> {
    let k_t = BinImage::try_from(pixels)?;
    let mut s_t = k_t.clone();

    let mut k_t_iter = k_t.into_iter();

    let sub_1_table = make_table(SubIter::First);
    let sub_2_table = make_table(SubIter::Second);

    let mut k = SubIter::First;
    let mut s = SubIter::Second;
    let mut flag = true;

    while flag {
        flag = false;
        while let Some((x, y, val)) = k_t_iter.next() {
            if val {
                let neighbors = s_t.get_neighbors(x, y);
                
                let j = bin_to_dec([neighbors.1, neighbors.2, neighbors.3, neighbors.4]);
                let i = bin_to_dec([neighbors.5, neighbors.6, neighbors.7, neighbors.8]);
       
                let d_out = match k {
                    SubIter::First => sub_1_table.get(i).unwrap().get(j).unwrap(),
                    SubIter::Second => sub_2_table.get(i).unwrap().get(j).unwrap(),
                };

                s_t.set_value(x, y, d_out.to_owned())?;

                if !d_out {
                    flag = true;
                }
            } else {
                s_t.set_value(x, y, false)?;
            }
        }
        let t = k;
        k = s;
        s = t;
    }

    Ok(s_t.get_pixels().to_vec())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subiter() {
        assert_eq!(
            sub_iter(
                &SubIter::First,
                true,
                true,
                true,
                true,
                true,
                true,
                true,
                true
            ),
            false
        );

        assert_eq!(
            sub_iter(
                &SubIter::First,
                true,
                true,
                false,
                true,
                true,
                true,
                true,
                true
            ),
            true
        );

        assert_eq!(
            sub_iter(
                &SubIter::First,
                true,
                false,
                true,
                true,
                false,
                false,
                false,
                false
            ),
            true
        );

        assert_eq!(
            sub_iter(
                &SubIter::First,
                true,
                true,
                true,
                true,
                false,
                false,
                false,
                true
            ),
            true
        );

        assert_eq!(
            sub_iter(
                &SubIter::First,
                true,
                false,
                true,
                false,
                false,
                false,
                false,
                false
            ),
            true
        );

        assert_eq!(
            sub_iter(
                &SubIter::First,
                false,
                false,
                true,
                false,
                true,
                false,
                false,
                false
            ),
            true
        );

        assert_eq!(
            sub_iter(
                &SubIter::First,
                false,
                true,
                true,
                true,
                true,
                true,
                false,
                false
            ),
            true
        );

        assert_eq!(
            sub_iter(
                &SubIter::Second,
                false,
                false,
                false,
                false,
                true,
                false,
                true,
                false
            ),
            true
        );

        assert_eq!(
            sub_iter(
                &SubIter::Second,
                false,
                false,
                false,
                true,
                true,
                true,
                true,
                true
            ),
            true
        );
    }

    #[test]
    fn test_direct_computation_vs_table() {
        let table_2 = make_table(SubIter::Second);

        let j = bin_to_dec([false, true, true, true]);
        let i = bin_to_dec([true, true, false, false]);

        let table_val = table_2.get(i).unwrap().get(j).unwrap();

        assert_eq!(
            sub_iter(
                &SubIter::First,
                false,
                true,
                true,
                true,
                true,
                true,
                false,
                false
            ),
            table_val.to_owned()
        );
    }

    #[test]
    fn test_make_table() {
        let table_1 = make_table(SubIter::First);
        let first_sub = BinImage::try_from(table_1).unwrap();
        let first_sub_expect =
            BinImage::try_from(PathBuf::from("./test_data/mapping_table_1_expect.txt")).unwrap();

        assert_eq!(first_sub.get_pixels(), first_sub_expect.get_pixels());

        let table_2 = make_table(SubIter::Second);
        let second_sub = BinImage::try_from(table_2).unwrap();
        let second_sub_expect =
            BinImage::try_from(PathBuf::from("./test_data/mapping_table_2_expect.txt")).unwrap();

        assert_eq!(second_sub.get_pixels(), second_sub_expect.get_pixels());
    }

    #[test]
    fn test_bin_to_dec() {
        assert_eq!(bin_to_dec([true, false, false, false]), 8);
        assert_eq!(bin_to_dec([true, true, false, false]), 12);
        assert_eq!(bin_to_dec([true, true, true, false]), 14);
    }

    #[test]
    fn test_char_b() {
        let img = BinImage::try_from(PathBuf::from("./test_data/b_char.txt")).unwrap();

        let thinned = imgthin(img.get_pixels().to_vec()).unwrap();

        let thinned_img = BinImage::try_from(thinned).unwrap();
        println!("{}", thinned_img);

        let expect_img =
            BinImage::try_from(PathBuf::from("./test_data/b_char_thinned.txt")).unwrap();
        // assert_eq!(expect_img.get_pixels().to_vec(), thinned);
    }
}
