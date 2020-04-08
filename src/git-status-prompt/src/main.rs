use std::process::{Command, Output};

fn main() {
    let status = get_status();
    if status.status.success() {
        print!("(");

        match output_as_option(get_branch_name()) {
            Some(branch) => {
                print_in_color(ConsoleColor::Magenta, branch.trim());

                let remote = output_as_option(get_config(&format!("branch.{}.remote", branch.trim())))
                    .and_then(|s| Some(String::from(s.trim())))
                    .unwrap_or(String::from("origin"));
                let remote_branch = output_as_option(get_config(&format!("branch.{}.merge", branch.trim())))
                    .and_then(|s| Some(String::from(s.replace("refs/heads/", "").trim())))
                    .unwrap_or(String::from(branch.trim()));
                let full_remote = format!("{}/{}", remote, remote_branch);
                if verify_rev(&full_remote).status.success() {
                    let ahead = String::from_utf8(get_log(&full_remote, branch.trim()).stdout).unwrap().lines().count();
                    let behind = String::from_utf8(get_log(branch.trim(), &full_remote).stdout).unwrap().lines().count();
                    if behind > 0 {
                        print!("↓{}", behind);
                    }
                    if ahead > 0 {
                        print!("↑{}", ahead);
                    }
                } else {
                    print!("_");
                }
            },
            None => {
                if let Some(commit) = output_as_option(get_commit()) {
                    match output_as_option(get_tag(&commit)) {
                        Some(tag) => {
                            print_in_color(ConsoleColor::Magenta, tag.trim());
                        }
                        None => {
                            print_in_color(ConsoleColor::Magenta, &commit[0..6]);
                        }
                    }
                }
            }
        }

        let status = parse_status(&output_as_option(status).unwrap());
        if status.conflicts > 0 {
            print!("|");
            print_in_color(ConsoleColor::Red, &format!("{}", status.conflicts));
        }
        if status.staged > 0 {
            print!("|");
            print_in_color(ConsoleColor::Yellow, &format!("{}", status.staged));
        }
        if status.notstaged > 0 {
            print!("|");
            print_in_color(ConsoleColor::Blue, &format!("{}", status.notstaged));
        }
        if status.untracked > 0 {
            print!("|");
            print_in_color(ConsoleColor::Cyan, &format!("{}", status.untracked));
        }

        print!(")");
    }
}

fn output_as_option(output : Output) -> Option<String> {
    if output.status.success() {
        Some(String::from_utf8(output.stdout).unwrap())
    } else {
        None
    }
}

fn get_status() -> Output {
    Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .arg("-uall")
        .output()
        .unwrap()
}

fn get_branch_name() -> Output {
    Command::new("git")
        .arg("symbolic-ref")
        .arg("-q")
        .arg("--short")
        .arg("HEAD")
        .output()
        .unwrap()
}

fn get_commit() -> Output {
    Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--format=format:%H")
        .output()
        .unwrap()
}

fn get_tag(commit : &str) -> Output {
    Command::new("git")
        .arg("tag")
        .arg("--points-at")
        .arg(commit)
        .output()
        .unwrap()
}

fn get_config(config : &str) -> Output {
    Command::new("git")
        .arg("config")
        .arg(config)
        .output()
        .unwrap()
}

fn verify_rev(rev : &str) -> Output {
    Command::new("git")
        .arg("rev-parse")
        .arg("-q")
        .arg("--verify")
        .arg(rev)
        .output()
        .unwrap()
}

fn get_log(from : &str, to : &str) -> Output {
    Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg(format!("{}..{}", from, to))
        .output()
        .unwrap()
}

struct Status {
    conflicts: usize,
    staged: usize,
    notstaged: usize,
    untracked: usize,
}


fn parse_status(status : &str) -> Status {
    let mut res = Status {
        conflicts: 0,
        staged: 0,
        notstaged: 0,
        untracked: 0,
    };

    for line in status.lines() {
        let first_chars : Vec<char> = line.chars().take(2).collect();
        match first_chars.as_slice() {
            ['D', 'D'] | ['A', 'A'] | [_ , 'U'] | ['U', _] => res.conflicts += 1,
            ['?', '?'] => res.untracked += 1,
            [' ', _] => res.notstaged += 1,
            [_, ' '] => res.staged += 1,
            [_, _] => {
                res.staged += 1;
                res.notstaged += 1;
            }
            _ => {}
        }
    }

    res
}

enum ConsoleColor {
    //Black = 30,
    Red = 31,
    //Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    //White = 37,
}

fn print_in_color(color : ConsoleColor, text : &str) {
    print!("\x1B[01;{}m{}\x1B[0m", color as i32, text);
}