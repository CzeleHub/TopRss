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
    collections::VecDeque,
    path::PathBuf,
};

use top_pss::{Layout, Unit};

mod top_pss;

const VERSION: &str = "0.4";

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    let mut ungroup: bool = false;
    let mut n_processess: usize = 3;
    let mut separator: Layout = Layout::Line;
    let mut unit: Option<Unit> = None;
    let mut group_count: bool = false;
    let mut path: PathBuf = PathBuf::from("/proc");

    // First argument is a program name. We do not need it
    let _self = args.pop_front();

    let mut args_iter = args.iter();
    while let Some(arg) = args_iter.next() {
        let arg = arg.as_str();
        match arg {
            "-h" | "--help" | "-H" | "-?" => {
                help();
                return;
            }
            "-v" | "--version" => {
                println!("TopRSS version: {VERSION}");
                return;
            }
            "-u" | "--ungroup" => {
                ungroup = true;
            }
            "-n" => {
                let expected_number = args_iter.next();
                if let Some(number) = expected_number {
                    match number.parse::<usize>() {
                        Ok(n) => n_processess = n,
                        Err(_) => {
                            eprintln!("Error: Could not parse '{number}' into unsigned integer");
                            return;
                        }
                    }
                } else {
                    eprintln!("Error: found option '-n', but no number was provided");
                    return;
                }
            }
            "--lines" => {
                separator = Layout::Lines;
            }

            "-a" | "--all" => {
                n_processess = usize::MAX;
            }

            "--group-count" => {
                group_count = true;
            }

            "--kb" => {
                unit = Some(Unit::kB);
            }

            "--mb" => {
                unit = Some(Unit::MB);
            }

            "--gb" => {
                unit = Some(Unit::GB);
            }

            "--run-tests-this-option-is-hidden-and-intended-to-be-used-to-perform-tests-by-developer-this-option-name-is-annoingly-long-for-a-reason" => {
                let expected_new_proc_path = args_iter.next();
                if let Some(p) = expected_new_proc_path {
                    let new_path = PathBuf::from(p);
                    if new_path.exists() {
                        path = new_path;
                    } else {
                        eprintln!("Error: Path '{}' does not exists",new_path.display());
                            return;
                    }
                    
                } else {
                    eprintln!("Error: found option '--run-tests-this-option-is-hidden-and-intended-to-be-used-to-perform-tests-by-developer-this-option-name-is-annoingly-long-for-a-reason', but no path was provided");
                    return;
                }
            }
            _ => {
                eprintln!("Error: Unknown argument '{arg}'");
                return;
            }
        }
    }

    if n_processess == 0 {
        return;
    }

    top_pss::toprss(ungroup, group_count, separator, n_processess, unit, path);
}

fn help() {
    println!(
        r#"
TopRSS version: {VERSION}
        usage:
            toppss
            toppss [options]

Command line utility for printing top ram processes

options:
  -h, --help, -H, -?                 display this help message and exit
  -v, --version                      display program's version and exit
      --group-count                  display quantity of grouped processes
  -u, --ungroup                      ungroup processes with the same name
  -n,                DEFAULT n = 3   display at most top 'n' processes
  -a, --all                          display all processes 
  -l, --lines                        display each process on separate line
      --kb                           display ram usage in kB
      --mb                           display ram usage in MB
      --gb                           display ram usage in GB
    "#
    )
}
