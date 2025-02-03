use std::env;

use shitifyer::shitify;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        println!("usage: ./program <image_file_name> <out_file_name>");
        return;
    }

    println!("shitifying: {}", args[1]);
    let _ = shitify::shitify(&args[1], &args[2]);
}
