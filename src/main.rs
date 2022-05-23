use std::collections::HashMap;
use std::env;

fn get_clean_arg() -> usize {
    let count: String = match env::args().nth(1) {
        Some(val) => val,
        None => {
            println!("Please enter a numeric value");
            std::process::exit(0x100);
        }
    };
    let count: usize = match count.trim().parse() {
        Ok(num) => num,
        Err(_err) => {
            println!("Numeric values only");
            std::process::exit(0x100);
        }
    };
    count
}

fn fib(count: usize, map: &mut HashMap<usize, u128>) -> u128 {
    if count < 2 {
       return count.try_into().unwrap();
    }

    if let Some(&result) = map.get(&count) {
        return result;
    }
    let result = fib(count - 1, map) + fib(count - 2, map);
    map.insert(count, result);
    result
}

fn main() {
    let count: usize = get_clean_arg();
    if count > 185 {
        println!("The maximum is 185");
        return
    }
    let mut map: HashMap<usize, u128> = HashMap::new();
    let result: u128 = fib(count, &mut map);
    println!("{}", result);
}
