//?  two d arry\vector example :
//  let two_d = vec!([false;1000];1000); 

use std::fs;
#[allow(unused_variables)]
#[allow(non_camel_case_types)]
pub fn run() {
    let mut grid = vec!([0;1000];1000); 

    let content = ex_file();
    let mut lines = content.split("\r").enumerate();
    // let curr_lines = &lines[0..10];
    for (i,line) in lines {

        //? three options of possible line structure :
        //1: turn on 489,959 through 759,964
        //2: turn off 427,423 through 929,502
        //3: toggle 756,965 through 812,992

        // println!("{line}");
        
        let mut start = 0;

        if line.contains("turn on") {
            // println!("turning on...");
            start = 8
        }
        else if line.contains("turn off") {
            // println!("turning off...");
            start = 9
        }
        else if line.contains("toggle") {
            // println!("toggling...");
            start = 8
        }


        //extract the coordinates of point (a)
        let end = line.find("through").unwrap();
        let mut a = &line[start..end];
        a = a.trim();
        let comma_i = a.find(',').unwrap();

        let [x_a,y_a] = [
            &a[..comma_i].trim()
                .parse::<usize>()
                .unwrap(),

            &a[comma_i+1..].trim()
                .parse::<usize>()
                .unwrap()
        ];
        // 

        //extract the coordinates of point (b)
        let index = line.find("through").unwrap()+"through".len();
        let mut b = &line[index..];                         
        b = b.trim();
        let comma_i = b.find(',').unwrap();
        
        let [x_b,y_b] = [
            &b[..comma_i].trim()
                .parse::<usize>()
                .unwrap(), 

            &b[comma_i+1..].trim()
                .parse::<usize>()
                .unwrap()
        ];
        // 
        // println!(" point a: ({x_a} , {y_a})");
        // println!(" point b : ({x_b} , {y_b})");

        if x_a>x_b||y_a>y_b {               ////! point a's x & y MUST be greater than b's accordingly:
            panic!("incompatable coors")    ////! such a panic means that either the data is corrupted
        }                                   ////! OR that the algorithm is invalid...

        let a = Point{x:x_a.clone(),y:y_a.clone()};
        let b = Point{x:x_b.clone(),y:y_b.clone()};
        
        let area = Aera::new(a,b);

        // let action = get_action(String::from(action_slice)); ////! trial for an enum that returns closure
        
        for Point { x:_x, y:_y } in area.points{
            let mut pointOnGrid = grid[_x-1 as usize][_y-1 as usize];

            if line.contains("turn on") {
                // println!("before : {pointOnGrid}");
                pointOnGrid += 1;
            }
            else if line.contains("turn off")&&pointOnGrid>0 {
                // println!("before : {pointOnGrid}");
                pointOnGrid -= 1;
            }
            else if line.contains("toggle") {
                // println!("before : {pointOnGrid}");
                pointOnGrid += 2;
            }
            grid[_x-1 as usize][_y-1 as usize]= pointOnGrid;
            // let mut pointOnGrid = grid[_x-1 as usize][_y-1 as usize];

            // println!("after : {pointOnGrid}\n")
        }
    }

    // let first = &grid[932];
    // println!("{first:?}");
    
    let mut count = 0;
    for line in grid { //counting turned on lights
        for var in line {
            count+=var;
        }
    }
    println!("{count}");

}
pub struct Aera{
    pub points : Vec<Point>
    // points : Vec<Vec<Point>> ////!
}

impl Aera {
    pub fn new(point_a:Point,point_b:Point) -> Self {
        let Point { x:x_a, y:y_a } = point_a;
        let Point { x:x_b, y:y_b } = point_b;

        let mut points :Vec<Point> = vec!();
        // let mut mini_grid:Vec<Vec<Point>> = vec!(); ////! use these in 2 dim grid only

        for i in x_a..x_b+1 {
            // let mut newLine :Vec<Point> = vec!(); ////!
            for j in y_a..y_b+1 {
                // println!("x:{},y:{}",i,j);
                let new_point = Point{
                    x:i,
                    y:j
                };
                points.push(new_point);
                // newLine.push(new_point); ////!
            }
            // mini_grid.push(newLine)  ////! 
        }

        return Aera { 
            points 
            // points : mini_grid ////!
        }
    }
}

#[derive(Debug)]
pub struct Point{
    pub x:usize,  //line of point in grid
    pub y:usize,  //index of point in grid
}

enum action {
    turn_on,
    turn_off,
    toggle
}

pub fn ex_file() ->String {
    let instructions =  fs::read_to_string("./excercice.txt").unwrap();
    // println!("{instructions}");
    return instructions
}

#[allow(unused)]
pub fn get_action(line:String)-> Box<dyn Fn(&bool)->()> {
    let mut Action= Box::new(|_ :&bool|{()});
        if line.starts_with("turn on") {
            println!("turning on...");
            let Action = Box::new(|mut x:bool|{ x=true ; });
        }
        else if line.starts_with("turn off") {
            println!("turning off...");
            let Action = Box::new(|mut x:bool|{x=false ;});
        }
        else if line.starts_with("toggle") {
            println!("toggling...");
            let Action = Box::new(|mut x:bool|{x=!x});
        }
    // return Box::new(|x:bool| Action&&x);
    return Action;
}