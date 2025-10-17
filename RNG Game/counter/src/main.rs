use std::io::{self};

fn main() {
    let mut count: i64 = 0;
    println!("---------------------COUNTER-----------------------");
    println!("Commands: + | - | add N | sub N | reset | show | quit");
    println!("COUNT IS  = 0 ");

    loop {
        print!("> ");

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("error: failed to read input");
            continue;
        }

        let mut parts = line.split_whitespace();
        let cmd = match parts.next() {
            Some(c) => c,
            None => continue,
        };

        match cmd {
            "inc" | "+" => {
                count += 1;
                println!("count = {}", count);
            }
            "dec" | "-" => {
                count -= 1;
                println!("count = {}", count);
            }
            "add" => match parts.next().and_then(|n| n.parse::<i64>().ok()) {
                Some(n) => {
                    count += n;
                    println!("count = {}", count);
                }
                None => println!("error: add requires a number"),
            },
            "sub" => match parts.next().and_then(|n| n.parse::<i64>().ok()) {
                Some(n) => {
                    count -= n;
                    println!("count = {}", count);
                }
                None => println!("error: sub requires a number"),
            },
            "reset" => {
                count = 0;
                println!("count = {}", count);
            }
            "show" => println!("count = {}", count),
            _ => println!("unknown command"),
        }
    }
}
