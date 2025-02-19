use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("/proc");
    match std::fs::read_dir(&path) {
        Ok(dir) => {
            let mut ls = dir
                .filter_map(|result| result.ok())
                .filter_map(|dir| dir.file_name().into_string().ok())
                .filter_map(|s| std::fs::read_to_string(format!("/proc/{s}/status")).ok())
                .filter_map(|content| {
                    let name_option = content.lines().find(|line| line.starts_with("Name:"));
                    let rss_option = content.lines().find(|line| line.starts_with("VmRSS:"));
                    if name_option.is_some() & rss_option.is_some() {
                        return Some((
                            name_option.unwrap().to_owned().split_off(6),
                            rss_option
                                .unwrap()
                                .to_owned()
                                .split_off(7)
                                .strip_suffix(" kB")
                                .unwrap()
                                .replace(" ", "")
                                .to_owned()
                                .parse::<u32>()
                                .unwrap(),
                        ));
                    } else {
                        None
                    }
                })
                .map(|pair| (pair.0, pair.1 / 1024))
                .collect::<Vec<(String, u32)>>();
            ls.sort_by(|v1, v2| v1.1.cmp(&v2.1));
            println!("{:?}", ls.last().unwrap());
        }
        Err(err) => {
            println!("ERROR: {}", err)
        }
    }
}
