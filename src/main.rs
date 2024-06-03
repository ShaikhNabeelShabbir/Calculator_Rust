use std::io;
fn main() {
    println!("Hello, world!");
    println!("Calculator");
    loop{

    println!("Chose Option\nA==Addition\nB==Subtraction\nC==Multiplication\nD==Division\nE==Exit");
    let mut opt=String::new();
    io::stdin().read_line(&mut opt).expect("enter");
    let opt=opt.trim().to_string();
    
    if opt=="E"{
        println!("Exiting");
        break;
    }
    
    println!("Enter the first variable  X  ");
    let mut x=String::new();
    io::stdin().read_line(&mut x).expect("enter");
    let x:i32=x.trim().parse().unwrap();

    println!("Enter the second variable Y ");
    let mut y=String::new();
    io::stdin().read_line(&mut y).expect("enter");
    let y:i32=y.trim().parse().unwrap();
    
    println!("X == {}",x);
    println!("Y == {}",y);

    if opt=="A"{
        println!("X + Y = {}",{x+y})
    }

    if opt=="B"{
        println!("X - Y = {}",{x-y})
    }

    if opt=="C"{
        println!("X * Y = {}",{x*y})
    }
    
    if opt=="D"{
        println!("X + Y = {}",{x/y})
    }

    }
}
