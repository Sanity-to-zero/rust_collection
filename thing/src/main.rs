// hopefully optimized Erastothenes sieve
//structure: take input of limit
//start incremental loop
//every prime is added to a key,value list
//key is the prime 
//value is countdown before next multiple
//each loop check if any countdown is True
//if all empty, add to list of primes and key/value

use std::io::Stdin;
struct Prime (u32, u32);
fn main() {
    let limit:u32 = Stdin::read_line(&self, buf);
    println!("Hello, world!");
    let primesList: std::vec::Vec<Prime>;
    let mut count: u32 = 1;
    while count < limit{
        let mut primeTrue:bool = true;
        for p in primesList{
            if count % p.1 == 0 {
                primeTrue = false;
            }
        }
        if !primeTrue {
            primesList.push((count, count))
        }
    }

}
