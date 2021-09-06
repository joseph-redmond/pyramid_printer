
fn main() {
    print_half_pyramid(8, true);
    print_half_pyramid(8, false);
    print_full_pyramid(8);
}

fn print_half_pyramid(height: i64, is_reversed: bool){
    let mut count: i64 = 1;

    loop {
        if is_reversed != true {
            let hashes: String = get_string_of_hashes(count);

            println!("{}", hashes);
        } else{
            let hashes: String = get_string_of_hashes(count);
            let spaces: String = get_string_of_spaces(height - count);

            println!("{}{}", spaces, hashes);
        }

        count = count + 1;

        if count > height{
            break;
        }
    }
}

fn print_full_pyramid(height: i64){
    let mut count: i64 = 1;

    loop {
        let hashes_right_side: String = get_string_of_hashes(count);
        let hashes_left_side: String = get_string_of_hashes(count);
        let spaces: String = get_string_of_spaces(height - count);

        println!("{}{}{}{}", spaces, hashes_left_side, "  ", hashes_right_side);

        count = count + 1;

        if count > height{
            break;
        }
    }
}

fn get_string_of_hashes(num: i64) -> String{
    let mut string_of_hashes: String = "".to_owned();

    loop {
        let temp_string: String = "#".to_owned();

        string_of_hashes = format!("{}{}", string_of_hashes, temp_string);

        if string_of_hashes.chars().count() == num as usize {
            break;
        }
    }
    string_of_hashes.to_string()
}

fn get_string_of_spaces(num: i64) -> String {
    let mut string_of_spaces: String = "".to_owned();

    loop {
        let temp_string: String = " ".to_owned();

        string_of_spaces = format!("{}{}", string_of_spaces, temp_string);

        if string_of_spaces.chars().count() > num as usize {
            break;
        }
    }
    string_of_spaces.to_string()
}
