use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(query: String, filename: String) -> Config {
        Config { query, filename }
    }
}

pub fn parse_config(args: &Vec<String>) -> Result<Config, &str> {
    if args.len() < 3 {
        return Err("缺少必要的参数");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    Ok(Config::new(query, filename)) 
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  
    // ? 表示 把错误抛出给调用该函数的调用者处理
    let content = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &content) {
      println!("{}", line);
    }
    Ok(())
}
// 搜索 这里的返回值的生命周期 只跟content有关系，所以只标注content和返回值就可以了
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in content.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}


#[cfg(test)]
mod tests {
  // 这里使用use super::*是方便引入上面我们定义的函数：search
  use super::*;

  #[test]
  fn search_sensitive() {
    let query = "duct";
    let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct.
    ";
    assert_eq!(vec!["safe, fast, productive."], search(query, content));
  }
  #[test]
  fn search_insensitive() {
    let query = "duct";
    let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct.
    ";
    assert_eq!(vec!["safe, fast, productive.", "Duct."], search(query, content));
  }
}
