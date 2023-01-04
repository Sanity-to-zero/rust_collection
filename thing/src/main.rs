// hopefully optimized Erastothenes sieve
//structure: take input of limit
//start incremental loop
//every prime is added to a key,value list
//key is the prime 
//value is countdown before next multiple
//each loop check if any countdown is True
//if all empty, add to list of primes and key/value


use std::io::{self, Read};
#[allow(unused)]
fn main() {
    let mut width: String = String::new();
    let mut height: String = String::new();
    let mut input = String::new();
    io::stdin().read_line(&mut width).expect("failed to read line");
    let limit:u32 = width.parse::<u32>().unwrap();
    io::stdin().read_line(&mut height).expect("failed to read line");
    let limit:u32 = height.parse::<u32>().unwrap();
    io::stdin().read_line(&mut input).expect("failed to read line");
    
    

}
#[allow(dead_code)]
#[allow(unused)]
fn get_data(size :u32)->(u32, u32){
    let mut row_counter: Vec<u32> = vec![];
    let mut column_counter: Vec<u32> = vec![];
    for n in 1..size{
        let mut cur = String::new();
        io::stdin().read_line(&mut cur).expect("failed to read line");
        let cur2:Vec<&str> = cur.split_ascii_whitespace().collect();
        if cur2.contains(&"r") {
            if row_counter.binary_search().is_ok() {
                row_counter.remove(index);
            }
        }
    }
    return (0,0);
}
