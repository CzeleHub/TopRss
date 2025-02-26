//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Lesser General Public License as published by
//     the Free Software Foundation, version 3 of the License.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Lesser General Public License for more details.

//     You should have received a copy of the GNU Lesser General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::VecDeque;

use top_rss::{Layout, Unit};

mod top_rss;

const VERSION: &str = "0.1";

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    let mut do_not_group: bool = false;
    let mut how_many: usize = 3;
    let mut layout: Layout = Layout::Line;
    let mut unit: Unit = Unit::MB;
    let mut show_group_count: bool = false;

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
                do_not_group = true;
            }
            "-n" => {
                let expected_number = args_iter.next();
                if let Some(number) = expected_number {
                    match number.parse::<usize>() {
                        Ok(n) => how_many = n,
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
            "--group" => {} // since its the default behaviour there is nothing to do
            "--lines" => {
                layout = Layout::Lines;
            }

            "-a" | "--all" => {
                how_many = usize::MAX;
            }

            "--group-count" => {
                show_group_count = true;
            }

            "--kb" => {
                unit = Unit::kB;
            }

            "--mb" => {
                unit = Unit::MB;
            }

            "--gb" => {
                unit = Unit::GB;
            }
            _ => {
                eprintln!("Error: Unknown argument '{arg}'");
                return;
            }
        }
    }

    if how_many == 0 {
        return;
    }

    top_rss::toprss(do_not_group, show_group_count, layout, how_many, unit);
}

fn help() {
    println!(
        r#"
TopRSS version: {VERSION}
        usage:
            toprss
            toprss [options]

Command line utility for printing top VmRSS processes

options:
  -h, --help, -H, -?                 display this help message and exit
  -v, --version                      display program's version and exit
  -g, --group        DEFAULT         group processes with the same name
      --group-count                  display how many processes are in a given group
  -u, --ungroup                      ungroup processes with the same name
  -n,                DEFAULT n = 3   display at most top 'n' processes
  -a, --all                          display all processes
  -o, --line         DEFAULT         display processes in one line 
  -l, --lines                        display each process on separate line
      --kb                           display VmRSS usage in kB
      --mb           DEFAULT         display VmRSS usage in MB
      --gb                           display VmRSS usage in GB
  -i, --intelligent                  display VmRSS usage in unit depending on size //to be implemented

    "#
    )
}
