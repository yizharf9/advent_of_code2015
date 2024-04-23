pub fn run() {
    let content = "1113222113".to_string();
    let n = 50;
    println!("the initial word is : {}" ,content);
    let mut result = lookNsay(content);
    // println!("{} : {}",0,result);
    for i in 1..n{
        result = lookNsay(result);
        // println!("{} : {}",i,result)
    }
    println!("the final result is {} with length {}",result,result.len())
    // lookNsay(content);
}

fn lookNsay(s:String)->String{
    let idxs = get_separating_idxs(&s);
    let mut result = "".to_string();
    let mut chars:Vec<&str>= s.split("").collect();
    chars.remove(0);
    chars.pop();
    // println!("{}",s);
    // println!("{:?}",chars);
    // println!("{:?}",idxs);

    for j in 0..idxs.len()-1{
        let idx =  idxs[j];
        let char = chars[idx];
        let mut iterations = idxs[j+1] - idxs[j];
        let iterations = iterations.to_string();
        
        let w = iterations + char ;
        result += &w;
    }
    return result
}

fn get_separating_idxs(s:&String)->Vec<usize> {
    let chars:Vec<&str> = s.split("").collect();
    let mut indexes:Vec<usize> = vec![];
    if chars.len() == 0 {
        return vec![0]
    }
    for i in 0..chars.len()-1{
        if chars[i] != chars[i+1]{
            indexes.push(i)
        } 
    }
    return indexes
}