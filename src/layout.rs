use std::{error, fmt};

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
}

#[derive(Clone, Debug)]
pub struct InvalidOptionError(String);

impl error::Error for InvalidOptionError {
    //    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    //        // Generic error, underlying cause isn't tracked.
    //        None
    //    }
    fn description(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for InvalidOptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid option <{}>", self.0)
    }
}

impl std::convert::From<std::num::ParseIntError> for InvalidOptionError {
    fn from(e: std::num::ParseIntError) -> Self {
        return InvalidOptionError(e.to_string());
    }
}

type Result<T> = std::result::Result<T, InvalidOptionError>;

impl Layout {
    pub fn new(opt: &str) -> Result<Self> {
        let split: Vec<&str> = opt.split(":").collect();
        if split.len() > 4 {
            return Err(InvalidOptionError(format!("too many ':' in '{}', ignoring", opt)));
        }

        let mon_idx = Self::parse_split(&split, 0)?.unwrap_or(-1);

        let rotation_deg = *split.get(1).or_else(|| Some(&"N")).unwrap();

        let rotation = match rotation_deg {
            "r" | "R" | "90" => Rotation::Right,
            "i" | "I" | "180" => Rotation::Inverted,
            "l" | "L" | "270" => Rotation::Left,
            "" | "n" | "N" | "0" => Rotation::Normal,
            _ => return Err(InvalidOptionError(format!("rotation '{}'", rotation_deg)))
        };

        let pos_x = Self::parse_split(&split, 2)?;
        let pos_y = Self::parse_split(&split, 3)?;

        return Ok(Layout {
            mon_idx,
            rotation,
            position: (pos_x, pos_y),
        });
    }

    fn parse_split(v: &Vec<&str>, pos: usize) -> Result<Option<i32>> {
        if let Some(s) = v.get(pos) {
            if s.len() > 0 {
                return Ok(Some(s.parse::<i32>()?));
            }
        }
        Ok(None)
    }
}

#[test]
fn test_new() {
    let test_valid_strings = vec![
        "1",
        "1",
        "1:90",
        "1:N",
        "1:R:",
        "1::10",
        "1::10",
        "1:::10",
        "1:L::10",
        "1:::",
        "1:I:100:0",
    ];

    for s in test_valid_strings {
        let l1 = Layout::new(&s);
        assert_eq!(l1.is_ok(), true);
        println!("{:?} => {:?}", s, l1.unwrap());
    }
}

#[test]
fn test_bad_new() {
    let test_invalid_strings = vec![
        "a1",
        "1:a",
        "1:90a",
        "1:99:c",
        "1:c:10",
        "1::10:one",
        "1:left::10",
        "1::::::",
    ];

    for s in test_invalid_strings {
        let l1 = Layout::new(&s);
        assert_eq!(l1.is_err(), true);
        println!("{:?} => {:?}", s, l1.err().unwrap());
    }
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
