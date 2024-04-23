use std::{collections::HashSet, fmt::Write};

use crate::utils;

pub fn run() {
    // 97..123 -> a..z
    // let mut content = "cqjxjnds".to_string();
    let mut content = "cqjxjndz".to_string();
    let restrictions:Vec<String> = vec!["i","o","l"].iter().map(|char|char.to_string()).collect();
    let mut valid = false;
    
    // while !valid{
        //     valid = valid_password(&content, &restrictions, 3);
        println!("{}",content);
        inc_phrase(&mut content);
        // println!("{}",content);
        // }
    let valid = valid_password(&content, &restrictions, 3);
    println!("{}",valid)
    
}

fn valid_password(s:&String,restrictions:&Vec<String>,window:usize)->bool {
    let Third_cond = char_pair(&s) ;
    let Second_cond = false_chars(s, &restrictions) ;
    let First_cond = increasing_straight(s,window) ;
    let valid = First_cond && Second_cond && Third_cond;
    println!("{}",s);
    // println!("First condition : {}", First_cond);
    // println!("Second condition : {}", Second_cond);
    // println!("Third condition : {}", Third_cond);
    return valid
}

// first condition
fn increasing_straight(s:&String,range:usize)->bool {
    let mut s_bytes = s.as_bytes();
    let window = match s_bytes.len()-range {
        0 => 1,
        _ => s_bytes.len()-range
    };
    for i in 0..window{
        let mut valid = 0; 
        for j in 0..range{
            
            let byte_n = s_bytes[i+j];
            let byte_np1 = s_bytes[i+j+1];

            if  byte_n + 1 == byte_np1  {
                valid += 1
            }
            // print!("{},{} ",byte_n + 1 ,byte_np1);
            // println!("{}",byte_n + 1  == byte_np1);
            if valid == range-1 {return true}
        }
    }
    false
}

//Second condition
fn false_chars(s:&String,restrictions:&Vec<String>)->bool {
    let mut legal_phrase = true;
    for char in restrictions{
        if s.contains(char){
            legal_phrase = false
        }
    }
    legal_phrase
}

//Third condition
fn char_pair(s:&String)->bool {
    let mut unique_chars = HashSet::new();
    for char in s.as_bytes(){
        unique_chars.insert(char.clone());
    }
    for c in unique_chars{
        let mut condition = false;
        let char = c as char;
        // println!("{}",c);
        let pair = char.to_string() + &char.to_string();
        if s.contains(&pair){
            return true
        }
    }
    false
}

fn inc_phrase(s:&mut String) {
    unsafe{
        let mut bytes = s.as_bytes_mut();
        bytes.reverse();
        let _bytes =&mut bytes[1..];
        // let mut carry = inc_char_bytes(&mut bytes[0], true);
        let mut carry =true;
        let mut carry = inc_char_bytes( bytes[0], carry);
        for chr in _bytes{
            carry = inc_char_bytes(chr, carry);
        }
        s.as_bytes_mut().reverse()
    }
}

fn inc_char(chr:&mut char,inc:bool,carry:bool)->bool {
    let mut C = chr.clone() as u8;
    let carry = carry as u8;

    if C > 122 {
        return true;
    }
    *chr = ((C + carry - 97)%26 + 97) as char;
    return false
}

fn inc_char_bytes(byte:&mut u8,carry:bool)->bool {
    let C = byte.clone();
    let carry = carry as u8;
    
    *byte = (C + carry - 97)%26 + 97;
    if C >= 123 {
        return true;
    }
    return false
}

fn cons_letter_bytes(byte:u8,carry:u8)->u8 {
    return (byte+ 1 + carry - 97)%26 + 97
}