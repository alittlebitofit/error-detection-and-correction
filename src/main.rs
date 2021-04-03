fn main() {
    hamify("10011010");

    find_and_fix_bad_bit("011100101110");
    
    find_and_fix_bad_bit("010101100011");
    find_and_fix_bad_bit("111110001100");
    find_and_fix_bad_bit("000010001010");
}

// calculates the number of parity bits using the formula: ceil(log(n) + 1)
fn calculate_number_of_parity_bits(input: i32) -> i32 {
    ((input as f64).log2() + 1 as f64).ceil() as i32
    // let res = ((input as f64).log2() + 1 as f64).ceil() as i32;
    // println!("res: {}", res);
    // res
}

// returns a codeword after hammifying it
fn hamify(input: &str) -> String {
    // total length of the output
    let total_length = input.len() as i32 + calculate_number_of_parity_bits(8);

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


fn find_and_fix_bad_bit(_bad_input: &str) -> String {

	// should decrement while decoding, otherwise the number of parity bits would have an extra bit
	let _number_of_parity_bits = calculate_number_of_parity_bits(_bad_input.len() as i32) - 1;

	// index of the bad bit
	let mut _bad_bit_parity_index = 0;

	// to skip/check these many times consecutively in every iteration
	let mut _skip_these_many_times = 0;

	// loop over number of parity bits
	for i in 0.._number_of_parity_bits {

		// skip parity number times
		_skip_these_many_times = 2_i32.pow(i as u32);

		// number of 1 bits for each parity position
		let mut _number_of_1_bits = 0;

		// number of skips so far
		let mut number_of_skips_so_far = 0;

		// check or skip these boolean check
		let mut check_or_skip = false;

		// inner loop to check number of parity bits for each parity position
		for (j_index, j_character) in _bad_input.chars().enumerate() {

			// this check is to start the skips and checks from the proper index
			if j_index as i32 + 1 >= _skip_these_many_times {

				// this is for skipping
				if check_or_skip {

					number_of_skips_so_far += 1;

					// if skipped preferred number of times, toggle and reset
					if number_of_skips_so_far == _skip_these_many_times {
						check_or_skip = false;
						number_of_skips_so_far = 0;
					}
				}

				// this is for checking
				else {
					number_of_skips_so_far += 1;

					// if checked preferred number of times, toggle and reset
					if number_of_skips_so_far == _skip_these_many_times {
						check_or_skip = true;
						number_of_skips_so_far = 0;
					}

					// if check bit is 1, increment number of 1 bits
					if j_character == '1' {
						_number_of_1_bits += 1;
					}
				}

			}
		}

		// before each iteration ends, increase the bad bit index
		if _number_of_1_bits % 2 != 0 {
			_bad_bit_parity_index += _skip_these_many_times;
		}

	}

	// decrement so that it doesn't go out of index
	_bad_bit_parity_index -= 1;


	// if bad bit does exist, then toggle its value and then return
	if _bad_bit_parity_index > -1 {

		// output
		let mut good_output = _bad_input.to_string();
		
		let bad_bit = good_output.remove(_bad_bit_parity_index as usize);

		if bad_bit == '0' {
			good_output.insert(_bad_bit_parity_index as usize, '1');
		} else {
			good_output.insert(_bad_bit_parity_index as usize, '0');
		}
		println!("this is the correct codeword: {}, from: {}", good_output, _bad_input);
		return good_output;
	}

	println!("nothing was wrong with: {}", _bad_input);
	// nothing was wrong in the input, no errors
	_bad_input.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(calculate_number_of_parity_bits(8), 4);
        assert_eq!(hamify("10011010"), "011100101010");
        assert_eq!(find_and_fix_bad_bit("011100101110"), "011100101010");
    }
}
