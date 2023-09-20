fn main() {
    let ages: Vec<usize> = vec![17, 4, 2, 23, 42, 12, 43, 20, 120, 1209, 12, 3, 5, 1, 5, 11, 1, 2];

    let sorted_ages = bubble_sort(ages);

    println!("{:?}", sorted_ages);
}

fn bubble_sort(mut unsorted_numbers: Vec<usize>) -> Vec<usize> {
    while is_sorted(&unsorted_numbers) == false {
        for idx in 0..unsorted_numbers.len() - 1 {
            let maybe_current_number = unsorted_numbers.get(idx);
            let maybe_next_number = unsorted_numbers.get(idx + 1);

            match (maybe_current_number, maybe_next_number) {
                (Some(current_number), Some(next_number)) => {
                    if current_number > next_number {
                        unsorted_numbers.swap(idx, idx + 1);
                    }
                }

                _ => {}
            }
        }
    }

    unsorted_numbers
}

fn is_sorted(maybe_sorted_numbers: &Vec<usize>) -> bool {
    let mut sorted = true;

    for (idx, current_number) in maybe_sorted_numbers.iter().enumerate() {
        let maybe_next_number = maybe_sorted_numbers.get(idx + 1);

        match maybe_next_number {
            Some(next_number) => {
                if next_number < current_number {
                    sorted = false;
                    break;
                }
            }

            None => ()
        }
    }

    sorted
}

