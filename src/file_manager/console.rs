use std::{env, fs, path::PathBuf};

use regex::Regex;

use crate::errors::enums::ErrLevel;

pub enum ConsoleArg {
    Clg,
    File(PathBuf),
    Help,
    Credits,
    License,
    Donate,
    About,
    NoCache,
    
    Level(ErrLevel),
}

pub struct Rule {
    name: &'static str,
    re: Regex,
}

pub struct ConsoleParser {
    args: Vec<String>,
}

impl ConsoleParser {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }

    pub fn parse(&self) -> Vec<ConsoleArg> {
        let console_args: Vec<Rule> = vec![
            Rule {name: "CLG", re: Regex::new(r"^clg$").unwrap()},
            Rule {name: "HELP", re: Regex::new(r"^-help$").unwrap()},
            Rule {name: "CREDITS", re: Regex::new(r"^-credits$").unwrap()},
            Rule {name: "LICENSE", re: Regex::new(r"^-license$").unwrap()},
            Rule {name: "ABOUT", re: Regex::new(r"^-about$").unwrap()},
            Rule {name: "DONATE", re: Regex::new(r"^-donate$").unwrap()},
            Rule {name: "NO_CACHE", re: Regex::new(r"^-nocache$").unwrap()},
            Rule {name: "DEBUG", re: Regex::new(r"^-debug$").unwrap()},
            Rule {name: "WARNING", re: Regex::new(r"^-warn$").unwrap()},
            Rule {name: "ERROR", re: Regex::new(r"^-err$").unwrap()},
            Rule {name: "FILE", re: Regex::new(r#"^(?:[a-zA-Z]:\\|\.{1,2}[\\/]|/)?(?:[^<>:"|?*\r\n]+[\\/])*[^<>:"|?*\r\n]+(?:\.[a-zA-Z0-9]+)?$"#).unwrap()},

        ];

        let exe_path = env::current_exe()
            .ok()
            .and_then(|p| fs::canonicalize(p).ok());

        let mut res: Vec<ConsoleArg> = Vec::new();
        let mut cur_arg: usize = 0;
        let mut pos: usize = 0;

        for a in &self.args {
            let mut best: Option<&Rule> = None;

            for arg in &console_args {
                if let Some(m) = arg.re.find(&a) {
                    if m.len() == a.len() {
                        best = Some(arg);
                        break;
                    }
                }
            }

            if let Some(r) = best {
                match r.name {
                    "CLG" => res.push(ConsoleArg::Clg),

                    "FILE" => {
                        let path = PathBuf::from(a);

                        let canonical = fs::canonicalize(&path).unwrap_or(path.clone());

                        if exe_path.as_ref() == Some(&canonical) {
                            res.push(ConsoleArg::Clg);
                        } else {
                            res.push(ConsoleArg::File(path));
                        }
                    }

                    "HELP" => res.push(ConsoleArg::Help),
                    "CREDITS" => res.push(ConsoleArg::Credits),
                    "LICENSE" => res.push(ConsoleArg::License),
                    "DONATE" => res.push(ConsoleArg::Donate),
                    "ABOUT" => res.push(ConsoleArg::About),
                    "NO_CACHE" => res.push(ConsoleArg::NoCache),
                    "DEBUG" => res.push(ConsoleArg::Level(ErrLevel::Debug)),
                    "WARNING" => res.push(ConsoleArg::Level(ErrLevel::Warning)),
                    "ERROR" => res.push(ConsoleArg::Level(ErrLevel::Error)),
                    _ => unreachable!(),
                }
            } else {
                panic!(
                    "Console, pos {}, arg #{}\n{}\nIncorrect rule",
                    pos + 1,
                    cur_arg + 1,
                    a
                )
            }

            cur_arg += 1;
            pos += a.len() + 1;
        }

        res
    }
}
