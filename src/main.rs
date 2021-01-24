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
    let cycle_count = cycle_count_re
        .captures(cond)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let condition_re = Regex::new(r"Condition: (\w+)").unwrap();
    let condition = condition_re
        .captures(cond)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let max_capacity_re = Regex::new(r"Maximum Capacity: (\d+)").unwrap();
    let max_capacity = max_capacity_re
        .captures(cond)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    vec![cycle_count, condition, max_capacity]
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
