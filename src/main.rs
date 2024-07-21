use std::collections::HashMap;
use std::io::{self, Write};

struct AmaLog {
    logics: HashMap<String, String>,
}

impl AmaLog {
    fn new() -> AmaLog {
        AmaLog {
            logics: HashMap::new(),
        }
    }

    fn set_fact(&mut self, subject: String, object: String) {
        self.logics
            .entry(subject)
            .and_modify(|value| *value = object.clone())
            .or_insert(object);
    }

    fn query(&mut self, subject: String, object: String) -> bool {
        let result = self.logics.get(&subject);
        if let Some(i) = result {
            if *i == object {
                true
            } else {
                self.query(i.clone(), object)
            }
        } else {
            false
        }
    }
}

/// Get standard input
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

fn main() {
    println!("AmaLog - simple amateur's logical model \n(c) 2024 梶塚太智. All rights reserved");
    let mut amalog = AmaLog::new();
    loop {
        let program = input("> ");
        let chars = program.chars().collect::<Vec<char>>();
        match chars.get(0).unwrap_or(&' ') {
            '!' => {
                let program = &program[1..chars.len()];
                let program = program.split("=").collect::<Vec<&str>>();
                if program.len() >= 2 {
                    let (subject, object) = (program[0].trim(), program[1].trim());
                    amalog.set_fact(subject.to_string(), object.to_string());
                    println!("Facts: {{{}}}", {
                        amalog
                            .logics
                            .iter()
                            .map(|(k, v)| format!("{k} = {v}"))
                            .collect::<Vec<String>>()
                            .join(", ")
                    })
                } else {
                    println!("Error: not a equal");
                }
            }
            '?' => {
                let program = &program[1..chars.len()];
                let program = program.split("=").collect::<Vec<&str>>();
                if program.len() >= 2 {
                    let (subject, object) = (program[0].trim(), program[1].trim());
                    println!("{}", amalog.query(subject.to_string(), object.to_string()));
                } else {
                    println!("Error: not a equal");
                }
            }
            _ => {}
        }
    }
}
