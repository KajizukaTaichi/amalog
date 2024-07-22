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

    fn set_fact(&mut self, subject: String, predicate: String) {
        self.logics
            .entry(subject)
            .and_modify(|value| *value = predicate.clone())
            .or_insert(predicate);
    }

    fn query_equal(&mut self, subject: String, predicate: String) -> bool {
        let result = self.logics.get(&subject);
        if let Some(i) = result {
            if *i == predicate {
                true
            } else {
                self.query_equal(i.clone(), predicate)
            }
        } else {
            false
        }
    }

    fn query_what(&mut self, subject: String, init: String) -> Option<String> {
        let predicate = self.logics.get(&subject).cloned();
        if let Some(i) = predicate {
            if i == init {
                println!("Error: lapsed into circular reasoning");
                return None;
            }
            if let Some(j) = self.query_what(i.clone(), init) {
                Some(j)
            } else {
                Some(i.clone())
            }
        } else {
            None
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
                let program = &chars[1..chars.len()].iter().collect::<String>();
                let program = program.split("=").collect::<Vec<&str>>();
                if program.len() >= 2 {
                    let (subject, predicate) = (program[0].trim(), program[1].trim());
                    amalog.set_fact(subject.to_string(), predicate.to_string());
                    println!("Facts: {{ {} }}", {
                        amalog
                            .logics
                            .iter()
                            .map(|(k, v)| format!("{k} = {v}"))
                            .collect::<Vec<String>>()
                            .join(", ")
                    })
                } else {
                    println!("Error: there isn't a equal to separate subject and predicate");
                }
            }
            '?' => {
                let program = &chars[1..chars.len()].iter().collect::<String>();
                let program = program.split("=").collect::<Vec<&str>>();
                if program.len() >= 2 {
                    let (subject, predicate) = (program[0].trim(), program[1].trim());
                    println!(
                        "{}",
                        amalog.query_equal(subject.to_string(), predicate.to_string())
                    );
                } else {
                    println!("Error: there isn't a equal to separate subject and predicate");
                }
            }
            _ => {
                if let Some(i) = amalog.query_what(program.clone(), program) {
                    println!("{}", i.clone())
                }
            }
        }
    }
}
