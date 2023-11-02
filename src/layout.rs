use anyhow::{anyhow, Context};
use log::debug;

#[derive(Debug, Clone)]
pub enum Rotation {
    Normal,
    Left,
    Right,
    Inverted,
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub mon_idx: i32,
    pub rotation: Rotation,
    pub position: (Option<i32>, Option<i32>),
    pub force: bool,
    pub primary: bool,
}

impl Layout {
    pub fn new(opt: &str) -> anyhow::Result<Self> {
        if opt.is_empty() {
            return Err(anyhow!("empty option"));
        }

        let mon_idx: i32 = regex::Regex::new(r"^(\d+).*")
            .unwrap()
            .captures(opt)
            .map(|c| c[1].parse::<i32>().unwrap())
            .context("invalid option - missing monitor")?;

        let rotation = match regex::Regex::new(r".*([rRiIlLnN]).*")
            .unwrap()
            .captures(opt)
        {
            Some(caps) => {
                if caps.len() > 2 {
                    return Err(anyhow!(
                        "invalid option - more than one rotation for monitor {}",
                        mon_idx
                    ));
                }
                match &(caps[1]) {
                    "r" | "R" => Rotation::Right,
                    "i" | "I" => Rotation::Inverted,
                    "l" | "L" => Rotation::Left,
                    "n" | "N" => Rotation::Normal,
                    _ => Rotation::Normal,
                }
            }
            None => Rotation::Normal,
        };

        let pos_x = regex::Regex::new(r".x(-?\d+).*")
            .unwrap()
            .captures(opt)
            .map(|c| c[1].parse::<i32>().unwrap());

        let pos_y = regex::Regex::new(r".y(-?\d+).*")
            .unwrap()
            .captures(opt)
            .map(|c| c[1].parse::<i32>().unwrap());

        let force = opt.contains('f') || opt.contains('F');
        let primary = opt.contains('p') || opt.contains('P');
        let layout = Layout {
            mon_idx,
            rotation,
            position: (pos_x, pos_y),
            force,
            primary,
        };
        debug!("layout: {:?}", layout);
        Ok(layout)
    }
}

#[cfg(test)]
mod tests {
    use crate::layout::Layout;

    #[test]
    fn test_layout() {
        let test_valid_strings = vec![
            "1",
            "1l",
            "1N",
            "1R",
            "1f",
            "1fP",
            "1x10",
            "1y-10",
            "1fpx10y-10",
        ];

        for s in test_valid_strings {
            let l1 = Layout::new(s);
            assert!(l1.is_ok());
            println!("{:?} => {:?}", s, l1.unwrap());
        }
    }

    // #[test]
    // fn test_bad_new() {
    //     let test_invalid_strings = vec![
    //         "a1",
    //         "1a",
    //         "1x90a",
    //         "1:99:c",
    //         "1:c:10",
    //         "1::10:one",
    //         "1:left::10",
    //         "1::::::",
    //     ];
    //
    //     for s in test_invalid_strings {
    //         let l1 = Layout::new(&s);
    //         assert_eq!(l1.is_err(), true);
    //         println!("{:?} => {:?}", s, l1.err().unwrap());
    //     }
    // }
}
/*
0:left 1:::245
<mon_idx>:<rotation=normal>:<x:0>:<y=0>:<on=1>


    7 xrandr \
    8     --output eDP --off \
    9     --output HDMI-A-0 --off \
   10     --output DisplayPort-0 --off \
   11     --output DisplayPort-1 --off \
   12     --output DisplayPort-2 --mode 1920x1080 --pos 0x0 --rotate left \
   13     --output DisplayPort-3 --mode 1920x1200 --pos 1080x360 --rotate normal \
   14     --output DisplayPort-4 --off
    //let mon = m.unwrap();

*/
