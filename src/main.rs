mod contentlib;
use contentlib::sayversion;

pub trait MyTrait {
    fn sum(&self) -> u32;
}

struct MyStruct {}
impl MyTrait for MyStruct {
    fn sum(&self) -> u32 {
        10
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("file name not passed");

    let file = std::fs::read_to_string(file_name).expect("unable to read file");

    let my_stru = MyStruct {};

    println!(" sum = {}", my_stru.sum());

    println!("{}", sayversion::sayversion());

    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value)
        } else {
            println!("Not a number")
        }
    });
}
