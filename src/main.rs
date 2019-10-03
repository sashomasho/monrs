#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::{env, process};
use std::collections::HashMap;

use layout::Layout;

use crate::output::MonitorLayoutPair;

mod layout;
mod monitors;
mod output;

fn main() {
    let mons = monitors::probe_all();
    let mut mon_map = HashMap::new();

    if !mons.is_empty() {
        println!("Connected monitors:")
    }
    for m in mons {
        println!("{}", m);
        mon_map.insert(m.idx, m);
    }

    let args: Vec<String> = env::args().skip(1).collect();

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

    if pairs
        .iter()
        .find_map(|p| match &p.1 {
            Some(layout) => {
                if layout.on {
                    Some(true)
                } else {
                    None
                }
            }
            _ => None,
        })
        .is_none()
    {
        println!("\nError: refusing to turn all monitors off");
        process::exit(3)
    }

    output::set_screen_output(&pairs)
}
