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
    /*let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to readline");
    let limit:u32 = input.parse::<u32>().unwrap();*/
    let limit:u32 = 20;
    println!("Hello, world!");
    let mut primes_list: Vec<Prime> = vec![];
    let mut count: u32   = 1;
    while count < limit{
        let mut prime_true:bool = true;
        for p in &mut primes_list{
            if count % p.0 == 0 {
                prime_true = false;
            }
        }
        if !prime_true {
            let temp = Prime(count, count);
            primes_list.push(temp)
        }
        println!("{}",prime_true);
        count = count + 1;
    }
    for p in primes_list{
        println!("{}",p.0)
    }
    

}
