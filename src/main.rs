use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;
use std::thread;
use std::time;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
struct Config {
    jobs: Vec<Address>,
}

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
struct Address {
    name: String,
    desc: String,
    cmd: String,
}

fn main() {
    println!("Start");

    let args: Vec<String> = env::args().collect();
    let config_file_path = &args[1];
    println!("config file path:{}", config_file_path);

    let config = load_config(config_file_path);
    // println!("{:?}", config);
    run_all_job(config);

    println!("The End");
}

fn run_all_job(config: Config) {
    let delay = time::Duration::from_secs(5);

    loop{
        for job in config.jobs.iter().clone() {
            println!("name {} cmd {}, desc {}", job.name, job.cmd, job.desc);
            // let cmd = &config.jobs[1].cmd;
            run_cmd(&job.cmd);
        }

        println!("sleeping for some sec ");
        thread::sleep(delay);
    }
}

fn load_config(config_file_path: &String) -> Config {
    let mut file = File::open(config_file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}

fn run_cmd(cmd: &String) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    io::stdout().write_all(&output.stdout).unwrap();
}
