use std::io::{self,prelude::*,BufReader};
use std::fs::{self, File};


pub fn run() {
    let content = ex_file();
    // let nice:Vec<String> = vec![];
    let mut count = 0;

    for res in content.lines() {
        let line = res.unwrap();
        println!("\n{line}");

        // let test1=vowel_check(&line);
        // let test2 = Double_check(&line[..]);
        // let test3 = sequential_check(&line);

        let test1 = double_double_seq(&line);
        let test2 = sandwich_once(&line);

        if test1&&test2  //&&!test3 //!for solution of part 1
        {
            count+=1;
        }
    }
    println!("{count}")
}

pub fn vowel_check(line:&String)->bool {
    let vowels = ['a','e','i','o','u'];
    let mut pushed_vowels = vec![];
    for char in line.chars() {
        if let Some(vowel) = vowels.iter().find(|vowel|{vowel==&&char}) {
            pushed_vowels.push(vowel.clone());
            if pushed_vowels.len()==3{
                println!("contains 3 vowels!");
                return true
            } 
        }
    }
    false
}

pub fn Double_check(line:&str) ->bool {
    for (i,char) in line.chars().enumerate() {
        let not_end = i!=line.len()-1;
            if not_end {
            if char.to_owned() == line.as_bytes()[i+1] as char {
                println!("has double chars!");
                return true
            }
        }
    }
    false
}

pub fn sequential_check(line:&String)->bool {
    let strings = ["ab","cd","pq","xy"];
    for string in strings {
        if line.contains(string) {
            return true
        }
    }
    false
}

pub fn double_double_seq(line:&String)->bool {
    for (i,char) in line[..line.len()-3].chars().enumerate() {
        let next_char = line.as_bytes()[i+1] as char;

        let init = 
        // String::from(
            format!("{char}{next_char}");
        // );

        if line[i+2..].contains(&init) {

            println!("has double sequential letters!: {init}");
            return true
        }
    };
    false
}

pub fn sandwich_once(line:&String)->bool {
    let mut count = 0;
    for (i,char) in line[..line.len()-2].chars().enumerate() {
        if line.as_bytes()[i+2]as char==char {
            let sandwich = &line[i..i+3];
            println!("sandwich : {sandwich}");
            return true
        }
    }
    false
}

fn ex_file()->BufReader<File> {
    let _flie = File::open("./excercice.txt").unwrap();
    let reader = BufReader::new(_flie);
    return reader
}


// pub fn double_double_seq(line:&String)->bool {
//     let letters = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0'];
//     for (i,char) in line[..line.len()-3].chars().enumerate() {
//         let abc_i_char = letters
//         .iter()
//         .position(|x|*x==char)
//         .unwrap();
//         let next_char = line.as_bytes()[i+1] as char;
//         let seq_abc= letters[abc_i_char+1];     
//         if seq_abc==next_char //for seuential eq
//         &&line[i..i+2]==line[i+2..i+4] //for second sequential eq
//         {
//             println!("has double sequential letters!");
//             return true
//         }
//     };
//     false
// }
