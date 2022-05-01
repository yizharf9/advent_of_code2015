use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self,prelude::*,BufReader};

pub fn run(){
    let mut map:HashMap<String,u16> = HashMap::new();
    
    while let None = map.get(&String::from("a")) {
        let content = ex_file();
        check_values(content, &mut map);
    }
    
    let A =  map.get(&String::from("a")).unwrap() ;
    println!("{A}");
}

fn check_values(content: BufReader<File>,map:&mut HashMap<String,u16>) {
    for res in content.lines(){
        let line = res.unwrap();
        let (kind,a,b,target,op) = CmdLine::new(line);

        //? all possible actions =>( "NOT" : !, "OR" : |, "AND" : &, "RSHIFT" : >>, "LSHIFT" : <<)
        if kind == "AND"||kind == "OR"{
            if let Some(val_a) = map.get(&a) {
                if let Some(val_b) = map.get(&b) {
                    let result = op(val_a.clone(),val_b.clone());
                    map.insert(target.clone(),result );
                }
                else if let Ok(B) = b.parse::<u16>(){
                        let result = op(val_a.clone(),B);
                        map.insert(target.clone(),result );
                }
            }else if let Some(val_b) = map.get(&b) {
                if let Ok(A) = a.parse::<u16>(){
                        let result = op(val_b.clone(),A);
                        map.insert(target.clone(),result );
                }
            }
            else if let Ok(A) = a.parse::<u16>(){
                if let Ok(B) = b.parse::<u16>(){
                    let result = op(A,B);
                    map.insert(target,result );
                }
            }
        }
        else if kind == "RSHIFT"||kind == "LSHIFT" {
            if let Some(val_a) = map.get(&a) {
                let B = b.parse::<u16>().unwrap();
                let result = op(val_a.clone(),B);
                map.insert(target.clone(),result );
            }
        }
        else if kind == "NOT" {
            if let Some(val_a) = map.get(&a) {
                let result = op(val_a.clone(),0);
                map.insert(target,result);
            }
        }
        else if let Some(A)= map.get(&a){
            map.insert(target,A.clone());
        }
        else if let Ok(A) = a.parse::<u16>(){
            map.insert(target,A);
        }
    }
}

fn ex_file()-> BufReader<File>{
    let file = File::open("./excercice.txt").unwrap();
    let reader = BufReader::new(file);
    return reader
}

pub struct CmdLine {
    a : String,
    b : String,
    target : String,
    op : Box<dyn Fn(u16,u16)->u16>
}

impl CmdLine  {
    pub fn new(line:String) -> (String,String,String,String,Box<dyn Fn(u16,u16)->u16>){
        let mut words:Vec<&str> = line.split(' ').collect();
        words.remove(words.len()-2);
        // println!("{words:?}");

        let op: Box<dyn Fn(u16,u16)-> u16>;
        let mut kind="";
        let mut a=String::from("");
        let mut b=String::from("");
        let target =  words[words.len()-1].to_owned() ;

        if words.len()==2 { //?in case of simple assigning statement => 234234 -> a
            (kind,op) = CmdLine::curr_action("");
            a = words[0].to_owned();
        }
        else if words.len()==3 { //?in case of any statement other than NOT
            (kind,op) = CmdLine::curr_action(words[0]);
            a = words[1].to_owned();
        }
        else { //?in case of NOT statement
            (kind,op) = CmdLine::curr_action(words[1]);
            a = words[0].to_owned();
            b = words[2].to_owned();
        }
        return (kind.to_owned(),a,b,target,op)
    }

    pub fn curr_action(action:&str) -> (&str,(Box<dyn Fn(u16,u16)-> u16>)) {
    if action == "OR" {
        return ("OR",Box::new(|a,b|{a|b}))
    }
    else if action == "AND" {
        return ("AND",Box::new(|a,b|{a&b}))
    }
    else if action == "LSHIFT" {
        return ("LSHIFT",Box::new(|a,b|{a<<b}))
    }
    else if action == "RSHIFT" {
        return ("RSHIFT",Box::new(|a,b|{a>>b}))
    }
    else if action == "NOT" {
        return ("NOT",Box::new(|a,_|{ !a}))
    }
    return("",Box::new(|a,_|{a}))
    }
}
