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

pub fn sleep_for_seconds(seconds: u64){
    let times_s: u64 = seconds * 1000;
    thread::sleep(Duration::from_millis(times_s));
}

pub fn get_numeric_input() -> u32 {
    let n: u32;
    let mut input:String;

    print!("Please enter a number: ");

    loop{
        std::io::stdout().flush().unwrap();
        input = String::new();
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
    let mut input: String;

    print!("Please enter a text value: ");

    loop{
        std::io::stdout().flush().unwrap();
        input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input:&str = input.trim();

        if !input.is_empty() {
            break;
        }else{
            println!("No input received , please input a text value!");
            sleep_for_seconds(2)
        } 
    }
    input
}