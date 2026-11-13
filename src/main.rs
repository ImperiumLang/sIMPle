use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file {}: {}", display, why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read file {}: {}", display, why),
        Ok(_) => println!("{} contains: \n {}", display, contents),
    };
}
