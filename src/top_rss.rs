// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use std::{collections::HashMap, path::PathBuf};

pub fn toprss(
    merge: bool,
    horizontal_layout: bool,
    all_processes: bool,
    n_processes: Option<usize>,
) {
    let path = PathBuf::from("/proc");
    match std::fs::read_dir(&path) {
        Ok(proc) => {
            let mut procs = proc
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
                                                            .split_off(7)
                                                            .strip_suffix(" kB")
                                                            .unwrap()
                                                            .replace(" ", "")
                                                            .to_owned()
                                                            .parse::<usize>()
                                                            .unwrap(),
                                                    },
                                                    unit: Unit::Mb,
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
            let mut combined: HashMap<String, (usize, &mut Process)> = HashMap::new();
            procs.iter_mut().for_each(|p| {
                if combined.contains_key(&p.name) {
                    combined.get_mut(&p.name).unwrap().0 += 1;
                    combined.get_mut(&p.name).unwrap().1.rss.kB += p.rss.kB;
                } else {
                    combined.insert(p.name.clone(), (1, p));
                }
            });

            let mut procs = combined
                .values()
                .map(|v| (v.0, (*(v.1)).clone()))
                .collect::<Vec<(usize, Process)>>();

            //procs.sort_by(|p1, p2| p1.name.cmp(&p2.name));
            procs.sort_by(|p1, p2| p1.1.rss.kB.cmp(&p2.1.rss.kB));

            let procs = procs.into_iter().rev();
            let procs = procs.into_iter().collect::<Vec<(usize, Process)>>();

            if all_processes {
                procs
                    //.collect::<Vec<Process>>()
                    .iter()
                    .for_each(|p| {
                        if p.0 == 1 {
                            print!("{} ", p.1)
                        } else {
                            print!("[{}]{} ", p.0, p.1)
                        }
                    });
                println!();
            } else {
                procs
                    .iter()
                    .take(n_processes.unwrap_or(3))
                    //.collect::<Vec<Process>>()
                    //.iter()
                    .for_each(|p| {
                        if p.0 == 1 {
                            print!("{} ", p.1)
                        } else {
                            print!("[{}]{} ", p.0, p.1)
                        }
                    });
                println!();
            }
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
        }
    };
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
#[derive(Clone, Copy)]
struct kB {
    kB: usize,
}
impl std::iter::Sum<usize> for kB {
    fn sum<I: Iterator<Item = usize>>(iter: I) -> Self {
        iter.sum()
    }
}
#[derive(Clone)]
enum Unit {
    Mb,
    Gb,
}

impl Unit {
    fn convert(&self, rss: kB) -> usize {
        match self {
            Unit::Mb => rss.kB / 1024,
            Unit::Gb => rss.kB / 1024 / 1024,
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Mb => f.write_str("MB"),
            Unit::Gb => f.write_str("GB"),
        }
    }
}
