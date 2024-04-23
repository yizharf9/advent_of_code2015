use std::fs::{ File};
use std::io::{BufReader, BufRead};
use hex;

//? upper case ascii - A(65)..Z(90)
//? lower case ascii - a(97)..z(122)

//                              \"  ,  \x , \\     
pub fn run() {
    let specials = ["\\\"","\\x","\\\\",""];
    let content = ex_file();
    let mut total_length = 0;
    let mut total_stored = 0;
    
    for res in content.lines() {
        let line = res.unwrap();
        println!("{line}");
        let length = line.len();
        let mut stored = line.len()-2;

        for (i,char) in line[..line.len()-2].chars().enumerate(){
            if &line[i..i+2] == "\\\"" {
                println!("=>  \\\"");
                stored -=1;
            }
            if &line[i..i+2] == "\\\\" {
                println!("=>  \\\\");
                stored -=1;
            }
            if &line[i..i+2] == "\"\"" {
                println!("=>  \"\"");
                stored -=1;
            }
            if &line[i..i+2] == "\\x"  {
                let full_phrase = &line[i..i+4].to_lowercase().to_owned();
                
                match hex::decode(&line[i+2..i+4]){
                    Ok(hex)=>{
                        let hex = (hex[0] as char);
                        if !hex.is_whitespace() {
                            println!("phrase : {full_phrase}");
                            println!("hex : {hex:?}");
                            stored -=3;
                        }
                    }
                    Err(_)=> println!("not a hex value")
                }
            }
        }
        println!("length: {length}");
        total_length+=length;
        println!("stored: {stored}");
        total_stored+=stored;
        println!("")
    }
    println!("length: {total_length}");
    println!("stored: {total_stored}");
    println!("reduction: {}",total_length-&total_stored);
}

fn eval_line(line : String)-> u8 {
    let specials = ["\\\"","\\x","\\\\","\"\""];
    let mut total = line.len();
    let mut line = line[1..line.len()-1].to_owned();
    let mut stored = line.len();

    if line.contains("\\\"") {
        let str = "\\\"";
        println!("=> \\\"");
        total += 1
    }
    else if line.contains("\\x") {
        println!("=> \\x");
        total +=1
    }
    else if line.contains("\\\\") {
        println!("=> \\\\");
        total += 1
    }
    else if line.contains("\"\"") {
        println!("=>\"\"");
    }
    else {
        println!("");
        // println!("=> ");
    }
    println!("stored in memory :{stored}");
    println!("actual len :{total}");
    1
}

fn ex_file()-> BufReader<File>{
    let file = File::open("./excercice.txt").unwrap();
    let reader = BufReader::new(file);
    return reader
}
