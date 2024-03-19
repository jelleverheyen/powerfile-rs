use crate::parser::Value;

pub trait Interpreter<'source, T> {
    fn interpret(&self, value: &Value<'source>) -> T;
}

pub struct TextInterpreter;

impl<'source> Interpreter<'source, Vec<String>> for TextInterpreter {
    fn interpret(&self, value: &Value<'source>) -> Vec<String> {
        match value {
            Value::Text(s) => vec![s.to_string()],
            Value::TextGroup(group) => {
                let mut result = Vec::new();
                for value in group {
                    result.extend(self.interpret(&value))
                }
                result
            }
            Value::ExpandableGroup(group) => {
                let result = Vec::new();

                group.iter().fold(result, |current, expander| {
                    combine(current, self.interpret(&expander))
                })
            }
        }
    }
}

fn combine(left: Vec<String>, right: Vec<String>) -> Vec<String> {
    // Check if either vector is empty
    if left.is_empty() {
        return right.to_vec();
    }
    if right.is_empty() {
        return left.to_vec();
    }

    // Combine the elements of both vectors
    let mut combined = Vec::new();
    for r in &right {
        for l in &left {
            combined.push(format!("{}{}", l, r));
        }
    }

    combined
}
