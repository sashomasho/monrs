use regex::Regex;
use std::fmt;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

const _DEFAULT_IDX: i32 = -1;

#[derive(Debug, PartialEq)]
pub struct Monitor {
    pub idx: i32,
    pub link: String,
    pub width: i32,
    pub height: i32,
    pub name: String,
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{idx}. {name} ({width}x{height}) on {link}",
            idx = self.idx,
            name = self.name,
            width = self.width,
            height = self.height,
            link = self.link
        )
    }
}

struct MonitorBuilder {
    idx: i32,
    header: String,
    edid: String,
    resolutions: Vec<(i32, i32)>,
    is_built: bool,
}

impl MonitorBuilder {
    fn new(idx: i32) -> Self {
        return MonitorBuilder {
            idx,
            header: String::new(),
            edid: String::new(),
            resolutions: vec![],
            is_built: false,
        };
    }

    fn build(mut self) -> Option<Monitor> {
        let m = parse_monitor(&self.header);
        self.is_built = true;
        match m {
            Some(mut mon) => {
                mon.name = self.model().unwrap_or("".to_owned());
                mon.idx = self.idx;
                mon.width = self.resolutions.get(0).map_or(-1, |tpl| tpl.0);
                mon.height = self.resolutions.get(0).map_or(-1, |tpl| tpl.1);
                Some(mon)
            }
            None => None,
        }
    }

    fn add_header(&mut self, header: &str) {
        self.header.push_str(header)
    }

    fn add_edid(&mut self, edid: &str) {
        self.edid.push_str(edid)
    }

    fn add_resolution(&mut self, width: i32, height: i32) {
        self.resolutions.push((width, height));
    }

    fn model(&self) -> Option<String> {
        match Command::new("edid-decode")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(e) => {
                println!("error executing command 'edid-decode': {}", e.to_string());
                None
            }
            Ok(mut p) => {
                p.stdin
                    .as_mut()
                    .unwrap()
                    .write_all(self.edid.as_bytes())
                    .unwrap();

                let mut model: Option<String> = None;
                let mut ascii_text: Vec<String> = vec![];

                let name_str = "Monitor name: ";
                let ascii_str = "ASCII string: ";
                if let Ok(out) = p.wait_with_output() {
                    for line in String::from_utf8(out.stdout).unwrap().lines() {
                        if line.contains(name_str) {
                            if let Some(s) = line.split(name_str).nth(1) {
                                model = Some(s.to_string());
                                break;
                            }
                        } else if line.contains(ascii_str) {
                            if let Some(s) = line.split(ascii_str).nth(1) {
                                ascii_text.push(s.to_string());
                            }
                        }
                    }
                }
                if model.is_some() {
                    return Some(model.unwrap().to_string());
                }

                if ascii_text.len() > 0 {
                    //return Some(ascii_text[0].to_string());
                    /*
                        let mut pfx = ascii_text[0].to_string();
                        let sfx = ascii_text.split_at(1).1.join(" ");
                        if sfx.len() > 0 {
                            pfx.push_str(" (");
                            pfx.push_str(&sfx);
                            pfx.push(')');
                        }
                    */
                    ascii_text.sort();
                    ascii_text.dedup();
                    return Some(ascii_text.join(" "));
                }
                Some("unknown".to_string())
            }
        }
    }
}

pub fn probe_all() -> Vec<Monitor> {
    let output = Command::new("xrandr")
        .arg("--props")
        .output()
        .ok()
        .expect("failed to execute, is xrandr available?");
    let s = String::from_utf8(output.stdout).expect("error processing output");

    lazy_static! {
        static ref RES_RE: Regex = Regex::new(r"\s+?(\d{3,5})x(\d{3,5}) .*").unwrap();
    }
    let mut lines = s.lines();
    let mut in_edid = false;
    let mut builder = MonitorBuilder::new(_DEFAULT_IDX);

    let mut idx = 0;
    let mut mons: Vec<Monitor> = vec![];
    while let Some(line) = lines.next() {
        if line.contains(" connected") {
            if let Some(mon) = builder.build() {
                mons.push(mon);
            }
            //if line.contains(" connected") {
            builder = MonitorBuilder::new(idx);
            builder.add_header(line);
            idx += 1;
            //}
        }
        if line.contains("EDID") {
            in_edid = true;
        } else if in_edid {
            if line.contains(":") {
                in_edid = false
            } else {
                builder.add_edid(line.trim());
            }
        } else {
            let captures = RES_RE.captures(line);
            if captures.is_some() {
                // println!("{} zzz {:?}", s, captures.iter().nth(2).unwrap());
                let caps = captures.unwrap();
                let width: i32 = caps
                    .get(1)
                    .map(|cap| cap.as_str().parse().unwrap())
                    .unwrap();
                let height: i32 = caps
                    .get(2)
                    .map(|cap| cap.as_str().parse().unwrap())
                    .unwrap();
                builder.add_resolution(width, height);
            }
        }
    }

    if let Some(mon) = builder.build() {
        mons.push(mon);
    }
    // println!("{:?}", mons);
    mons
}

fn parse_monitor(s: &str) -> Option<Monitor> {
    // println!("----> {}", s);
    lazy_static! {
        static ref MON_LINE: Regex = Regex::new(r"(^[A-Za-z0-9\-]+).* connected.*").unwrap();
    }

    let captures = MON_LINE.captures(s);
    if captures.is_some() {
        // println!("{} zzz {:?}", s, captures.iter().nth(2).unwrap());
        let caps = captures.unwrap();
        let link = caps.get(1).map_or("", |m| m.as_str()).to_owned();
        return Some(Monitor {
            idx: _DEFAULT_IDX,
            link,
            width: -1,
            height: -1,
            name: String::new(),
        });
    }

    None
}
