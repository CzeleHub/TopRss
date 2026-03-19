//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Lesser General Public License as published by
//     the Free Software Foundation, version 3 of the License.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Lesser General Public License for more details.

//     You should have received a copy of the GNU Lesser General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    collections::HashMap,
    fs::{DirEntry, ReadDir},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

pub fn toprss(
    ungroup: bool,
    group_count: bool,
    separator: Layout,
    how_many: usize,
    unit: Option<Unit>,
    path: PathBuf,
) {
    match std::fs::read_dir(path) {
        Ok(proc) => {
            let mut procs = get_processes(proc)
                .into_iter()
                .map(|p| (1, p))
                .collect::<Vec<(u32, Process)>>();

            if !ungroup {
                let mut combined: HashMap<String, (u32, Process)> = HashMap::new();
                let procs_iter = procs.into_iter();
                for p in procs_iter {
                    if let Some(item) = combined.get_mut(p.1.name.as_str()) {
                        item.0 += 1;
                        item.1.kB += p.1.kB;
                    } else {
                        combined.insert(p.1.name.clone(), (1, p.1));
                    }
                }

                procs = combined
                    .into_values()
                    .map(|v| (v.0, v.1))
                    .collect::<Vec<(u32, Process)>>();
            }

            procs.sort_by(|p1, p2| p1.1.kB.cmp(&p2.1.kB));
            procs = procs.into_iter().rev().collect::<Vec<(u32, Process)>>();

            // let mut total: usize = 0;
            // procs.iter().for_each(|p| total += p.1.kB);
            // println!("{}/16G", Unit::GB.string(total));

            display_processes(procs, how_many, group_count, unit, separator);
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
        }
    };
}

fn get_processes(dir: ReadDir) -> Vec<Process> {
    dir.filter_map(|result| match result {
        Ok(dir_entry) => dir_entry.file_name().as_bytes().first().and_then(|byte| {
            if byte.is_ascii_digit() {
                Some(dir_entry)
            } else {
                None
            }
        }),
        Err(_) => None,
    })
    .collect::<Vec<DirEntry>>()
    .iter()
    .filter_map(|dir_entry| {
        let path = dir_entry.path();

        let smaps_rollup = path.join("smaps_rollup");
        let status = path.join("status");
        if let Ok(string_smaps_rollup) = std::fs::read_to_string(smaps_rollup)
            && let Ok(string_status) = std::fs::read_to_string(status)
        {
            try_new_process(&string_status, &string_smaps_rollup)
        } else {
            None
        }
    })
    .collect::<Vec<Process>>()
}

#[allow(non_snake_case)]
fn try_new_process(status: &str, smaps_rollup: &str) -> Option<Process> {
    let name_option = status.lines().find(|line| line.starts_with("Name:"));
    let pss_option = smaps_rollup.lines().find(|line| line.starts_with("Pss:"));

    if let Some(name) = name_option
        && let Some(pss) = pss_option
        && let Some(str_kB) = pss.to_owned().split_whitespace().nth(1)
        && let Ok(kB) = str_kB.parse::<usize>()
    {
        Some(Process {
            name: name.to_owned().split_off(6),
            kB,
        })
    } else {
        None
    }
}

fn display_processes(
    collection: Vec<(u32, Process)>,
    first_n_elements: usize,
    group_count: bool,
    unit: Option<Unit>,
    separator: Layout,
) {
    collection.iter().take(first_n_elements).for_each(|p| {
        let size = if let Some(u) = unit {
            u.string(p.1.kB)
        } else if p.1.kB < 1024 {
            Unit::kB.string(p.1.kB)
        } else if p.1.kB / 1024 < 1024 {
            Unit::MB.string(p.1.kB)
        } else {
            Unit::GB.string(p.1.kB)
        };
        let output = if group_count {
            format!("[{}] {} {}{}", p.0, p.1, size, separator)
        } else {
            format!("{} {}{}", p.1, size, separator)
        };

        print!("{output}");
    });
}

#[derive(Clone)]
#[allow(non_snake_case)]
struct Process {
    name: String,
    pub kB: usize,
}

impl std::fmt::Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Unit {
    kB,
    MB,
    GB,
}

#[allow(non_snake_case)]
impl Unit {
    fn string(&self, kB: usize) -> String {
        match self {
            Unit::kB => format!("{kB} kB"),
            Unit::MB => {
                let MB = kB / 1024;
                format!("{MB} MB")
            }
            Unit::GB => {
                let GB = (kB as f32 / 1024. / 1024. * 100.).trunc() / 100.;
                format!("{GB} GB")
            }
        }
    }
}

pub enum Layout {
    Lines,
    Line,
    Other(String),
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layout::Line => f.write_str(" "),
            Layout::Lines => f.write_str("\n"),
            Layout::Other(string) => f.write_str(string),
        }
    }
}
