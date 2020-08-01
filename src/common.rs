// Common methods to improved and default algorithm
#[derive(Clone, Debug)]
pub enum SubIter {
    First,
    Second,
}

pub fn calculate_ap_and_bp(
    p2: bool,
    p3: bool,
    p4: bool,
    p5: bool,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: bool,
) -> (usize, usize) {
    let mut arr = vec!(p2, p3, p4, p5, p6, p7, p8, p9);
    let b_p = arr.iter().map(|p| if *p { 1 } else { 0 }).sum::<usize>();

    arr.push(p2);

    let mut a_p = 0;
    let mut prev_p = true;

    for p in arr.iter() {
        if !prev_p && p.to_owned() {
            a_p += 1;
        }
        prev_p = p.to_owned();
    }

    
    (a_p, b_p)
}


