use std::cmp::max_by_key;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let sample_path = "./sample.txt";
    let excercice_path = "./excercice.txt";
    let content = ex_file(&excercice_path);
    let mut roads : HashMap<(String,String),i32> = HashMap::new();
    let mut locations:HashSet<String> = HashSet::new(); 

    for line in content.lines(){
        let line = line.unwrap();
        let (a,b,dist) = extract(line);
        roads.insert((a.clone(),b.clone()), dist);
        locations.insert(a);
        locations.insert(b);
    }
    let locations:Vec<String> = locations.into_iter().collect();
    let mut result: Vec<Vec<String>>= vec![];
    let mut current_route: Vec<String>= vec![];
    println!("{:?}",locations);
    println!("{}",locations.len());
    
    get_routes(locations,&mut current_route,&mut result);

    println!("{:?}",roads);
    println!("{}",result.len());
    // for route in result{
    //     println!("{:?}",route);
    //     let distance = get_total_distance(route, &roads);
    //     println!("{:?}",distance);
    // }
    let longest = result
    .iter()
    .map(|route| get_total_distance(route.to_owned(), &roads)).max().unwrap();

    let shortest = result.iter()
    .map(|route| get_total_distance(route.to_owned(), &roads))
    .min().unwrap();

    println!("the shortest route is : {shortest}");
    println!("the longest route is : {longest}")
}

fn get_routes(
    locations: Vec<String>,
    current_route:&mut Vec<String>,
    routes:&mut Vec<Vec<String>>,
){
    if locations.len() == 0 {
        routes.push(current_route.clone());
        return;
    }

    for i in 0..locations.len(){
        // println!("{}",i);
        // println!("{:?}",current_route);
        let chosen_step = locations[i].clone();
        
        let mut possible_routes = locations.clone();
        let mut next_route = current_route.clone();
        // exclude the i-th element within the possible locations vector
        possible_routes.remove(i);
        // continue in the i-th route
        next_route.push(chosen_step);

        get_routes( possible_routes, &mut next_route, routes)
    }
}

fn extract(line:String)->(String,String,i32) {
    let mut vals : Vec<&str> = line.split(' ').collect();
    
    let _a = vals[0].to_string();
    let _b = vals[2].to_string();
    let dist = vals[4].parse::<i32>().unwrap() ;
    return (_a,_b,dist)
}

pub fn ex_file(path:&str)-> BufReader<File>{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    return reader
}

fn get_total_distance(route:Vec<String>,roads:&HashMap<(String,String),i32>)->i32 {
    let mut total_distance = 0;
    for i in 0..route.len()-1{
        let source = route[i].clone();
        let destination = route[i+1].clone();
        let s_d = (source.clone(),destination.clone());
        let d_s = (destination.clone(),source.clone());

        let distance = match roads.get(&s_d) {
            Some(data) => data.clone(),
            None => match roads.get(&d_s) {
                Some(data) => data.clone(),
                None => 0
            }
        };
        total_distance+=distance
    }
    return total_distance
}
