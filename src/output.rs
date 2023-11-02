use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use log::debug;

use crate::layout::{Layout, Rotation};
use crate::monitors::Monitor;

#[derive(Debug)]
pub struct MonitorLayoutPair {
    pub monitor: Monitor,
    pub layout: Option<Layout>,
}

impl MonitorLayoutPair {
    pub fn new(monitor: Monitor, layout: Option<Layout>) -> Self {
        MonitorLayoutPair { monitor, layout }
    }
}

pub fn set_screen_output(mon_layouts: &Vec<MonitorLayoutPair>) {
    let all_args = build_args(mon_layouts);
    let mut is_first = true;
    for args in all_args {
        if !is_first {
            sleep(Duration::from_millis(2000));
        }
        is_first = false;
        let mut cmd = Command::new("xrandr");
        for a in &args {
            cmd.arg(a);
        }
        debug!("COMMAND: {:?}", cmd);
        let result = cmd.output();
        match result {
            Ok(output) => {
                match output.status.code() {
                    Some(0) => (),
                    _ => eprintln!(
                        "Error: {:?}",
                        String::from_utf8(output.stderr).unwrap_or("unknown".to_string())
                    ),
                }
                debug!(
                    "OUTPUT: {}",
                    String::from_utf8(output.stdout).unwrap_or("not human readable".to_string())
                );
            }
            Err(e) => eprintln!("err {}", e),
        }
    }
    println!("\nall set");
}

pub fn build_args(pairs: &Vec<MonitorLayoutPair>) -> Vec<Vec<String>> {
    //layouts.sort_by_key(|l| l.mon_idx);

    let mut all_args = vec![];

    let mut current_pos_x = 0;
    let mut current_pos_y = 0;

    let mut args = vec![];
    for p in pairs {
        match &p.layout {
            Some(layout) if layout.force => {
                args.push("--output".to_string());
                args.push(p.monitor.link.to_string());
                args.push("--off".to_string());
                all_args.push(args.clone());
                args.clear();
            }
            _ => (),
        }
        args.push("--output".to_string());
        args.push(p.monitor.link.to_string());
        if let Some(layout) = &p.layout {
            args.push("--rotation".to_string());
            args.push(
                match layout.rotation {
                    Rotation::Normal => "normal",
                    Rotation::Left => "left",
                    Rotation::Right => "right",
                    Rotation::Inverted => "inverted",
                }
                .to_string(),
            );
            args.push("--pos".to_string());
            current_pos_y = layout.position.1.unwrap_or(current_pos_y);
            args.push(format!(
                "{}x{}",
                layout.position.0.unwrap_or(current_pos_x),
                current_pos_y
            ));

            args.push("--mode".to_string());
            args.push(format!("{}x{}", p.monitor.width, p.monitor.height));
            current_pos_x += match layout.rotation {
                Rotation::Normal | Rotation::Inverted => p.monitor.width,
                Rotation::Left | Rotation::Right => p.monitor.height,
            };
            args.push("--auto".to_string());
        } else {
            args.push("--off".to_string());
        }
    }
    all_args.push(args);
    all_args
}

#[test]
fn test_args() {
    let mons = vec![
        MonitorLayoutPair::new(
            Monitor {
                idx: 0,
                link: "DisplayPort-1".to_string(),
                width: 1920,
                height: 1200,
                name: "hp".to_string(),
            },
            Some(Layout {
                mon_idx: 1,
                position: (None, None),
                rotation: Rotation::Left,
                force: false,
                primary: false,
            }),
        ),
        MonitorLayoutPair::new(
            Monitor {
                idx: 1,
                link: "DisplayPort-2".to_string(),
                width: 1920,
                height: 1080,
                name: "dell".to_string(),
            },
            Some(Layout {
                mon_idx: 1,
                position: (Some(1080), Some(230)),
                rotation: Rotation::Normal,
                force: false,
                primary: false,
            }),
        ),
    ];

    set_screen_output(&mons);
}
