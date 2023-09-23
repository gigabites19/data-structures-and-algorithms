fn main() {
    let haystack: Vec<isize> = vec![-1, -2, 0, 1, 2, 3, 4, 5, 13, 15, 21, 101];
    let needle: isize = 15; 
    // bug: can't find negative values. e.g.: let needle: isize = -1;

    let maybe_min = haystack.get(0);
    let maybe_max = haystack.get(haystack.len() - 1);

    match (maybe_min, maybe_max) {
        (Some(min), Some(max)) => {
            if needle < *min || needle > *max {
                println!("Needle is out of bounds of the vector.");
            } else {
                println!("Looking for index in haystack for value {needle}");

                let maybe_needle_idx = binary_search(&needle, &haystack, 0, haystack.len() - 1);

                match maybe_needle_idx {
                    Ok(needle_idx) => {
                        println!("\nfound index! {needle_idx}");
                    }

                    Err(_) => {
                        println!("\nCould not find value {needle} in haystack.");
                    }
                }
            }
        }

        _ => {}
    }

}

fn binary_search(needle: &isize, haystack: &Vec<isize>, mut from: usize, mut to: usize) -> Result<usize, isize> {
    let mid_idx = get_mid(&from, &to);
    let maybe_mid_number = haystack.get(mid_idx);

    match maybe_mid_number {
        Some(possibly_needle) => {
            if to - from == 0 && possibly_needle != needle {
                return Result::Err(-1);
            }

            if possibly_needle == needle {
                Result::Ok(mid_idx)
            } else if possibly_needle < needle {
                from = mid_idx + 1;
                return binary_search(needle, haystack, from, to);
            } else if possibly_needle > needle {
                to = mid_idx;
                return binary_search(needle, haystack, from, to);
            } else {
                return Result::Err(-1);
            }
        }

        None => Result::Err(-1)
    }
}

fn get_mid(one: &usize, two: &usize) -> usize {
    ((one + two) as f32 / 2.0).floor() as usize
}

