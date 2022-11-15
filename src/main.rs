fn main() {

    let x: f64 = 10.5;
    let y: f64 = 7.3;
    let w = 4;
        
    match w{
        1=> println!("{} + {} = {}", x , y,  x + y),
        2=> println!("{} - {} = {}", x , y,  x - y),
        3=> println!("{} * {} = {}", x , y,  x * y),
        4=> println!("{} / {} = {}", x , y,  x / y),
        5=> println!("{} ^ {} = {}", x , y,  f64::powf(x, y)),  
        6=> println!("The rest of {} / {} is {}",x , y,  x % y),
        _=> println!("error in data"),
    }
 
}

