use std::{env, fs, io::Write, process::exit};
use the_six_characters::transpile;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Usage: {} <input> <output>", args[0]);
        exit(1);
    }

    let contents = match fs::read_to_string(&args[1]) {
        Ok(contents) => contents,
        Err(_) => {
            println!("Unable to read the file");
            exit(1);
        }
    };

    let mut file = match fs::File::create(&args[2]) {
        Ok(file) => file,
        Err(_) => {
            println!("cannot create output file");
            exit(1);
        }
    };

    match file.write_all(transpile(&contents).as_bytes()) {
        Err(_) => {
            println!("Cannot write to output file");
            exit(1);
        }
        _ => {}
    }
}
