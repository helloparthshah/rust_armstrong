use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let len = input.trim().len();
    let mut sum=0;
    for x in 0..len{
        let a: i32=input[x..(x+1)].parse().unwrap();
        sum +=i32::pow(a, len as u32);
    }
    if sum == n{
        println!("{} is an Armstrong number.",n);
    }
    else{
        println!("{} is not an Armstrong number.",n);
    }
}