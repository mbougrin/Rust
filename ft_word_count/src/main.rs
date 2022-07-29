use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

fn help(name: &String) {
    println!("{name} file_name")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let mut count = 0;
            let file = File::open(&args[1])
                .expect("file not found!");
            let buf_reader = BufReader::new(file);
            for _line in buf_reader.lines() {
                count+=1;
            }
            println!("number word : {}", count);
        },
        _ => {
            help(&args[0]);
        }
    }
    Ok(())
}
