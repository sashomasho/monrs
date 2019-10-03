#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::{env, process};

use layout::Layout;

use crate::output::MonitorLayoutPair;

mod layout;
mod monitors;
mod output;

fn main() {
    let mons = monitors::probe_all();
    let mut mon_map = HashMap::new();

    if !mons.is_empty() {
        println!("\nAttached monitors:")
    }
    for m in mons {
        println!("{}", m);
        mon_map.insert(m.idx, m);
    }

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        short_help();
        process::exit(0);
    }

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        long_help();
        process::exit(0);
    }

    let mut pairs: Vec<MonitorLayoutPair> = args
        .iter()
        .filter_map(|s| match Layout::new(s) {
            Ok(layout) => Some(layout),
            Err(e) => {
                println!("\nWarning: {}", e);
                None
            }
        })
        .filter_map(|layout| match mon_map.remove(&layout.mon_idx) {
            Some(mon) => Some(MonitorLayoutPair::new(mon, Some(layout))),
            None => {
                println!(
                    "\nWarning: Monitor with index: {} not found",
                    layout.mon_idx
                );
                None
            }
        })
        .collect();

    if pairs.len() == 0 {
        println!("\nError: No valid arguments provided for current monitor setup");
        process::exit(2)
    }

    //the rest which don't have a matching layout
    for (_, mon) in mon_map {
        pairs.push(MonitorLayoutPair { 0: mon, 1: None })
    }

    output::set_screen_output(&pairs)
}

fn long_help() {
    println!("
Usage:
 {prog} monitor-idx:<rotation>:<x>:<y> ...

Where:
    * monitor-idx as printed by the program, mandatory
    * rotation - N (normal), R (right), I (inverted), L (left) (default: N),
     numbers 0, 90, 180, 270 are also supported
    * x - position X of the monitor absolute px (default: relative to the width of the
     left standing one or 0 if first)
    * y - position Y of the monitor, absolute px (default: same as the value to the left
      stating one or 0 if first)


Sample for 3 monitors:
 0. Laptop Display (1600x1200)
 1. Monitor One (1920x1080),
 2. Monitor Two (1920x1200)

Execute:
 {prog} 1:L 2:::300 0

There are 3 arguments one for each monitors. The missing values (those between the empty
 colons are automatically field according the description above, resulting to:

* 1:L - the left monitor is the one with idx 1 and is rotated by 270 degrees, all other
 settings are set to default, which is effectively the same as 1:L:0:0

* 2:::300 - the second monitor is the one with idx 2, it will be positioned to right of 1,
 and with Y offset set to 300, which is effectively the same as 1:N:1080:300

* 0 - the third monitor with idx 0 will be positioned to the right of 2 with X offset
 equal to the width of the previous two monitors, and Y offset which is effectively the
 same as 1:N:3000:300

Note: If no argument is provided for a monitor, it will be turned off
", prog=env::args().into_iter().nth(0).unwrap());
}

fn short_help() {
    println!("
Usage:
 {prog} monitor-idx:<rotation>:<x>:<y> ...

Options:
 --help for more detailed information", prog=env::args().into_iter().nth(0).unwrap())
}