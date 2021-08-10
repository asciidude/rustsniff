use std::env;
use std::process;
use std::sync::mpsc::{channel};
use std::thread;
mod scanner;
mod args;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = args::Args::new(&args).unwrap_or_else(
        | err | {
            if err.contains("help") {
                process::exit(0);
            } else {
                println!("{0} had issues parsing provided arguments: {1}", args[0].clone(), err);
                process::exit(0);
            }
        }
    );

    let (tx, rx) = channel();
    let threads = arguments.threads;
    let ip = arguments.ip;

    for i in 0..threads {
        let tx = tx.clone();
        
        thread::spawn(move || {
            scanner::scan(tx, i, ip, threads); // f!#k the borrow-checker
        });
    }

    for i in &args {
        println!("{0}", i);
    }

    drop(tx);

    let mut out = vec![];
    for i in rx {
        out.push(i);
    }

    println!("");

    out.sort();
    for i in out {
        println!("{0} is an open port", i);
    }
}