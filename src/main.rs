mod contentlib;

fn main() {
    let file_name = std::env::args().nth(1).expect("file name not passed");

    let file = std::fs::read_to_string(file_name).expect("unable to read file");

    println!("{}", contentlib::sayversion());

    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value)
        } else {
            println!("Not a number")
        }
    });
}
