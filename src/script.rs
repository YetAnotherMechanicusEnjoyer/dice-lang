use super::dice::roll;
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
            //let val = eval_expr(inner, &self.state);
            println!("{}", inner);
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
    if expr.starts_with("roll(") && expr.ends_with(")") {
        let inner = &expr[5..expr.len() - 1].trim_matches('"');
        return roll(inner);
    }

    if let Some(pos) = expr.find('+') {
        let left = expr[..pos].trim();
        let right = expr[pos + 1..].trim();
        return eval_expr(left, state) + eval_expr(right, state);
    }
    if let Some(pos) = expr.find('-') {
        let left = expr[..pos].trim();
        let right = expr[pos + 1..].trim();
        return eval_expr(left, state) - eval_expr(right, state);
    }

    if let Ok(val) = expr.parse::<i32>() {
        return val;
    }
    if let Some(val) = state.get(expr) {
        return *val;
    }

    0
}
