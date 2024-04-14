use std::{io::{stdin, stdout, Write}, process::Command};
use std::path::Path;
use std::env;
fn main() {

    loop {
        print!(":>");
        stdout().flush().expect("Failed to flush stdout");

        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        let mut parts = input.trim().split_whitespace();


        let command: &str = parts.next().unwrap();

        let args = parts;
        match command {
            "cd" =>{
                let new_dir: &str = args.peekable().peek().map_or("/",|x|  *x);
                let new_dir: &str = if new_dir == "~" {"/"} else { new_dir};
                let root: &Path = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root){
                    eprint!("{}", e);
                }

            },
            "exit" => return,
            command =>{
                let child = Command::new(command).args(args).spawn();

                match child {
                    Ok( mut child ) => { let _ = child.wait(); },
                    Err(e) => eprintln!("{}",e),
                };
            }
        }
    }



}
