fn main() {

    let x = 27;
    let y = 7;
    let w = 1;
        
    match w{
        1=> println!("{}", x + y),
        2=> println!("{}", x - y),
        3=> println!("{}", x * y),
        4=> println!("{}", x / y),
        5=> println!("{}", x ^ y),  
        6=> println!("{}", x % y),
        _=> println!("asd"),
    }
 
}

