// hopefully optimized Erastothenes sieve
//structure: take input of limit
//start incremental loop
//every prime is added to a key,value list
//key is the prime 
//value is countdown before next multiple
//each loop check if any countdown is True
//if all empty, add to list of primes and key/value

use std::io;
struct Prime (u32, u32);
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to readline");
    let limit:u32 = input.parse::<u32>().unwrap();
    println!("Hello, world!");
    let mut primesList: Vec<Prime> = vec![];
    let mut count: u32   = 1;
    while count < limit{
        let mut primeTrue:bool = true;
        for p in &primesList{
            if count % p.1 == 0 {
                primeTrue = false;
            }
        }
        if !primeTrue {
            let temp = Prime((count), (count));
            primesList.push(temp)
        }
    }
    for p in primesList{
        print!("{}",p.0)
    }
    count = count + 1;

}
