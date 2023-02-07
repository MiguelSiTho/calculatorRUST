use std::io;
//
fn main() {
    println!("[1]Converter or [2]calculator?");
    let mut iniput = String::new();
    io::stdin()
        .read_line(&mut iniput)
        .expect("error!");
    let iniput: i32 = iniput.trim().parse().expect("error!");

    if iniput == 1 {
        println!("Choose a option: 
        1[x + y] 
        2[x - y] 
        3[x * y] 
        4[x / y] 
        5[x ^ y] 
        6[The rest of x / y] 
        7[square root of x and y] 8[x! and y!] 
        9[summation, x = index of summation (i), y = upper limit of summation (n) and z = indexed variable for the sum (x)
        10[product, x = index of product (i), y = upper limit of product (n) and z = indexed variable for the product (x)");
        let mut inputa = String::new();
        io::stdin()
            .read_line(&mut inputa)
            .expect("Failed to read input");
        let w: i32 = inputa.trim().parse().expect("error");
        
        println!("enter the first number:");
        let mut inputb = String::new();
        io::stdin()
            .read_line(&mut inputb)
            .expect("Failed to read input");

        println!("enter the second number:");
        let mut inputc = String::new();
        io::stdin()
            .read_line(&mut inputc)
            .expect("Failed to read input");

        let mut inputd = String::new();

        let x: f64 = inputb.trim().parse().expect("error");
        let y: f64 = inputc.trim().parse().expect("error");
        
        match w{
            1=> println!("{} + {} = {}", x , y,  x + y),
            2=> println!("{} - {} = {}", x , y,  x - y),
            3=> println!("{} * {} = {}", x , y,  x * y),
            4=> println!("{} / {} = {}", x , y,  x / y),
            5=> println!("{} ^ {} = {}", x , y,  f64::powf(x, y)),  
            6=> println!("The rest of {} / {} is {}",x , y,  x % y),
            7=> println!("√{} = {}, √{} = {}", x ,  x.sqrt() , y , y.sqrt()),
            8=> println!("{}! = {}, {}! = {}", x , fact(x as u128), y , fact(y as u128)),
            9 => { println!("enter the third number:");
            io::stdin()
                .read_line(&mut inputd)
                .expect("Failed to read input");
            let z: f64 = inputd.trim().parse().expect("error");
            println!("Result: {}", some(x, y, z))},
            10 => { println!("enter the third number:");
            io::stdin()
                .read_line(&mut inputd)
                .expect("Failed to read input");
                let z: f64 = inputd.trim().parse().expect("error");
            println!("Result: {}", product(x, y, z))},
            _=> println!("error in data"),
        }
    } else {
        println!("Choose a option:\n [1] Celsius to Fahrenheit   [2]Fahrenheit to Celsius");
        let mut input1 = String::new();
        io::stdin()
            .read_line(&mut input1)
            .expect("error!");        
        let input1: i32 = input1.trim().parse().expect("error!");
        match input1 {
            1 => {
                println!("Enter the value");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("error!");
                let input2: i32 = input2.trim().parse().expect("error!");
                println!("{} Celsius in Fahrenheit is: {}", input2, input2 * (9/5) +32);
            }
            2 => {}
        }
    
    }
    //TODO make a way to the user can use multiple inputs
    /*if w == 1 {
        add1(w);
    }
 
    println!("{w}");*/
    
}

fn some(x: f64, y: f64, z: f64) -> f64 {
    let mut sum = 0.0;
    for _i in x as i32..y as i32 {
        sum += z;
    }
    sum
}

fn product(x: f64, y: f64, z: f64) -> f64 {
    let mut sum = 1.0;
    for _i in x as i32..y as i32 {
        sum *= z;
    }
    sum
}

fn fact(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => fact(num - 1) * num,
    }
}


/*fn add1(num: i32) -> i32 {
    num + 1
}*/