fn main() {
    hamify("10011010");
    // result -> "010101100011"
    find_and_fix_bad_bit();
}

// calculates the number of parity bits using the formula: ceil(log(n) + 1)
fn calculate_bits(input: i32) -> i32 {
    ((input as f64).log2() + 1 as f64).ceil() as i32
    // let res = ((input as f64).log2() + 1 as f64).ceil() as i32;
    // println!("res: {}", res);
    // res
}

// TODO 1: convert 10011010 to _ _ 1 _ 0 0 1 _ 1 0 1 0 - DONE
// TODO 2: check parity bit position and replace it with either 1 or 0 - DONE
// TODO 3: find and fix a bad bit

fn find_and_fix_bad_bit() {
	// println!("todo");
}

// returns a codeword after hammifying it
fn hamify(input: &str) -> String {
    // total length of the output
    let total_length = input.len() as i32 + calculate_bits(8);

    // intermediate result
    let mut res = String::new();

    // input made iterable to insert in result
    let mut input_iterable = input.as_bytes().iter();

    // loop over input to make an intermediate result
    for i in 1..=total_length {
        if i & i - 1 == 0 {
            // if index is power of 2, insert underscore
            res.push('_');
        } else {
            // else insert next character from input
            let temp = input_iterable.next();
            res.push(*temp.unwrap() as char);
        }
    }

    // this is to check for number of skips based of parity position
    let mut skip_these_many_times = 0;

    // index at which underscores must be replaced in result
    let mut index = 0;

    // final result to return
    let mut result = res.clone();

    // loop over the intermediate sequence
    for i in res.chars() {
        skip_these_many_times += 1;

        // if it is the position of a parity bit
        if i == '_' {
            // to skip or not to skip
            let mut check_or_skip = false;

            // decides the parity bit's value
            let mut number_of_1_bits = 0;

            // skip these many times over a second loop of intermediate result
            let mut number_of_skips_so_far = 0;

            // loop over intermediate result again to find out number of 1 bits
            for (l_index, l_char) in res.as_bytes().iter().enumerate() {
                // first set the position at which this inner loop has to start
                if l_index + 1 >= skip_these_many_times {
                    // helper function
                    use std::str;

                    // if it has to be skipped
                    if check_or_skip {
                        // first increment the skip value
                        number_of_skips_so_far += 1;

                        // check if the number of skips match, if they do then toggle the skip's boolean value to false and reset skip value
                        if number_of_skips_so_far == skip_these_many_times {
                            // toggle
                            check_or_skip = false;

                            // reset
                            number_of_skips_so_far = 0;
                        }
                    }
                    // if it were to not be skipped
                    else {
                        number_of_skips_so_far += 1;

                        // increment the variable if the unskipped value is 1
                        if str::from_utf8(&[*l_char]).unwrap() == "1" {
                            number_of_1_bits += 1;
                        }

                        // toggle and reset
                        if number_of_skips_so_far == skip_these_many_times {
                            // toggle
                            check_or_skip = true;

                            // reset
                            number_of_skips_so_far = 0;
                        }
                    }
                }
            }

            // if parity is even, replace the underscore with 0
            if number_of_1_bits % 2 == 0 {
                result.remove(index);
                result.insert(index, '0');
            } else {
                // otherwise with 1
                result.remove(index);
                result.insert(index, '1');
            }
        }
        index += 1;
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(calculate_bits(8), 4);
        assert_eq!(hamify("10011010"), "011100101010");
    }
}
