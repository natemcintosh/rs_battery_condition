use chrono::Local;
use regex::Regex;
use rusqlite::Connection;
use std::env;
use std::process::Command;

fn get_cond() -> String {
    let output = Command::new("/usr/sbin/system_profiler")
        .arg("SPPowerDataType")
        .output()
        .expect("Failed to execute command");

    String::from_utf8(output.stdout).expect("Could not understand output of command")
}

fn get_parts_of_interest(cond: &str) -> Vec<&str> {
    let cycle_count_re = Regex::new(r"Cycle Count: (\d+)").unwrap();
    let cycle_count = get_re_group(cycle_count_re, cond).unwrap();

    let condition_re = Regex::new(r"Condition: (\w+)").unwrap();
    let condition = get_re_group(condition_re, cond).unwrap();

    let max_capacity_re = Regex::new(r"Maximum Capacity: (\d+)").unwrap();
    let max_capacity = get_re_group(max_capacity_re, cond).unwrap();

    vec![cycle_count, condition, max_capacity]
}

fn get_re_group(pattern: Regex, s: &str) -> Option<&str> {
    Some(pattern.captures(s)?.get(1)?.as_str())
}
fn main() {
    let cond = get_cond();

    let mut info = get_parts_of_interest(&cond);

    let dt = Local::now().to_rfc3339();
    let mut stuff = vec![dt.as_str()];
    stuff.append(&mut info);

    let cli_args = env::args().skip(1).collect::<Vec<_>>();
    let db_name = cli_args.first().expect("Could not get database path");

    let conn = Connection::open(&db_name).expect("Could not connect to database");
    conn.execute("INSERT INTO battery_condition VALUES (?1,?2,?3,?4)", stuff)
        .expect("Could not add row to database");
}
