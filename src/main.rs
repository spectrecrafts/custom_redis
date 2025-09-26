use std::io::{self, Write};
use custom_redis::command::Command;
use custom_redis::storage::Redis;

fn main() {
    let mut redis = Redis::open("redis.wal");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let command = Command::parse_inputs(input);

        match command {
            Command::GET { key } => {
                match redis.get(&key) {
                    Some(value) => println!("{}", value),
                    None => println!("(nil)"),
                }
            }
            Command::SET { key, value, .. } => {
                redis.set(key.clone(), value.clone());
                println!("OK");
            }
            Command::DEL { key } => {
                let deleted = redis.delete(&key);
                if deleted {
                    println!("(1)");
                } else {
                    println!("(0)");
                }
            }
            Command::EXIT => {
                Command::handle_exit();
                break;
            }
            Command::UNKNOWN => {
                Command::handle_unknown();
            }
        }
    }
}
