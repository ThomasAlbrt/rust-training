#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

pub fn lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    },
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Wrong code!");
                entry.clear();
                state = State::Locked;
            }
            State::Unlocked => {
                println!("Unlocked!");
                return;
            }
        }
    }
}