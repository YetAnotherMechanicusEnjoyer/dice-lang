use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Str(String),
}

pub type State = HashMap<String, Value>;

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
                match eval_expr(part, &self.state) {
                    Value::Int(i) => output.push_str(&i.to_string()),
                    Value::Str(s) => output.push_str(&s),
                }
            }
            println!("{}", output.trim_end());
            return 1;
        }

        if line.contains('=') {
            let parts: Vec<_> = line.splitn(2, '=').collect();
            let key = parts[0].trim().to_string();
            let value = eval_expr(parts[1].trim(), &self.state);
            self.state.insert(key, value);
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
    let cond = cond.trim();
    let ops = ["<=", ">=", "==", "<", ">", "!="];

    for op in ops {
        if let Some(pos) = cond.find(op) {
            let left = cond[..pos].trim();
            let right = cond[pos + op.len()..].trim();
            let lv = eval_expr(left, state);
            let rv = eval_expr(right, state);

            return match (lv, rv) {
                (Value::Int(l), Value::Int(r)) => match op {
                    "<=" => l <= r,
                    ">=" => l >= r,
                    "==" => l == r,
                    "<" => l < r,
                    ">" => l > r,
                    "!=" => l != r,
                    _ => false,
                },
                (Value::Str(l), Value::Str(r)) => match op {
                    "==" => l == r,
                    "!=" => l != r,
                    _ => {
                        eprintln!("Error: Invalid operator `{op}` for string comparison");
                        false
                    }
                },
                _ => {
                    eprintln!("Error: Type mismatch in condition: `{cond}`");
                    false
                }
            };
        }
    }
    match eval_expr(cond, state) {
        Value::Int(i) => i != 0,
        Value::Str(s) => !s.is_empty(),
    }
}

fn eval_expr(expr: &str, state: &State) -> Value {
    let mut expr = expr.to_string();

    if expr.starts_with('"') && expr.ends_with('"') {
        return Value::Str(expr[1..expr.len() - 1].to_string());
    }

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
        let vstr = match val {
            Value::Int(i) => i.to_string(),
            Value::Str(s) => format!("\"{s}\""),
        };
        let vreg = Regex::new(&format!(r"\b{}\b", regex::escape(key))).unwrap();
        expr = vreg.replace_all(&expr, vstr).to_string();
    }
    match meval::eval_str(&expr) {
        Ok(result) => Value::Int(result as i32),
        Err(_) => {
            if expr.starts_with('"') && expr.ends_with('"') {
                return Value::Str(expr[1..expr.len() - 1].to_string());
            }
            eprintln!("Error: Failed to evaluate expression: {expr}");
            Value::Int(0)
        }
    }
}
