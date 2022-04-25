use md5::*;

pub fn run(){

    let astr = "bgvyzdsv";
    let mut count = -1;
    
    loop {
        count+=1;
        let mut dig = format!("{:x}",compute(format!("{}{}",astr,count)));
        // println!("dig : {}, count: {}",dig,count);
        if &dig[0..6]=="000000" {
            println!("dig : {}, count: {}",dig,count);
            break;
        }
    }
}

pub fn test(){
    let mut arr = [9,45,0,5,3,6,4];

    let mut a = &arr[3];
    println!("{a}");
    a = &1000;
    println!("{a}");
}

