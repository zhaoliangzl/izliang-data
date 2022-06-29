#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate serde;
extern crate serde_yaml;

use std::fs;
use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufReader;
// use std::io::prelude::*;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::Path;
use std::process::Command;

const STR_PATH: &str = "/tmp/izliang/data/config";
const STR_CONFIG_PATH: &str = "/tmp/izliang/data/config/server.yaml";

// const STR_PATH :&str = "config";
// const STR_CONFIG_PATH:&str = "config/server.yaml";

fn main() {
    let mut config: Config = init_config_file();

    println!("获取配置{:?}", config);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_config_by_file,
            save_config_by_file,
            handler_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    server: String,
    listen: String,
}

fn init() {
    let output = Command::new(
        "/Users/izliang/Projects/izliangProjects/GoLandProjects/excel-tools/excel_tools",
    )
    .arg(&"-h")
    .output()
    .expect("执行异常，提示");
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
}

/**
* 初始化配置文件
*/
fn init_config_file() -> Config {
    let mut config: Config = Config {
        server: "".to_string(),
        listen: "".to_string(),
    };

    let string = String::from(STR_PATH);
    // 读取文件，如果不存在，则进行创建文件返回空的对象
    let path = Path::new(&string);
    match create_dir_all(path) {
        Ok(f) => f,
        Err(err) => {
            println!("{:?}", err)
        }
    }

    let path = Path::new(STR_CONFIG_PATH);
    // 现在判断文件是否存在，
    let mut file = match File::open(path) {
        Ok(f) => {
            println!("文件存在,加载配置文件");
            let str_data = fs::read_to_string(STR_CONFIG_PATH).expect("read file error");
            println!("配置为:{}", str_data);
            config = serde_yaml::from_str::<Config>(&str_data).expect("get yaml error");
        }
        Err(e) => {
            println!("文件不存在,初始化文件");
            let toml_string = serde_yaml::to_string(&config).unwrap();
            println!("初始化toml:{}", toml_string);
            fs::write(STR_CONFIG_PATH, toml_string).expect("Could not write to file!");
        }
    };
    config
}

#[tauri::command]
fn get_config_by_file() -> Config {
    // If something fails
    // Err("This failed!".into());
    // If it worked
    // Ok("This worked!".into(),"1".into());
    let str_data = fs::read_to_string(STR_CONFIG_PATH).expect("read file error");
    // println!("配置为:{}", str_data);
    let config = serde_yaml::from_str::<Config>(&str_data).expect("get yaml error");
    config
}

#[tauri::command]
fn save_config_by_file(config: Config) {
    // let config :Config = Config { server:server, listen: listen};
    let toml_string = serde_yaml::to_string(&config).unwrap();
    println!("保存toml:{}", toml_string);
    fs::write(STR_CONFIG_PATH, toml_string).expect("Could not write to file!");
    //test_http();
}

#[tauri::command]
fn handler_file(
    input_type: String,
    output_type: String,
    input: String,
    output: String,
    is_dir: bool,
) {
    println!("原类型{}", input_type);
    println!("目标类型{}", output_type);
    println!("输入{}", input);
    println!("输出{}", output);
    println!("目录批处理:{}", is_dir);
    let config = get_config_by_file();
    println!("工具地址:{}", config.server);

    let mut action = String::from(input_type);
    action.push('2');
    action.push_str(&output_type);

    // println!("执行命令:{}",cmd);
    // let cmd = " -a ".to_owned()+&input_type+"2".to_owned()+&output_type+" -i ".to_owned()+&input+" -o ".to_owned()+&output;
    if is_dir {
        let output = Command::new(config.server)
            .arg("-a")
            .arg(action)
            .arg("-r")
            .arg("-i")
            .arg(input)
            .arg("-o")
            .arg(output)
            .output()
            .expect("执行异常，提示");
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    } else {
        let output = Command::new(config.server)
            .arg("-a")
            .arg(action)
            .arg("-i")
            .arg(input)
            .arg("-o")
            .arg(output)
            .output()
            .expect("执行异常，提示");
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    }
}

async fn test_http() {
    if let Ok(resp) = get().await {
        println!("get {:#?}", resp);
    } else {
        println!("get 请求失败")
    }

    if let Ok(res) = post().await {
        println!("post {:#?}", res);
    } else {
        println!("post 请求失败")
    }
}

// #[tauri::command]
// fn get_config_by_file() -> Result<String, String>{
//   println!("我被调用了");
//   let f = std::env::current_dir().unwrap();
//   println!("当前目录 = {}", f.display());
//   // let config = Config{
//   //   server:"1".to_string(),
//   //   listen:"2".to_string(),
//   // };
//   Err("This failed!".into());
//   // If it worked
//   Ok("This worked!".into());
//   // println!("数据:{}",config.server);

// //   let f = File::open("/src/1.txt").unwrap();
// //   let reader = BufReader::new(f);
// //   for line in reader.lines() {
// //     // line 是 std::result::Result<std::string::String, std::io::Error> 类型
// //     // line 不包含换行符
// //     let line = line.unwrap();
// //     println!("{}", line);
// }

async fn get() -> Result<HashMap<String, String>, reqwest::Error> {
    Ok(reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?)
}

async fn post() -> Result<HashMap<String, Value>, reqwest::Error> {
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 组装要提交的数据
    let mut data = HashMap::new();
    data.insert("user", "zhangsan");
    data.insert("password", "https://docs.rs/serde_json/1.0.59/serde_json/");

    // 发起post请求并返回
    Ok(client
        .post("https://httpbin.org/post")
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}
