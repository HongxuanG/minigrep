use std::{env, process};

use minigrep::{parse_config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    

    let config = parse_config(&args).unwrap_or_else(|err| {
        println!("发生错误：{}", err);
        process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("读取文件报错：{}", e);
        process::exit(1)
    }
}

