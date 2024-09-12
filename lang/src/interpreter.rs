use crate::parser::Value;

pub trait Interpreter<'source, T> {
    fn interpret(&self, value: &Value<'source>) -> T;
}

pub struct TextInterpreter;

impl<'source> Interpreter<'source, Vec<String>> for TextInterpreter {
    fn interpret(&self, value: &Value<'source>) -> Vec<String> {
        match value {
            Value::Text(s) => vec![s.to_string()],
            Value::TextGroup(group) => group
                .iter()
                .flat_map(|value| self.interpret(value))
                .collect(),
            Value::ExpandableGroup(group) => group.iter().fold(Vec::new(), |current, expander| {
                cartesian_product(current, self.interpret(expander))
            }),
            Value::CharRange(start, end) => (*start..=*end).map(|x| x.to_string()).collect(),
            Value::NumberRange(start, end) => (*start..=*end).map(|x| x.to_string()).collect(),
        }
    }
}

pub struct SizeInterpreter;
impl<'source> Interpreter<'source, u32> for SizeInterpreter {
    fn interpret(&self, value: &Value<'source>) -> u32 {
        match value {
            Value::Text(_) => 1,
            Value::TextGroup(group) => group.iter().map(|value| self.interpret(value)).sum(),
            Value::ExpandableGroup(group) => group
                .iter()
                .fold(1, |current, expander| current * self.interpret(expander)),
            Value::CharRange(start, end) => (*end as u32 - *start as u32) + 1,
            Value::NumberRange(start, end) => (end - start) + 1,
        }
    }
}

fn cartesian_product(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    // Check if either vector is empty
    if left.is_empty() {
        return right.to_vec();
    }
    if right.is_empty() {
        return left.to_vec();
    }

    // Combine the elements of both vectors
    let mut combined = Vec::new();
    for l in &left {
        for r in &right {
            combined.push(format!("{}{}", l, r));
        }
    }

    combined
}
