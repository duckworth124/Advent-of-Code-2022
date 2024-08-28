use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone)]
enum Expression {
    Value(f64),
    Variable,
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Expression::Value(v) => *v,
            Expression::Variable => x,
            Expression::Add(e1, e2) => e1.evaluate(x) + e2.evaluate(x),
            Expression::Sub(e1, e2) => e1.evaluate(x) - e2.evaluate(x),
            Expression::Mul(e1, e2) => e1.evaluate(x) * e2.evaluate(x),
            Expression::Div(e1, e2) => e1.evaluate(x) / e2.evaluate(x),
        }
    }

    fn find_root(&self) -> i64 {
        let exp = if self.evaluate(0.0) > 0.0 {
            Expression::Mul(Box::new(self.clone()), Box::new(Expression::Value(-1.0)))
        } else {
            self.clone()
        };
        let mut lower = 0;
        let mut upper = 1;
        while exp.evaluate(upper as f64) <= 0.0 {
            upper *= 2;
        }

        while upper > lower + 1 {
            let mid = (lower + upper) / 2;
            if exp.evaluate(mid as f64) > 0.0 {
                upper = mid;
            } else {
                lower = mid;
            };
        }
        lower
    }
}

enum Monkey {
    Value(f64),
    Variable,
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Monkey {
    fn get_expression(&self, monkeys: &Monkeys) -> Expression {
        match self {
            Monkey::Value(x) => Expression::Value(*x),
            Monkey::Add(m1, m2) => Expression::Add(
                Box::new(monkeys.get_expression(m1)),
                Box::new(monkeys.get_expression(m2)),
            ),

            Monkey::Sub(m1, m2) => Expression::Sub(
                Box::new(monkeys.get_expression(m1)),
                Box::new(monkeys.get_expression(m2)),
            ),

            Monkey::Mul(m1, m2) => Expression::Mul(
                Box::new(monkeys.get_expression(m1)),
                Box::new(monkeys.get_expression(m2)),
            ),

            Monkey::Div(m1, m2) => Expression::Div(
                Box::new(monkeys.get_expression(m1)),
                Box::new(monkeys.get_expression(m2)),
            ),

            Monkey::Variable => Expression::Variable,
        }
    }

    fn new(input: &str) -> Self {
        if let Ok(x) = input.parse() {
            return Monkey::Value(x);
        };

        let left = input.chars().take(4).collect();
        let right = input.chars().skip(7).collect();
        let op = input.chars().nth(5).unwrap();
        match op {
            '+' => Monkey::Add(left, right),
            '-' => Monkey::Sub(left, right),
            '*' => Monkey::Mul(left, right),
            '/' => Monkey::Div(left, right),
            _ => panic!("invalid character"),
        }
    }
}

struct Monkeys(HashMap<String, Monkey>);

impl Monkeys {
    fn new(input: &str, is_human_unknown: bool) -> Self {
        let monkeys = input
            .lines()
            .map(|line| {
                let name = line.chars().take(4).collect::<String>();
                if name == *"humn" && is_human_unknown {
                    let monkey = Monkey::Variable;
                    return (name, monkey);
                }
                let rhs = line.chars().skip(6).collect::<String>();
                let monkey = Monkey::new(&rhs);
                (name, monkey)
            })
            .collect();

        Monkeys(monkeys)
    }

    fn get_expression(&self, name: &str) -> Expression {
        self.0[name].get_expression(self)
    }

    fn get_function(&self) -> Expression {
        let root_exp = self.get_expression("root");
        match root_exp {
            Expression::Add(l, r) => Expression::Sub(l, r),
            Expression::Sub(l, r) => Expression::Sub(l, r),
            Expression::Mul(l, r) => Expression::Sub(l, r),
            Expression::Div(l, r) => Expression::Sub(l, r),
            _ => panic!(),
        }
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let monkeys_1 = Monkeys::new(&input, false);
    let monkeys_2 = Monkeys::new(&input, true);
    let root_exp = monkeys_1.get_expression("root");
    let output_1 = root_exp.evaluate(0.0);

    let function = monkeys_2.get_function();
    let output_2 = function.find_root();

    println!("part 1: {output_1} part 2: {output_2}");
}
