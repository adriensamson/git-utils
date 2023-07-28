extern crate git2;
use git2::{Repository, Branch};
use std::env::current_dir;

fn main() {
    if let Ok(repo) = Repository::discover(current_dir().unwrap()) {
        if repo.is_empty().unwrap() {
            println!("(empty)");
            return;
        }
        print!("(");

        let head = repo.head().unwrap();
        if head.is_branch() {
            print_in_color(ConsoleColor::Magenta, head.shorthand().unwrap());
            let local_branch = Branch::wrap(head);
            if let Ok(remote_branch) = local_branch.upstream() {
                let (ahead, behind) = repo.graph_ahead_behind(local_branch.into_reference().target().unwrap(), remote_branch.into_reference().target().unwrap()).unwrap();
                if behind > 0 {
                    print!("↓{}", behind);
                }
                if ahead > 0 {
                    print!("↑{}", ahead);
                }
            } else {
                print!("_");
            }
        } else if head.is_tag() {
            print_in_color(ConsoleColor::Magenta, &format!(":{}", head.shorthand().unwrap()));
        } else {
            print_in_color(ConsoleColor::Magenta, &format!(":{}", &head.target().unwrap().to_string()[0..6]));
        }

        let status = get_status(&repo);
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

struct Status {
    conflicts: usize,
    staged: usize,
    notstaged: usize,
    untracked: usize,
}


fn get_status(repo : &Repository) -> Status {
    let mut res = Status {
        conflicts: 0,
        staged: 0,
        notstaged: 0,
        untracked: 0,
    };

    for line in repo.statuses(None).unwrap().iter() {
        if line.status().is_conflicted() {
            res.conflicts += 1;
            continue;
        }
        if line.status().is_index_deleted()
            || line.status().is_index_modified()
            || line.status().is_index_new()
            || line.status().is_index_renamed()
            || line.status().is_index_typechange() {
            res.staged += 1;
        }
        if line.status().is_wt_deleted()
            || line.status().is_wt_modified()
            || line.status().is_wt_renamed()
            || line.status().is_wt_typechange() {
            res.notstaged += 1;
        }
        if line.status().is_wt_new() {
            res.untracked += 1;
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
    print!("\x01\x1B[01;{}m\x02{}\x01\x1B[0m\x02", color as i32, text);
}
