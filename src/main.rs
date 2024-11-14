mod utils;
mod crypto_tools;

const PRINT_L: u32 = 50;

fn main() {
    print_program_option(true);

    loop {

        let option_selected_n: u32 = utils::get_numeric_input();

        if check_numeric_option_selectd(option_selected_n, 0, 4) {

            match option_selected_n {
                0 => exit_program(),
                1 => challenge_1(),
                2 => challenge_2(),
                3 => challenge_3(),
                4 => challenge_4(),
                _ => continue,
            }
        }

        print_program_option(true);
    }
}

fn print_program_header(){
    utils::print_new_lines(2);
    utils::print_symbol(PRINT_L,'*');
    utils::print_with_spaces(PRINT_L, "Cryptopals: Challenge Set 1");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);
}

fn print_program_option(clear_screen_b:bool){
    if clear_screen_b{
        utils::clear_c();    
    }
    print_program_header();
    utils::print_new_lines(2);
    utils::print_with_spaces(PRINT_L, "Which challenge would you like to run?");
    utils::print_new_lines(1);
    println!("Option 1: Challenge 1 = Convert hex to base64.");
    println!("Option 2: Challenge 2 = Fixed XOR.");
    println!("Option 3: Challenge 3 = Single-byte XOR cipher.");
    println!("Option 4: Challenge 4 = Detect single-character XOR.");
    println!("Option 0: Exit program.");
    utils::print_new_lines(1);
}

fn check_numeric_option_selectd(option_n: u32, min_value: u32, max_value: u32) -> bool {
    let c: bool = option_n >= min_value && option_n <= max_value;
    if c {
        println!("You have selected option : {option_n}");
        utils::sleep_for_seconds(1);
    }else{
        utils::print_new_lines(2);
        println!("You have select : {option_n}, which is not an option in this program. Please select again!");
        utils::sleep_for_seconds(2);
        utils::clear_c(); 
    }
    c
}

fn challenge_1(){
    let mut option_selected_n: u32;
    let mut own_hex: String;
    'outer: loop {
        let mut challenge_hex_s:&str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        utils::clear_c();
        utils::print_new_lines(2);
        utils::print_with_spaces(PRINT_L, "Challenge 1 : Convert hex to base64.");
        utils::print_symbol(PRINT_L,'*');
        utils::print_new_lines(2);
        println!("Convert a hexadecimal string to base64.");
        println!("Would you like use the default hexadecimal value in the challenge or input you're own? ");
        println!("Option 1: Use the default hexadecimal value : {}.", challenge_hex_s);
        println!("Option 2: Input you're own hexadecimal value.");
        println!("Option 0: Return to main menu.");
        utils::print_new_lines(1);

        option_selected_n = utils::get_numeric_input();

        if check_numeric_option_selectd(option_selected_n, 0, 2){
            if option_selected_n !=0 {
                if option_selected_n == 1 {
                    println!("You have chosen to use the default hexadecimal value : {} .", challenge_hex_s);

                }else if option_selected_n == 2 {
                    println!("You have chosen to use you're own hexadecimal value.");
                    own_hex = utils::get_string_input();
                    challenge_hex_s = own_hex.as_str();
                }

                let b64:String = crypto_tools::convert_hex_to_base64(challenge_hex_s);

                utils::display_processing(5, PRINT_L);
                
                utils::print_new_lines(2);
                println!("The used the hexadecimal value : {}", challenge_hex_s);
                println!("The base64 convertion is : {}", b64);
                utils::print_new_lines(2);
                utils::get_any_input("Press enter to end reviewing.");

                'inner: loop {
                    utils::clear_c();
                    utils::print_new_lines(2);
                    println!("Would you like to do this challenge again, or return to the main menu?");
                    println!("Option 1: Do the challenge again.");
                    println!("Option 0: Return to main menu.");
                    utils::print_new_lines(1);

                    option_selected_n = utils::get_numeric_input();

                    if check_numeric_option_selectd(option_selected_n, 0, 2){
                        if option_selected_n == 1 {
                            println!("Wonderful!! Let's do this chalnge again!");
                            utils::sleep_for_seconds(1);
                            break 'inner; 

                        }else{
                            println!("Thank you! Returning to main menu.");
                            utils::sleep_for_seconds(1);
                            break 'outer; 
                        }

                    }
                }
            }else{
                println!("Returning to main menu.");
                utils::sleep_for_seconds(1);
                break 'outer;
            }
        }
    }
}

fn challenge_2(){
    let mut option_selected_n: u32;
    let mut own_hex_1: String;
    let mut own_hex_2: String;

    'outer: loop {
        let mut challenge_hex_s_1:&str = "1c0111001f010100061a024b53535009181c";
        let mut challenge_hex_s_2:&str = "686974207468652062756c6c277320657965";
        utils::clear_c();
        utils::print_new_lines(2);
        utils::print_with_spaces(PRINT_L, "Challenge 2 = Fixed XOR.");
        utils::print_symbol(PRINT_L,'*');
        utils::print_new_lines(2);

        println!("Fixed XOR.");
        println!("Would you like use the default hexadecimal values in the challenge or input you're own? ");
        println!("Option 1: Use the default hexadecimal values:");
        println!("      Hexadecimal 1 value : {} .", challenge_hex_s_1);
        println!("      Hexadecimal 2 value : {} .", challenge_hex_s_2);
        println!("Option 2: Input you're own hexadecimal value.");
        println!("Option 0: Return to main menu.");
        utils::print_new_lines(1);

        option_selected_n = utils::get_numeric_input();
    }
}

fn challenge_3(){
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_with_spaces(PRINT_L, "Challenge 3 = Single-byte XOR cipher.");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);
}

fn challenge_4(){
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_with_spaces(PRINT_L, "Challenge 4 = Detect single-character XOR.");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);    
}

fn exit_program(){
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_symbol(PRINT_L,'*');
    utils::print_with_spaces(PRINT_L, "Thank you for using this program!");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);
    std::process::exit(0);
}