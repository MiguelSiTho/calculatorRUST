use std::io;

fn main() {
    println!("enter a number for x:");
    let mut input1 = String::new();
    io::stdin()
           .read_line(&mut input1)
           .expect("Failed to read input");

    println!("enter a number for y:");
    let mut input2 = String::new();
    io::stdin()
          .read_line(&mut input2)
          .expect("Failed to read input");

    println!("Choose a option: 1[addition], 2[subtraction], 3[multiplication], 4[division], 5[exponatiation], 6[The rest of x / y], 7[square root], 8[exponential]");
    let mut input3 = String::new();
    io::stdin()
         .read_line(&mut input3)
         .expect("Failed to read input");
     
    let x: f64 = input1.trim().parse().expect("error");
    let y: f64 = input2.trim().parse().expect("error");
    let w: i32 = input3.trim().parse().expect("error");

    match w{
        1=> println!("{} + {} = {}", x , y,  x + y),
        2=> println!("{} - {} = {}", x , y,  x - y),
        3=> println!("{} * {} = {}", x , y,  x * y),
        4=> println!("{} / {} = {}", x , y,  x / y),
        5=> println!("{} ^ {} = {}", x , y,  f64::powf(x, y)),  
        6=> println!("The rest of {} / {} is {}",x , y,  x % y),
        7=> println!("√{} = {}, √{} = {}", x ,  x.sqrt() , y , y.sqrt()),
        8=> println!("{}! = {}, {}! = {}", x , fact(x as u128), y , fact(y as u128)),
        _=> println!("error in data"),
    }
 
}

fn fact(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => fact(num - 1) * num,
    }
}

