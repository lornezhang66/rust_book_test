use rand::Rng;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Less};

fn main() {
    println!("Hello, world!");
    let rng:u32 = rand::thread_rng().gen_range(0..=100);
    another_function(rng);

    println!("================================================");

    let tuple: (i32,f32) = (1,3.3);

    let a = tuple.0;
    let b = tuple.1;

    println!("a is {a}, b is {b}");


    println!("================================================");

    let x: [i32;5]=[3;5];
    let mut i = 0;
    loop {
        match i.cmp(&x.len()) {
            Equal => {
                println!("over");
                break;
            },
            Less=> {
                println!("{}", x[i]);
            },
            Ordering::Greater => {
                println!("over");
                break;
            }
        }
        i=i+1;

    }

    println!("================================================================");
    let mut i = 0;
    loop {
        if i<x.len() {
            println!("使用控制语句：{}", x[i]);
        } else{
            break;
        }
        i = i+1;
    }

    println!("================================================================");
    let mut i = 0;
    while i < x.len() {
        println!("while 条件循环：{}", x[i]);
        i +=1;
    }

    println!("================================================================");

    for i in x{
        println!("for 循环{}", i);
    }

    for i in (1..=100).rev() {
        println!("for 循环{}",i);
    }
}


fn another_function(i: u32){
    let number = five()+i;
    println!("This function number is {number}")
}


fn five() -> u32 {
    5
}