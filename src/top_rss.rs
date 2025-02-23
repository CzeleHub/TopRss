//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Lesser General Public License as published by
//     the Free Software Foundation, version 3 of the License.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Lesser General Public License for more details.

//     You should have received a copy of the GNU Lesser General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{collections::HashMap, path::PathBuf};

pub fn toprss(merge: bool, layout: Layout, how_many: HowMany, unit: Unit) {
    let path = PathBuf::from("/proc");
    match std::fs::read_dir(&path) {
        Ok(proc) => {
            let procs = proc
                .filter_map(|result| match result {
                    Ok(dir_entry) => match dir_entry.file_type() {
                        Ok(ftype) => {
                            if ftype.is_dir() | ftype.is_symlink() {
                                let status = dir_entry.path().join("status");
                                if status.exists() {
                                    match std::fs::read_to_string(status) {
                                        Ok(content) => {
                                            let name_option = content
                                                .lines()
                                                .find(|line| line.starts_with("Name:"));
                                            let rss_option = content
                                                .lines()
                                                .find(|line| line.starts_with("VmRSS:"));
                                            if let Some(name) = name_option {
                                                rss_option.map(|rss| Process {
                                                    name: name.to_owned().split_off(6),
                                                    rss: kB {
                                                        kB: rss
                                                            .to_owned()
                                                            .split_whitespace()
                                                            .nth(1)
                                                            .unwrap()
                                                            .parse::<usize>()
                                                            .unwrap(),
                                                    },
                                                    unit,
                                                })
                                            } else {
                                                None
                                            }
                                        }
                                        Err(err) => {
                                            eprintln!("ERROR: {}", err);
                                            None
                                        }
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Err(err) => {
                            eprintln!("ERROR: {}", err);
                            None
                        }
                    },
                    Err(err) => {
                        eprintln!("ERROR: {}", err);
                        None
                    }
                })
                .collect::<Vec<Process>>();

            let mut combined: HashMap<String, (u32, Process)> = HashMap::new();
            let procs_iter = procs.into_iter();
            for p in procs_iter {
                if let Some(item) = combined.get_mut(p.name.as_str()) {
                    item.0 += 1;
                    item.1.rss.kB += p.rss.kB;
                } else {
                    combined.insert(p.name.clone(), (1, p));
                }
            }
            // procs.into_iter().for_each(|p| {

            // });

            let mut procs = combined
                .into_values()
                .map(|v| (v.0, v.1))
                .collect::<Vec<(u32, Process)>>();

            //procs.sort_by(|p1, p2| p1.name.cmp(&p2.name));
            procs.sort_by(|p1, p2| p1.1.rss.kB.cmp(&p2.1.rss.kB));

            let procs = procs.into_iter().rev().collect::<Vec<(u32, Process)>>();

            display_processes(procs, how_many, layout);
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
        }
    };
}

fn display_processes(collection: Vec<(u32, Process)>, print: HowMany, layout: Layout) {
    match print {
        HowMany::All => {
            collection.iter().for_each(|p| {
                if matches!(layout, Layout::Line) {
                    print!("{} ", p.1)
                } else {
                    println!("{}", p.1)
                }
            });
            if matches!(layout, Layout::Line) {
                println!()
            }
        }
        HowMany::Top(n) => {
            collection.iter().take(n).for_each(|p| {
                if matches!(layout, Layout::Line) {
                    print!("{} ", p.1)
                } else {
                    println!("{}", p.1)
                }
            });
            if matches!(layout, Layout::Line) {
                println!()
            }
        }
    }
}

#[derive(Clone)]
struct Process {
    name: String,
    rss: kB,
    unit: Unit,
}

impl std::fmt::Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "{}: {} {}",
                self.name,
                self.unit.convert(self.rss),
                self.unit
            )
            .as_str(),
        )
    }
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Clone, Copy)]
struct kB {
    kB: usize,
}
impl std::iter::Sum<usize> for kB {
    fn sum<I: Iterator<Item = usize>>(iter: I) -> Self {
        iter.sum()
    }
}
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Unit {
    kB,
    MB,
    GB,
}

enum Number {
    Usize(usize),
    Float(f32),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Usize(u) => f.write_str(&u.to_string()),
            Number::Float(float) => f.write_str(&float.to_string()),
        }
    }
}

impl Unit {
    fn convert(&self, rss: kB) -> Number {
        match self {
            Unit::kB => Number::Usize(rss.kB),
            Unit::MB => Number::Usize(rss.kB / 1024),
            Unit::GB => Number::Float((rss.kB as f32 / 1024. / 1024. * 100.).trunc() / 100.),
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::kB => f.write_str("kB"),
            Unit::MB => f.write_str("MB"),
            Unit::GB => f.write_str("GB"),
        }
    }
}

pub enum Layout {
    Lines,
    Line,
}

pub enum HowMany {
    All,
    Top(usize),
}
