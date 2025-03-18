//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Lesser General Public License as published by
//     the Free Software Foundation, version 3 of the License.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Lesser General Public License for more details.

//     You should have received a copy of the GNU Lesser General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{collections::VecDeque, io::Write};

const PROC_DIR: &str = "/tmp/toprss/dummy_proc";
const PROC_SUBDIR_1: &str = "/tmp/toprss/dummy_proc/1";
const PROC_SUBDIR_2: &str = "/tmp/toprss/dummy_proc/2";
const PROC_SUBDIR_3: &str = "/tmp/toprss/dummy_proc/3";
const PROC_SUBDIR_4: &str = "/tmp/toprss/dummy_proc/4";
const PROC_SUBDIR_5: &str = "/tmp/toprss/dummy_proc/5";

const PROC_SUBDIR_STATUS_1: &str = "/tmp/toprss/dummy_proc/1/status";
const PROC_SUBDIR_STATUS_2: &str = "/tmp/toprss/dummy_proc/2/status";
const PROC_SUBDIR_STATUS_3: &str = "/tmp/toprss/dummy_proc/3/status";
const PROC_SUBDIR_STATUS_4: &str = "/tmp/toprss/dummy_proc/4/status";
const PROC_SUBDIR_STATUS_5: &str = "/tmp/toprss/dummy_proc/5/status";

fn main() -> Result<(), std::io::Error> {
    // remove a dummy directory if it already exists
    let _ = remove_dummy_directory();
    // create a dummy proc directory in tmp
    create_dummy_directory()?;

    let mut args: VecDeque<String> = std::env::args().collect();

    // First argument is a program name. We do not need it
    let _self = args.pop_front();

    let path_to_toprss = args.pop_front().expect("Error: no path argument provided\nPlease specify path to toprss command as a first argument");

    let test_args = [
        (
            vec!["--all"],
            "Ala: 8.58 GB CatOwnsAla: 887 MB OwnsA: 878 MB Cat: 87 MB \n",
        ),
        (
            vec!["--group-count"],
            "[1] Ala: 8.58 GB [2] CatOwnsAla: 887 MB [1] OwnsA: 878 MB \n",
        ),
        (
            vec!["--ungroup"],
            "Ala: 8.58 GB OwnsA: 878 MB CatOwnsAla: 878 MB \n",
        ),
        (
            vec!["--lines"],
            "Ala: 8.58 GB\nCatOwnsAla: 887 MB\nOwnsA: 878 MB\n",
        ),
        (
            vec!["--kb"],
            "Ala: 9000000 kB CatOwnsAla: 909000 kB OwnsA: 900000 kB \n",
        ),
        (
            vec!["--mb"],
            "Ala: 8789 MB CatOwnsAla: 887 MB OwnsA: 878 MB \n",
        ),
        (
            vec!["--gb"],
            "Ala: 8.58 GB CatOwnsAla: 0.86 GB OwnsA: 0.85 GB \n",
        ),
        (vec!["-n", "0"], ""),
        (vec!["-n", "1"], "Ala: 8.58 GB \n"),
        (
            vec!["-n", "sASASdasda-1"],
            "Error: Could not parse 'sASASdasda-1' into unsigned integer\n",
        ),
        (
            vec!["-n", "-1"],
            "Error: Could not parse '-1' into unsigned integer\n",
        ),
        (
            vec!["-n", "-n"],
            "Error: Could not parse '-n' into unsigned integer\n",
        ),
        (
            vec!["-n", "9999999"],
            "Ala: 8.58 GB CatOwnsAla: 887 MB OwnsA: 878 MB Cat: 87 MB \n",
        ),
        (
            vec!["-n"],
            "Error: found option '-n', but no number was provided\n",
        ),
        (
            vec!["-n", "--help"],
            "Error: Could not parse '--help' into unsigned integer\n",
        ),
        (
            vec!["-n", "4"],
            "Ala: 8.58 GB CatOwnsAla: 887 MB OwnsA: 878 MB Cat: 87 MB \n",
        ),
    ];

    // run program with different args
    for arg in test_args {
        let result = perform_test(&path_to_toprss, arg);
        println!("{result}");
    }

    // remove dummy directory directory
    let _ = remove_dummy_directory();
    Ok(())
}

fn create_dummy_directory() -> Result<(), std::io::Error> {
    std::fs::create_dir_all(PROC_DIR)?;
    std::fs::create_dir_all(PROC_SUBDIR_1)?;
    std::fs::create_dir_all(PROC_SUBDIR_2)?;
    std::fs::create_dir_all(PROC_SUBDIR_3)?;
    std::fs::create_dir_all(PROC_SUBDIR_4)?;
    std::fs::create_dir_all(PROC_SUBDIR_5)?;

    let mut f1 = std::fs::File::create_new(PROC_SUBDIR_STATUS_1)?;
    writeln!(f1, "Name:\tAla")?;
    writeln!(f1, "VmRSS:\t9000000 kB")?;

    let mut f2 = std::fs::File::create_new(PROC_SUBDIR_STATUS_2)?;
    writeln!(f2, "Name:\tOwnsA")?;
    writeln!(f2, "VmRSS:\t900000 kB")?;

    let mut f3 = std::fs::File::create_new(PROC_SUBDIR_STATUS_3)?;
    writeln!(f3, "Name:\tCat")?;
    writeln!(f3, "VmRSS:\t90000")?;

    let mut f4 = std::fs::File::create_new(PROC_SUBDIR_STATUS_4)?;
    writeln!(f4, "Name:\tCatOwnsAla")?;
    writeln!(f4, "VmRSS:\t9000 kB")?;

    let mut f5 = std::fs::File::create_new(PROC_SUBDIR_STATUS_5)?;
    writeln!(f5, "Name:\tCatOwnsAla")?;
    writeln!(f5, "VmRSS:\t900000 kB")?;

    Ok(())
}

fn remove_dummy_directory() -> Result<(), std::io::Error> {
    std::fs::remove_dir_all(PROC_DIR)?;

    Ok(())
}

fn perform_test(program: &str, test: (Vec<&str>, &str)) -> String {
    let mut toprss = std::process::Command::new(program);
    let toprss = toprss
        .arg("--run-tests-this-option-is-hidden-and-intended-to-be-used-to-perform-tests-by-developer-this-option-name-is-annoingly-long-for-a-purpose")
        .arg(PROC_DIR);

    let toprss = if test.0.len() > 1 {
        toprss.args(test.0.as_slice())
    } else {
        toprss.arg(test.0.first().unwrap())
    };

    let toprss = toprss
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let out = toprss.wait_with_output().expect("failed to wait on child");

    //assert!(out.status.success());
    let output = String::from_utf8(out.stderr)
        .expect("Error: Failed to convert Vec<u8, Global> into utf8 String");
    let out = if output.is_empty() {
        String::from_utf8(out.stdout)
            .expect("Error: Failed to convert Vec<u8, Global> into utf8 String")
    } else {
        output
    };

    if out.eq(test.1) {
        format!("\x1b[32m[PASS]\x1b[0m\targ: {:?}", test.0)
    } else {
        let status = format!("\x1b[31m[FAIL]\x1b[0m\targ: {:?}", test.0);
        format!(
            "{status}\n\t\x1b[31m> Result:\x1b[0m\t{:?}\n\t\x1b[31m> Expected:\x1b[0m\t{:?}",
            out, test.1
        )
    }
}
