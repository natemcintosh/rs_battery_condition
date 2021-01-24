# rs_battery_condition
### Author: Nathan McIntosh

---
## About
Monitor your Mac's battery cycle count, condition, and maximum capacity. Put the results in a SQLite database. This tool is meant to be run as a cronjob to track your battery's stats over time. 

---
## Setup
1. Make sure you have the [Rust programming language installed](https://www.rust-lang.org)
1. Clone this repo to a location on your computer: `git clone https://github.com/natemcintosh/rs_battery_condition.git`
1. Create the database somewhere on your system, e.g. `sqlite3 /path/to/your/battery_condition.db < create_battery_condition_table.sql`
1. Build in release mode: `cargo build --release`
1. Optional: test whether it works with your database: 
    1. Run the tool: `./target/release/rs_battery_condition /path/to/your/battery_condition.db`
    1. Check that a row was added to the database: `sqlite3 /path/to/your/battery_condition.db < print_table.sql`
1. Optional: add a cronjob to run it as often as you like. To edit your cronjobs: `crontab -e`. Then add a line like this: `30 6 * * * /path/to/this/repo/rs_battery_condition/target/release/rs_battery_condition /path/to/your/battery_condition.db >> /tmp/get_battery_condition.log 2>&1`
    - `30 6 * * *` means run the tool every day at 06:30. See a site like [this](https://crontab.guru) to help figure out how often you want to run the tool. 
    - The ` >> /tmp/get_battery_condition.log 2>&1` will pipe any errors to the log `/tmp/get_battery_condition.log`. Useful for debugging issues while setting up the cronjob. 
    - Be *sure* that all paths are absolute paths. Cron does not run in the same interactive environment with the same paths as you do, and so needs to be pointed directly to everything. 