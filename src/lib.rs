use wasm_bindgen::prelude::*;
use std::fs::File;
use std::io::{Write, Read};
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;
use chrono::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  println!("The Rust function say() received {}", s);
  let r = String::from("hello ");
  return r + s;
}

#[wasm_bindgen]
pub fn read_file() -> String {
  let local: DateTime<Local> = Local::now();
  let file_name: String = "/".to_string() + &local.format("%Y-%m-%d").to_string() + ".txt";

  _initialize();
  let f1 = File::open(&file_name);
  let mut old_content = String::new();
  match f1 {
      Ok(_file) => {
        // 读取之前的内容
        println!("文件已经存在");
        let mut f = File::open(&file_name).expect("file not found");
        let mut s = String::new();
        old_content = match f.read_to_string(&mut s) {
            Ok(_) => s,
            Err(e) => e.to_string(),
        };
      },
      Err(_err) => {
        // 创建文件
        println!("未读取到文件");
      }
  }

  return old_content;
}

#[wasm_bindgen]
pub fn edit_file(params: &str) -> String {
  let local: DateTime<Local> = Local::now();
  let file_name: String = "/".to_string() + &local.format("%Y-%m-%d").to_string() + ".txt";
  
  // 传过来的变量不允许携带换行符
  let contents: (String, String) = serde_json::from_str(&params).unwrap();
  let name = contents.0.replace("\n", "") + "\n";
  let content = contents.1.replace("\n", "") + "\n";

  println!("传过来的变量为：{} - {}", name, content);

  // 读取文件
  _initialize();

  let f1 = File::open(&file_name);
  let mut old_content = String::new();
  match f1 {
      Ok(_file) => {
        // 读取之前的内容
        println!("文件已经存在");
        let mut f = File::open(&file_name).expect("file not found");
        let mut s = String::new();
        old_content = match f.read_to_string(&mut s) {
            Ok(_) => s,
            Err(e) => e.to_string(),
        };
      },
      Err(_err) => {
        // 创建文件
        println!("未读取到文件");
      }
  }

  // 写入新留言
  let mut output = File::create(&file_name).unwrap();
  output.write_all(old_content.as_bytes()).expect("cannot open file");
  output.write_all(name.as_bytes()).expect("cannot open file");
  output.write_all(content.as_bytes()).expect("cannot open file");

  return old_content.to_string() + &name.to_string() + &content.to_string();
}