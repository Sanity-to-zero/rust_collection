

// universal constants

use std::io;
use std::fs;
fn main() {
    println!("Hello, world!");
    
    let mut category = String::new();
    let mut inputs = String::new();
    println!("Please enter a category: 'kinematics' 'dynamics' 'energetics'");
    io::stdin().read_line(&mut category).expect("Could not read line");
    println!("Please enter what you wish to find as well as given numbers in m/s/kg form: 
    \n eg:  'a vi=0 t=10 d=20'");
    io::stdin().read_line(&mut inputs).expect("Could not read line");
    let equations_file = fs::read_to_string("equations.txt");
    // struct equation{
    //     str answer,
    //     str[] requirements,
    //     str equation
    // }
    
}
