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
    collapse: bool,
    group_count: bool,
    separator: Separator,
    how_many: usize,
    unit: Option<Unit>,
    path: PathBuf,
) {
    match std::fs::read_dir(path) {
        Ok(proc) => {
            let mut procs = get_processes(proc);

            if collapse {
                let mut collapsed: HashMap<(usize, String), Process> = HashMap::new();

                for process in procs.into_iter() {
                    if let Some(existing_process) =
                        collapsed.get_mut(&(process.ppid, process.name.clone()))
                    {
                        existing_process.kB += process.kB;
                    } else {
                        collapsed.insert((process.ppid, process.name.clone()), process);
                    }
                }

                procs = collapsed
                    .into_values()
                    //.map(|v| (v.0, v.1))
                    .collect::<Vec<Process>>();
            }

            procs.sort_by(|p1, p2| p1.kB.cmp(&p2.kB));
            procs = procs.into_iter().rev().collect::<Vec<Process>>();

            display_processes(procs, how_many, group_count, unit, separator);
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
        }
    };
}

fn get_processes(dir: ReadDir) -> Vec<Process> {
    dir.filter_map(|result| {
        if let Ok(dir_entry) = result
            && let Some(byte) = dir_entry.file_name().as_bytes().first()
            && byte.is_ascii_digit()
        {
            Some(dir_entry)
        } else {
            None
        }
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
    let ppid_option = status.lines().find(|line| line.starts_with("PPid:"));
    let pss_option = smaps_rollup.lines().find(|line| line.starts_with("Pss:"));

    if let Some(str_name) = name_option
        && let name = str_name.to_owned().split_off(6)
        && let Some(str_ppid) = ppid_option
        && let Some(str_val_ppid) = str_ppid.split_whitespace().nth(1)
        && let Ok(ppid) = str_val_ppid.parse::<usize>()
        && let Some(pss) = pss_option
        && let Some(str_kB) = pss.split_whitespace().nth(1)
        && let Ok(kB) = str_kB.parse::<usize>()
    {
        Some(Process { ppid, name, kB })
    } else {
        None
    }
}

fn display_processes(
    collection: Vec<Process>,
    first_n_elements: usize,
    group_count: bool,
    option_unit: Option<Unit>,
    separator: Separator,
) {
    collection.iter().take(first_n_elements).for_each(|p| {
        let size = if let Some(unit) = &option_unit {
            unit.string(p.kB)
        } else if p.kB < 1024 {
            Unit::kB.string(p.kB)
        } else if p.kB / 1024 < 1024 {
            Unit::MB.string(p.kB)
        } else {
            Unit::GB.string(p.kB)
        };
        let output = if group_count {
            // format!("[{}] {} {}{}", p.group_count, p, size, separator)
            format!("TODO")
        } else {
            format!("{} {} ppid: {}{}", p.name, size, p.ppid, separator)
        };

        print!("{output}");
    });
}

// #[derive(Clone)]
#[allow(non_snake_case)]
struct Process {
    ppid: usize,
    name: String,
    pub kB: usize,
}

#[allow(non_camel_case_types)]
// #[derive(Clone, Copy)]
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

pub enum Separator {
    Lines,
    Line,
    Other(String),
}

impl std::fmt::Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Separator::Line => f.write_str(" "),
            Separator::Lines => f.write_str("\n"),
            Separator::Other(string) => f.write_str(string),
        }
    }
}
