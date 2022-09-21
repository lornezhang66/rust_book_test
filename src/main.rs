use rand::Rng;

fn main() {
    println!("Hello, world!");
    let rng:u32 = rand::thread_rng().gen_range(0..=100);
    another_function(rng);
}


fn another_function(i: u32){

    println!("This function number is {i}")
}
