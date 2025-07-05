use regex::Regex;
use std::{collections::HashMap, fs};

pub type State = HashMap<String, i32>;

pub struct Script {
    lines: Vec<String>,
    state: State,
}

impl Script {
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Load Error: Failed to read script file");
        let lines = content
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with("//"))
            .map(String::from)
            .collect();
        let state = std::collections::HashMap::new();

        Script { lines, state }
    }

    pub fn run(&mut self) {
        let mut i = 0;
        while i < self.lines.len() {
            i += self.run_line(i);
        }
    }

    fn run_line(&mut self, index: usize) -> usize {
        let line = &self.lines[index];

        if line.starts_with("if ") && line.ends_with(" then") {
            return self.run_if(index);
        }

        if line.starts_with("print(") && line.ends_with(")") {
            let inner = &line[6..line.len() - 1];
            let parts = inner.split(',').map(|s| s.trim()).collect::<Vec<_>>();
            let mut output = String::new();

            for part in parts {
                if part.starts_with('"') && part.ends_with('"') {
                    output.push_str(&part[1..part.len() - 1]);
                } else {
                    let val = eval_expr(part, &self.state);
                    output.push_str(&val.to_string());
                }
                output.push(' ');
            }
            println!("{}", output.trim_end());
            return 1;
        }

        if let Some(pos) = line.find('=') {
            let var = line[..pos].trim();
            let expr = line[pos + 1..].trim();
            let val = eval_expr(expr, &self.state);
            self.state.insert(var.to_string(), val);
            return 1;
        }

        1
    }

    fn run_if(&mut self, index: usize) -> usize {
        let line = &self.lines[index];
        let cond = line.trim_start_matches("if ").trim_end_matches(" then");

        if eval_condition(cond, &self.state) {
            if index + 1 < self.lines.len() {
                self.run_line(index + 1);
                return 3;
            }
        } else {
            return 3;
        }
        1
    }
}

fn eval_condition(cond: &str, state: &State) -> bool {
    let ops = ["<=", ">=", "==", "<", ">", "!="];

    for op in ops {
        if let Some(pos) = cond.find(op) {
            let left = cond[..pos].trim();
            let right = cond[pos + op.len()..].trim();
            let lv = eval_expr(left, state);
            let rv = eval_expr(right, state);

            return match op {
                "<=" => lv <= rv,
                ">=" => lv >= rv,
                "==" => lv == rv,
                "<" => lv < rv,
                ">" => lv > rv,
                "!=" => lv != rv,
                _ => false,
            };
        }
    }
    false
}

fn eval_expr(expr: &str, state: &State) -> i32 {
    let mut expr = expr.to_string();

    let dreg = Regex::new(r"(\d*)d(\d+)(?:\s*\+\s*(\d+))?").unwrap();
    expr = dreg
        .replace_all(&expr, |caps: &regex::Captures| {
            let num = caps
                .get(1)
                .map_or("1", |m| m.as_str())
                .parse::<u32>()
                .unwrap_or(1);
            let sides = caps.get(2).unwrap().as_str().parse::<u32>().unwrap_or(6);
            let bonus = caps
                .get(3)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap_or(0));
            let mut total = 0;
            for _ in 0..num {
                total += (rand::random::<u32>() % sides + 1) as i32;
            }
            (total + bonus).to_string()
        })
        .to_string();

    for (key, val) in state.iter() {
        let vreg = Regex::new(&format!(r"\b{}\b", regex::escape(key))).unwrap();
        expr = vreg.replace_all(&expr, val.to_string()).to_string();
    }
    match meval::eval_str(&expr) {
        Ok(result) => result as i32,
        Err(_) => {
            eprintln!("Error: Failed to evaluate expression: {expr}");
            0
        }
    }
}
