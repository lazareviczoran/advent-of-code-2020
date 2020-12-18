use std::fs::read_to_string;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let data = read1("input.txt");
    println!("part1 solution: {:?}", calculate_sum(&data));
    let data = read2("input.txt");
    println!("part2 solution: {}", calculate_sum(&data));
}

fn calculate_sum(equations: &[Equation]) -> usize {
    equations.iter().map(|eq| evaluate(eq)).sum()
}

fn evaluate(equation: &Equation) -> usize {
    let mut total = 0;
    let mut curr_op = None;
    let mut op_iter = equation.ops.iter();
    for val in &equation.vals {
        match val {
            Value::Val(v) => execute(&mut total, &curr_op, *v),
            Value::Eq(eq) => execute(&mut total, &curr_op, evaluate(eq)),
        }
        curr_op = op_iter.next();
    }
    total
}

fn execute(total: &mut usize, op: &Option<&Operator>, val: usize) {
    match op {
        Some(Operator::Add) | None => *total += val,
        Some(Operator::Mul) => *total *= val,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
enum Value {
    Val(usize),
    Eq(Equation),
}

#[derive(Debug, Clone)]
struct Equation {
    vals: Vec<Value>,
    ops: Vec<Operator>,
}

fn parse_equation(
    chars: &mut Peekable<Chars<'_>>,
    map_fn: &dyn Fn(&mut Vec<Value>, &mut Vec<Operator>, Value) -> Value,
) -> Equation {
    let mut vals = Vec::new();
    let mut ops = Vec::new();
    while let Some(ch) = chars.next() {
        match ch {
            '0'..='9' => {
                let mut curr_token = String::new();
                curr_token.push(ch);
                while let Some(ch2) = chars.peek() {
                    if ch2.is_ascii_digit() {
                        curr_token.push(*ch2);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let curr_val = map_fn(&mut vals, &mut ops, Value::Val(curr_token.parse().unwrap()));
                vals.push(curr_val)
            }
            '(' => {
                let curr_val = map_fn(
                    &mut vals,
                    &mut ops,
                    Value::Eq(parse_equation(chars, map_fn)),
                );
                vals.push(curr_val);
            }
            ')' => return Equation { vals, ops },
            '+' => ops.push(Operator::Add),
            '*' => ops.push(Operator::Mul),
            _ => {}
        }
    }
    Equation { vals, ops }
}

fn wrap_if_addition(vals: &mut Vec<Value>, ops: &mut Vec<Operator>, val: Value) -> Value {
    let mut curr_val = val;
    if ops.last() == Some(&Operator::Add) {
        let op = ops.pop().unwrap();
        let temp_vals = vec![vals.pop().unwrap(), curr_val];
        curr_val = Value::Eq(Equation {
            vals: temp_vals,
            ops: vec![op],
        });
    }
    curr_val
}

fn read1(filename: &str) -> Vec<Equation> {
    read(filename, &|_, _, x| x)
}

fn read2(filename: &str) -> Vec<Equation> {
    read(filename, &wrap_if_addition)
}

fn read(
    filename: &str,
    map_fn: &dyn Fn(&mut Vec<Value>, &mut Vec<Operator>, Value) -> Value,
) -> Vec<Equation> {
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| parse_equation(&mut l.chars().peekable(), map_fn))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read1("test-input.txt");
        assert_eq!(calculate_sum(&data), 71);
        let data = read1("test-input1.txt");
        assert_eq!(calculate_sum(&data), 26);
        let data = read1("test-input2.txt");
        assert_eq!(calculate_sum(&data), 437);
        let data = read1("test-input3.txt");
        assert_eq!(calculate_sum(&data), 12240);
        let data = read1("test-input4.txt");
        assert_eq!(calculate_sum(&data), 13632);
    }

    #[test]
    fn test2() {
        let data = read2("test-input.txt");
        assert_eq!(calculate_sum(&data), 231);
        let data = read2("test-input5.txt");
        assert_eq!(calculate_sum(&data), 51);
        let data = read2("test-input1.txt");
        assert_eq!(calculate_sum(&data), 46);
        let data = read2("test-input2.txt");
        assert_eq!(calculate_sum(&data), 1445);
        let data = read2("test-input3.txt");
        assert_eq!(calculate_sum(&data), 669060);
        let data = read2("test-input4.txt");
        assert_eq!(calculate_sum(&data), 23340);
    }
}
