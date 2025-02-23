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

use top_rss::{Layout, Print, Unit};

mod top_rss;

const VERSION: &str = "0.1";

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    let mut merge_same_name: bool = true;
    let mut number_of_processes: Print = Print::Top(3);
    let mut layout: Layout = Layout::Line;
    let mut unit: Unit = Unit::MB;

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
            "u" | "--unmerge" => {
                merge_same_name = false;
            }
            "-n" => {
                let expected_number = args_iter.next();
                if let Some(number) = expected_number {
                    match number.parse::<usize>() {
                        Ok(n) => number_of_processes = Print::Top(n),
                        Err(_) => {
                            eprintln!("Error: Could not parse '{number}' into number");
                            return;
                        }
                    }
                } else {
                    eprintln!("Error: found option '-n', but no number was provided");
                    return;
                }
            }
            "--lines" => {
                layout = Layout::Lines;
            }

            "-a" | "--all" => {
                number_of_processes = Print::All;
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

    top_rss::toprss(merge_same_name, layout, number_of_processes, unit);
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
  -h, --help, -H, -?        display this help message and exit
  -v, --version             display program's version and exit
  -u, --unmerge             unmerge processes with the same name
  -n,                       display at most top 'n' processes
  -a, --all                 display all processes
  -l, --lines               display each process on separate line
      --kb                  display VmRSS usage in kB
      --mb                  display VmRSS usage in MB 
      --gb                  display VmRSS usage in GB

    "#
    )
}
