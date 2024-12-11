//////////////////////////////////////////////////////////////
/// A selection of useful tools for every day applications ///
//////////////////////////////////////////////////////////////

use std::string::String;
use std::str;
use std::{thread, time::Duration};
use std::io::Write;

pub fn print_symbol(amount: u32, symbol: char){
    for i in 0..amount{
        if i == (amount-1){
            println!("{}", symbol);
        }else{
            print!("{}", symbol);
        }
    }  
}

//////////////////////////////////////////////////////////////
///               Terminal ("GUI") functions               ///
//////////////////////////////////////////////////////////////

pub fn print_with_spaces(print_length:u32, text_string:&str){
    let s:String  = text_string.trim().to_owned();
    let l:u32 = s.len().try_into().unwrap(); 

    if print_length <= l {
        println!("{}", text_string)
    }else{
        let n:f32 = ((print_length - l)/2) as f32;
        let start_n:u32 = n.floor() as u32;
        let end_n:u32 = n.ceil() as u32;
        let mut s2 = String::new();

        for _ in 0..start_n{
            s2.push(' ');    
        }

        s2.push_str(&s);

        for _ in 0..end_n{
            s2.push(' ');    
        }
        println!("{}", s2)
    }

}

pub fn print_new_lines(amount: u32){
    for _ in 0..amount{
        println!("");
    }
}

pub fn clear_c(){
    clearscreen::clear().unwrap();
}

pub fn display_processing(time_in_seconds:u64,print_length:u32){
    let mut t_s = time_in_seconds*900;
    let loop_time_millis: u64 = 100;
    let mut print_dot_count:u32 = 0;
    let dots_max_number:u32 = 4;
    let mut processing_info_s: String;
    let mut processing_end_s: String;

    let n:f32 = ((print_length)/3) as f32;
    let start_n:u32 = n.floor() as u32;

    processing_end_s = "Processing complete!".to_string();

    for _ in 0..start_n{
        let s:String = " ".to_string();
        processing_end_s = s + &processing_end_s;   
    }


    while t_s > 0 {
        clear_c();
        processing_info_s = "Processing".to_string();

        for _ in 0..print_dot_count{
            processing_info_s.push('.');    
        }

        for _ in 0..start_n{
            let s:String = " ".to_string();
            processing_info_s = s + &processing_info_s;  
        }

        print_new_lines(2);
        print_symbol(print_length, '*');
        print_new_lines(1);
        println!("{processing_info_s}");
        print_new_lines(1);
        print_symbol(print_length, '*');

        thread::sleep(Duration::from_millis(loop_time_millis));

        if print_dot_count >= dots_max_number {
            print_dot_count = 0;
        }else{
            print_dot_count += 1;    
        } 

        t_s = t_s-loop_time_millis;
    }

    clear_c();
    print_new_lines(2);
    print_symbol(print_length, '*');
    print_new_lines(1);
    println!("{processing_end_s}");
    print_new_lines(1);
    print_symbol(print_length, '*');

    thread::sleep(Duration::from_millis(time_in_seconds*100));
    clear_c();
}

pub fn display_under_construction(print_length:u32){
    print_new_lines(2);
    print_symbol(print_length, '*');
    print_new_lines(1);
    print_with_spaces(print_length, "This task is still under construction!");
    print_new_lines(1);
    print_symbol(print_length, '*');
    sleep_for_seconds(2)
}
//////////////////////////////////////////////////////////////
///               System functions                         ///
//////////////////////////////////////////////////////////////

pub fn sleep_for_seconds(time_in_seconds: u64){
    let times_s: u64 = time_in_seconds * 1000;
    thread::sleep(Duration::from_millis(times_s));
}

//////////////////////////////////////////////////////////////
///               Input handling functions                 ///
//////////////////////////////////////////////////////////////

pub fn get_numeric_input() -> u32 {
    let n: u32;

    print!("Please enter a number: ");

    loop{
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input:&str = input.trim();

        let mut is_input_numeric: bool = true;

        for c in input.chars(){
            if !c.is_numeric(){
                is_input_numeric = false;
            }
        }

        if is_input_numeric {
            n = match input.parse() {
                Ok(num) => num,
                Err(_) => continue,
            }; 
            break;
        }else{
            println!("Input is not numeric, please input a numeric value!");
            sleep_for_seconds(2)
        } 
    }
    n 
}

pub fn get_string_input() -> String {
    print!("Please enter a text value: ");
    let mut input = String::new();

    loop{
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input:&str = input.trim();

        if !input.is_empty() {
            break;
        }else{
            println!("No input received , please input a text value!");
            sleep_for_seconds(2)
        } 
    }
    input.trim().to_owned()
}

pub fn get_any_input(text_string_for_info:&str){
    print!("{text_string_for_info}");
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
}

