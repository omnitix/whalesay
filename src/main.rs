use std::{env, io};

pub static WHALE: &str = "      \\         .                  \0       \\       \":\"               \0        \\    ___:____     |\"\\/\"|\0         \\ ,'        `.    \\  /   \0           |  O        \\___/  |    \0         ~^~^~^~^~^~^~^~^~^~^~^~^~";

fn get_len(_str: &str) -> usize {
    _str.chars().count() as usize
}

fn get_max_len(strs: Vec<String>) -> usize {
    let mut result: usize = 0;
    strs.iter().for_each(|_str| {
        let _len = get_len(&_str);
        if _len > result {
            result = _len;
        }
    });
    result
}

fn split_mind(text: String, max_len: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut buf: String = String::new();
    text.split_whitespace().for_each(|word| {
        if get_len(word) > max_len {
            let mut remain = word;
            while !remain.is_empty() {
                if get_len(remain) < max_len {
                    buf = remain.to_string();
                    break;
                } else {
                    let (p, r) = remain.split_at(max_len);
                    result.push(p.to_string());
                    remain = r;
                }
            }
        } else if buf.is_empty() {
            buf.push_str(word);
        } else if get_len(&buf) + get_len(word) + 1 <= max_len {
            buf.push_str(format!(" {}", word).as_str());
        } else {
            result.push(buf.clone());
            buf = word.to_string();
        }
    });
    if !buf.is_empty() {
        result.push(buf);
    }

    result
}

fn print_mind(text: String) {
    let max_line_len = 35_usize;

    let mut splitten_mind: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();
    if text.contains("\n") && args.contains(&"-n".to_string()) {
        text.split("\n").for_each(|t| {
            if !t.is_empty() {
                split_mind(t.to_string(), max_line_len)
                    .iter()
                    .for_each(|line| {
                        splitten_mind.push(line.to_string());
                    });
            }
        });
    } else {
        splitten_mind = split_mind(text.clone(), max_line_len);
    }

    let text_max_len = get_max_len(splitten_mind.clone());
    println!("  {}", "_".repeat(text_max_len + 2));

    splitten_mind.iter().for_each(|line| {
        if splitten_mind.len() == 1 {
            println!(" < {} >", text);
        } else {
            let stroke: String = line.to_string() + &" ".repeat(text_max_len - get_len(line));
            if line == splitten_mind.iter().next().unwrap() {
                println!(" / {} \\", stroke);
            } else if line == splitten_mind.iter().next_back().unwrap() {
                println!(" \\ {} /", stroke);
            } else {
                println!(" | {} |", stroke);
            }
        }
    });

    println!("  {}", "-".repeat(text_max_len + 2));
}

fn print_whale() {
    WHALE.split("\0").for_each(|line| {
        println!("{}", line);
    })
}

fn get_text_from_args(_args: Vec<String>) -> Option<String> {
    let mut args = _args;
    args.remove(0);
    for arg in args {
        if !arg.starts_with("-") {
            return Some(arg);
        }
    }

    None
}

static HELP: &str = "whalesay [-n, --help] \"text\"
-n flag used for saving line breaks.    
\nalso can display pipe text. e.g. `ls / | whalesay`";

fn main() {
    let args: Vec<String> = env::args().collect();
    let text: String;
    if args.contains(&"--help".to_string()) {
        println!("{}", HELP);
    } else {
        text = match get_text_from_args(args) {
            Some(txt) => txt,
            None => io::read_to_string(io::stdin()).unwrap()
        };

        print_mind(text.trim().to_string());
        print_whale();
    }
}
