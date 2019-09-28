use crate::layout::{Layout, Rotation};
use crate::monitors::Monitor;
use std::process::Command;

pub struct MonitorLayoutPair(pub Monitor, pub Layout);

impl MonitorLayoutPair {
    pub fn new(mon: Monitor, layout: Layout) -> Self {
        MonitorLayoutPair(mon, layout)
    }
}

pub fn set_screen_output(mon_layouts: &Vec<MonitorLayoutPair>) {
    let args = build_args(mon_layouts);
    println!("{:?}", args.join(" "));
    let mut cmd = Command::new("xrandr");
    for a in &args {
        cmd.arg(a);
    }
    let result = cmd.output();
    match result {
        Ok(output) => {
            match output.status.code() {
                Some(0) => println!("all set"),
                _ => println!(
                    "error: {:?}",
                    String::from_utf8(output.stderr).unwrap_or("unknown".to_string())
                ),
            }
            println!("{:?}", String::from_utf8(output.stdout))
        }
        Err(e) => println!("err {}", e.to_string()),
    }
}

pub fn build_args(pairs: &Vec<MonitorLayoutPair>) -> Vec<String> {
    //layouts.sort_by_key(|l| l.mon_idx);

    let mut current_pos_x = 0;

    let mut args = vec![];
    for p in pairs {
        args.push("--output".to_string());
        args.push(p.0.link.to_string());
        args.push("--rotation".to_string());
        args.push(
            match p.1.rotation {
                Rotation::Normal => "normal",
                Rotation::Left => "left",
                Rotation::Right => "right",
                Rotation::Inverted => "inverted",
            }
            .to_string(),
        );

        args.push("--pos".to_string());
        args.push(format!(
            "{}x{}",
            p.1.position.0.unwrap_or(current_pos_x),
            p.1.position.1.unwrap_or(0)
        ));
        current_pos_x += p.0.width;

        args.push("--mode".to_string());
        args.push(format!("{}x{}", p.0.width, p.0.height));
        current_pos_x += match p.1.rotation {
            Rotation::Normal | Rotation::Inverted => p.0.width,
            Rotation::Left | Rotation::Right => p.0.height,
        };
    }
    return args;
}

#[test]
fn test_args() {
    let mons = vec![
        MonitorLayoutPair(
            Monitor {
                idx: 0,
                link: "DisplayPort-1".to_string(),
                width: 1920,
                height: 1200,
                name: "hp".to_string(),
            },
            Layout {
                mon_idx: 0,
                position: (None, None),
                rotation: Rotation::Left,
                on: true,
            },
        ),
        MonitorLayoutPair(
            Monitor {
                idx: 1,
                link: "DisplayPort-2".to_string(),
                width: 1920,
                height: 1080,
                name: "dell".to_string(),
            },
            Layout {
                mon_idx: 1,
                position: (Some(1080), Some(230)),
                rotation: Rotation::Normal,
                on: true,
            },
        ),
    ];

    let res = set_screen_output(&mons);
    println!("{:?}", res);
}
