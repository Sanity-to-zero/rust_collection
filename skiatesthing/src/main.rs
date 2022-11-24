

// universal constants

use std::io;
use std::fs;
fn main() {
    println!("Hello, world!");
    
    // let mut category = String::new();
    // println!("Please enter a category: 'kinematics' 'dynamics' 'energetics'");
    // io::stdin().read_line(&mut category).expect("Could not read line");
    // not needed
    let mut inputs = String::new();
    println!("Please enter what you wish to find as well as given numbers in m/s/kg form: 
    \n eg:  'a vi=0 t=10 d=20'");
    io::stdin().read_line(&mut inputs).expect("Could not read line");
    let equations_file = fs::read_to_string("equations.txt").unwrap();
    let val_to_find = inputs.chars().next().unwrap();
    let mut possible: Vec<String> = vec![];
    let eq_lines = equations_file.lines();
    for line in eq_lines{
        if line.starts_with(val_to_find) {
            possible.push(String::from(line));
        }
    }
    let mut v_ins = inputs.split_ascii_whitespace();
    v_ins.next();
    // eg {vi=0,d=20,t=24}
    let mut equation = String::new();
    for eq in possible{
        let mut eq: Vec<&str> = eq.split(';').collect();
        eq.reverse();
        eq.pop();
        let req = eq.clone().pop().unwrap().split_ascii_whitespace();
        let temp_vec:Vec<&str> = v_ins.clone().collect();
        let temp_req:Vec<&str> = req.collect();
        // eg. {d,a,t}
        let any_works: bool = check_equation(&temp_vec, &temp_req);
        
        // exits loop when equation is found to be more efficient
        if any_works {
            equation.push_str(eq.pop().clone().unwrap());
            break;
        }
        
    }

    // stops program if equation is not found
    if equation.is_empty(){
        panic!("could not find equation!");
    }
    
    // evaluate expression
}


fn check_equation(input: &Vec<&str>, line: &Vec<&str>   )-> bool{
    // returns true if given values match needed values
    
    let var_vec = line.clone().pop().unwrap().split_ascii_whitespace();
    let mut t2 = input.clone();
    let mut works = true;
    t2.reverse();
    for var in var_vec{
        let cur = t2.pop().unwrap();
        if !var.split('=').next().unwrap().eq(cur) {
            works = false;
            break;
        }
    }
    if works {
        return true;
    } else {return false;}
    
}
