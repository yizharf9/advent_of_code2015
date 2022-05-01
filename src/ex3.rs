use std::fs;

pub  fn run(){
        let input =fs::read_to_string("C:/Users/user/coding/rust/test_file/src/excercice.txt").unwrap_or(String::from("didnt execute properly..."));
        
        let mut location = Coor{x:0,y:0};
        let mut robo_location = Coor{x:0,y:0};
        let mut stops : Vec<Coor> = vec!(Coor{x:0,y:0});
        let mut arrived : Vec<Coor> = vec!();
        let mut robo_turn = false;
        // let mut arrived : Vec<Coor> = vec!();
        
        // println!("{input}");
        
        for char in input.chars() {
            if robo_turn {
                robo_location = robo_location.move_coor(char); 
                let Coor{x,y}=robo_location;
                let added_robo_stop = Coor{x:x.clone(),y:y.clone()};
                stops.push(added_robo_stop);
            }else {
                location = location.move_coor(char);
                let Coor{x,y}=location;
                let added_stop = Coor{x:x.clone(),y:y.clone()};
                stops.push(added_stop);
            }
            robo_turn = !robo_turn
        }

        for coor in &stops {
            if !coor.included_in(&arrived){
                let Coor { x, y }=coor;
                let added = Coor{
                    x:x.clone(),
                    y:y.clone()
                };
                arrived.push(added);
            }
        }
        
        stops.sort_by_key(|&Coor{x,y:_}|x);

        println!("{:?}",&arrived);
        println!("{:?}",arrived.len());
        // println!("{:?}",input.len());
}



#[derive(Debug)]
//general struct for a Coordinate x,y
pub struct Coor{ 
    x: i32,
    y: i32
}

//basic method for comparing two structs 
impl PartialEq for Coor { 
    fn eq(&self,other:&Coor) -> bool {
        self.x==other.x&&self.y==other.y
    }
}


//method for changing a given coor based on given char input ("<,>,v,^") 
    impl Coor { 
    fn move_coor(mut self,movement:char) ->Self {
        if movement=='>' {
            self.x+=1;
        }
        else if movement=='<' {
            self.x-=1;
        }
        else if movement=='^' {
            self.y+=1
        }
        else if movement=='v'||movement=='V' {
            self.y-=1
        }
        // println!("x:{},y:{}",self.x,self.y);
        return self
    }

    //method that retruns if a coor is included in coor vec
    pub fn included_in(&self,stops:&Vec<Coor>)->bool { 
        for stop in stops {
            if self.x==stop.x&&self.y==stop.y {
                return true
            }
        }
        false
    }
}
