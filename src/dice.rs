use rand::Rng;
use regex::Regex;

pub fn roll(expression: &str) -> i32 {
    let re = Regex::new(r"(?P<num>\d*)d(?P<sides>\d+)(?:\s*\+\s*(?P<bonus>\d+))?").unwrap();
    if let Some(caps) = re.captures(expression) {
        let num = caps
            .name("num")
            .map_or("1", |m| m.as_str())
            .parse::<u32>()
            .unwrap();
        let sides = caps["sides"].parse::<u32>().unwrap();
        let bonus = caps
            .name("bonus")
            .map_or(0, |m| m.as_str().parse::<i32>().unwrap());

        let mut total = 0;
        for _ in 0..num {
            total += rand::rng().random_range(1..=sides) as i32;
        }
        total + bonus
    } else {
        0
    }
}
