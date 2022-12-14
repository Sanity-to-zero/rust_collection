

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
    let equations_file = fs::read_to_string("skiatesthing/src/equations.txt").unwrap();
    let val_to_find = inputs.split(';').next().unwrap();
    let possible: Vec<String> = get_possible(equations_file, val_to_find);
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


fn check_equation(line: &Vec<&str>, input: &Vec<&str>   )-> bool{
    // returns true if given values match needed values
    
    //let mut var_vec: Vec<&str> = line.clone().pop().unwrap().split_ascii_whitespace().collect();
    let mut t2 = input.clone();
    t2.sort();
    t2.reverse();
    
    print!("{:#?}",line);
    // todo: sort vecs so it can work in any order or alt check through given and see if var exist then pop from given
    for var in line{
        let t3 = var.split('=').next().unwrap();
        let cur = t2.pop().unwrap().split('=').next().unwrap();
        print!("{:#?}",cur);
        println!("{:#?}",t3);
        if !t3.eq(cur) {
            return false;
        }
    }
    return true;
    
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
    #[ignore = "functional"]
    fn test_poss(){ //Functional
        let test_in: Vec<&str> = vec!["vi=0","t=10","d=20"];
        // Fixed: functional
        let file = fs::read_to_string("src/equations1.txt").unwrap();
        let mut poss = get_possible(file, "a");
        let expected_vec: Vec<&str> = vec!["a;  vi d t       ;    (d - 'vi' * 't') / 0.5 * 't' * 't'","a;  vf vi t      ;    ('vf' - 'vi') / 't'","a;  vf vi d      ;    (('vf' * 'vf') - ('vi' * 'vi')) / (2 * d)","a;  fnet m       ;    'fnet' / 'm'"];
        assert_eq!(expected_vec, poss);
        
    }
    #[test]
    
    fn test_eq(){
        let test_in: Vec<&str> = vec!["vi=0","t=10","d=20"];
        let test2_in: Vec<&str> = vec!["vf=0","t=10","d=20"];
        let t3_in: Vec<&str> = vec!["fa=29","W=34"];
        let mut eq: Vec<&str> = "a; d  t  vi ;    (d - 'vi' * 't') / 0.5 * 't' * 't'".split(';').collect();
        let mut e2: Vec<&str> =  "d;                       W  fa ;    'W' / 'fa'".split(';').collect();
        e2.pop();
        eq.pop();
        let l2:Vec<&str> = e2.clone().pop().unwrap().split_ascii_whitespace().collect();
        let req = eq.clone().pop().unwrap().split_ascii_whitespace();
        let temp_req:Vec<&str> = req.collect();
        print!("{:#?}",test2_in);
        assert_eq!(check_equation(&temp_req, &test_in), true);
        assert_eq!(check_equation(&temp_req, &test2_in), false);
        assert_eq!(check_equation(&l2, &t3_in),true);
    }
}

