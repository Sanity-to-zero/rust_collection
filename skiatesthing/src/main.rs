

// universal constants

use std::io;
use std::fs;
use std::ptr::eq;
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
    let val_to_find = inputs.split(';').next().unwrap();
    let possible: Vec<String> = get_possible(equations_file, val_to_find);
    
    let mut v_ins = inputs.split_ascii_whitespace();
    v_ins.next();
    // eg {vi=0,d=20,t=24}
    let mut equation = "";
    for eq in possible{
        let mut thing: Vec<&str> = eq.split(';').collect();
        thing.reverse();
        thing.pop();
        let mut req = thing.clone().pop().unwrap().split_ascii_whitespace();
        // eg. {d,a,t}
        let mut eq_works: bool = true;
        while let Some(thingy) = v_ins.next() {
            if !thingy.split('=').next().unwrap().eq(req.next().unwrap()) {
                eq_works = false;
                break;
            }
        }
        if eq_works { //use equation / copy expression to equation variable
            let mut temp: Vec<&str> = eq.copy().split(';').collect();
            eq.
            equation = temp.pop().unwrap();
            
        }
        
    }
    if equation.is_empty() {
        panic!("no equation found!");
    }
    // struct equation{
    //     str answer,
    //     str[] requirements,
    //     str equation
    // }
    
}

#[allow(unused)]
fn get_possible(file: String, pattern: &str) -> Vec<String>{
    let temp = file.lines();
    let mut possible = Vec::new();
    for line in temp{
        if line.starts_with(pattern) {
            possible.push(String::from(line));
        }
    }
    return possible;
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_checker(){
        let test_in: Vec<&str> = vec!["vi=0","t=10","d=20"];
        let file = fs::read_to_string("equations.txt").unwrap();
        let poss = get_possible(file, "a");
        
    }
}