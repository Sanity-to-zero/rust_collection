// hopefully optimized Erastothenes sieve
//structure: take input of limit
//start incremental loop
//every prime is added to a key,value list
//key is the prime 
//value is countdown before next multiple
//each loop check if any countdown is True
//if all empty, add to list of primes and key/value


use std::io;
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
    for n in 1..size{
        //
    }
    return (0,0);
}
