extern crate regex;
#[macro_use]
extern crate lazy_static;

mod layout;
mod monitors;
mod output;

use crate::output::MonitorLayoutPair;
use layout::Layout;
use std::collections::HashMap;
use std::{env, process};

fn main() {
    let mons = monitors::probe_all();
    let mut mon_map = HashMap::new();
    for m in mons {
        println!("{}", m);
        mon_map.insert(m.idx, m);
    }

    let args: Vec<String> = env::args().skip(1).collect();

    let pairs: Vec<MonitorLayoutPair> = args
        .iter()
        .filter_map(|s| Layout::new(s).ok())
        .filter_map(|layout| match mon_map.remove(&layout.mon_idx) {
            Some(mon) => Some(MonitorLayoutPair::new(mon, layout)),
            None => {
                println!("monitor with index: {} not found", layout.mon_idx);
                None
            }
        })
        .collect();

    if !mon_map.is_empty() {
        for v in mon_map.values() {
            println!(
                "configuration for monitor [{}. {}] not found",
                v.idx, v.name
            );
        }
        process::exit(1)
    }

    if pairs.len() == 0 {
        println!("no valid arguments provided for current monitor setup");
        process::exit(2)
    }

    if pairs
        .iter()
        .find_map(|p| match p.1.on {
            true => Some(true),
            false => None,
        })
        .is_none()
    {
        println!("blackout mode not supported");
        process::exit(3)
    }

    output::set_screen_output(&pairs)
}
