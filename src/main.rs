use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;
use std::thread;
use std::time;

use chrono::{self, Timelike};
use crontab::{Crontab, Tm};

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
    crontab: String,
}

struct ScheduledJob<'a> {
    t: Tm,
    address: &'a Address,
}

fn main() {
    println!("Start, version: 2022-08-21");

    let args: Vec<String> = env::args().collect();
    let config_file_path = &args[1];
    println!("config file path:{}", config_file_path);

    let config = load_config(config_file_path);
    // println!("{:?}", config);
    cron_example();
    // run_all_job(config);
    run_all_job_v2(config);

    println!("The End");
}

fn run_all_job_v2(config: Config) {
    let delay = time::Duration::from_secs(10);
    // 이번 시각 계획을 하였는가?
    // 마지막 실행시간 분단위
    // 마지막 시간하고 달라져야지 계획실행

    // 다음 실행시간을 계획
    // 다음 실행 시간이 되면 작업 실행

    let mut last_scheduled_min: u32 = 100;

    let mut job_queue: Vec<ScheduledJob> = Vec::new();

    let mut job_queue_map = HashMap::new();

    loop {
        let now = chrono::offset::Local::now();
        let minute_value = now.minute();
        println!("now min {}", minute_value);

        if last_scheduled_min == minute_value {
            thread::sleep(delay);
            continue;
        } else {
            last_scheduled_min = minute_value;
        }

        // schedule
        for job in config.jobs.iter().clone() {
            println!(
                "name {} crontab {} cmd {}, desc {}",
                job.name, job.crontab, job.cmd, job.desc
            );

            let ne = get_next_event(&job.crontab);
            match ne {
                Some(x) => {
                    // job_queue.push(sj);

                    let key = String::from(&job.name);
                    if !job_queue_map.contains_key(&key) {
                        println!(
                            "next scheduled time: {}-{}-{} {}:{}",
                            x.tm_year + 1900,
                            x.tm_mon + 1,
                            x.tm_mday,
                            x.tm_hour,
                            x.tm_min,
                        );
                        let sj = ScheduledJob { t: x, address: job };

                        job_queue_map.insert(key, sj);
                    }
                }
                None => {}
            }
            // let cmd = &config.jobs[1].cmd;
            // run_cmd(&job.cmd);
        }

        let mut remove_keys: Vec<&str> = Vec::new();

        for (k, sj) in &job_queue_map {
            println!("schedule {}", sj.address.name);
            println!("minute value {} tm_min {}", minute_value, sj.t.tm_min);

            if minute_value as i32 == sj.t.tm_min {
                run_cmd(&sj.address.cmd);
                // job_queue.remove(i);
                remove_keys.push(&sj.address.name);
            }
        }

        // for (i, sj) in (&job_queue).iter().enumerate() {
        //     println!("schedule {}", sj.address.name);
        //     println!("minute value {} tm_min {}", minute_value, sj.t.tm_min);

        //     if minute_value as i32 == sj.t.tm_min {
        //         run_cmd(&sj.address.cmd);
        //         // job_queue.remove(i);
        //         remove_keys.push(&sj.address.name);
        //     }
        // }

        for k in remove_keys {
            job_queue_map.remove(k);
        }

        // println!("sleeping for some sec");
        // thread::sleep(delay);
    }
}

fn run_all_job(config: Config) {
    let delay = time::Duration::from_secs(10);

    loop {
        for job in config.jobs.iter().clone() {
            println!("name {} cmd {}, desc {}", job.name, job.cmd, job.desc);
            // let cmd = &config.jobs[1].cmd;
            run_cmd(&job.cmd);
        }

        println!("sleeping for some sec");
        thread::sleep(delay);
    }
}

fn get_next_event(crontab_str: &str) -> Option<crontab::Tm> {
    let crontab = Crontab::parse(crontab_str).expect("unknown parse error"); // every hour
    let ne: Option<crontab::Tm> = crontab.find_next_event(); // Option<Tm>
    return ne;
}

fn cron_example() {
    let crontab = Crontab::parse("0 * * * *").expect("unknown parse error"); // every hour

    println!("Minutes: {:?}", crontab.schedule.minutes);
    println!("Hours: {:?}", crontab.schedule.hours);

    // See when the next event will occur:
    let ne: Option<crontab::Tm> = crontab.find_next_event(); // Option<Tm>

    match ne {
        Some(x) => println!(
            "result {}-{}-{} {}:{}",
            x.tm_year + 1900,
            x.tm_mon + 1,
            x.tm_mday,
            x.tm_hour,
            x.tm_min,
        ),
        None => {}
    }

    // crontab.find_next_event_utc(); // Option<Tm>

    // Or when the next event relative to a given time is:
    // let time = at_utc(Timespec::new(1500001200, 0));
    // crontab.find_event_after(&time); // Option<Tm>
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
