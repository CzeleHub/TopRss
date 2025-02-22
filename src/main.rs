// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use std::collections::VecDeque;

mod top_rss;

const VERSION: &str = "0.1";

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    let mut merge = false;
    let mut number_option: Option<usize> = Some(2);
    let mut horizontal_layout = true;
    let mut all_processes = false;

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
            "-m" | "--merge" => {
                merge = true;
            }
            "-n" | "--number" => {
                let expected_number = args_iter.next();
                if let Some(number) = expected_number {
                    match number.parse::<usize>() {
                        Ok(n) => number_option = Some(n),
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
            "--vertical" => {
                horizontal_layout = false;
            }

            "-a" | "--all" => {
                all_processes = true;
            }
            _ => {
                eprintln!("Error: Unknown argument '{arg}'");
                return;
            }
        }
    }

    top_rss::toprss(merge, horizontal_layout, all_processes, number_option);
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
  -h, --help            display this help message and exit
  -v, --version         display program's version number and exit
  -m, --merge           merge processes with the same name
  -n, --number          display at most top 'n' processes
  -a, --all             display all processes
  -c, --color           color the output

    "#
    )
}
