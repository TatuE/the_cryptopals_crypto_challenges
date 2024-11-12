mod utils;

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

fn display_processing(){
    utils::print_new_lines(2);
    utils::print_symbol(PRINT_L,'*');
    println!("Processing....");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);
    utils::sleep_for_seconds(1);
    println!("Processing complete!");
}

fn challenge_1(){
    let mut option_selected_n: u32;
    'outer: loop {
        let mut challenge_hex_s: String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        utils::clear_c();
        utils::print_new_lines(2);
        utils::print_with_spaces(PRINT_L, "Challenge 1 : Convert hex to base64.");
        utils::print_symbol(PRINT_L,'*');
        utils::print_new_lines(2);
        println!("Convert a hexadecimal string to base64.");
        println!("Would you like use the default hexadecimal value or input you're own? ");
        println!("Option 1: Use the default hexadecimal value : {}.", challenge_hex_s);
        println!("Option 2: Input you're own hexadecimal value.");
        println!("Option 0: Return to main menu.");
        utils::print_new_lines(1);

        option_selected_n = utils::get_numeric_input();

        if check_numeric_option_selectd(option_selected_n, 0, 2){
            if option_selected_n !=0 {
                if option_selected_n == 1 {
                    println!("You have chosen to use the default hexadecimal value : {}.", challenge_hex_s);

                }else if option_selected_n == 2 {
                    println!("You have chosen to use you're own hexadecimal value.");
                    challenge_hex_s = utils::get_string_input();
                }

                let s = challenge_hex_s.clone();

                let b64:String = base64::encode(hex::decode(challenge_hex_s).unwrap());

                display_processing();

                println!("The used the hexadecimal value :  {}", s);
                println!("The base64 convertion is : {}", b64);
                utils::sleep_for_seconds(5);
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
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_with_spaces(PRINT_L, "Challenge 2 = Fixed XOR.");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);    
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