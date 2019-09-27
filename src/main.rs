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
            None => None,
        })
        .collect();

    if pairs.len() == 0 {
        println!("no valid arguments provided for current monitor setup");
        process::exit(1)
    }

    output::set_screen_output(&pairs)
}
