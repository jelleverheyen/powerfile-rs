use crate::parser::Value;

pub trait Visitor<'source, T> {
    fn visit_value(&mut self, value: Value<'source>) -> T;
}

pub struct TextInterpreter;

impl<'source> Visitor<'source, Vec<String>> for TextInterpreter {
    fn visit_value(&mut self, value: Value<'source>) -> Vec<String> {
        return match value {
            Value::Text(s) => vec!(s.to_owned()),
            Value::TextGroup(group) => {
                let mut result = Vec::new();
                for value in group {
                    result.extend(self.visit_value(value))
                }
                result
            },
            Value::ExpandableGroup(group) => {
                let mut result = Vec::new();
                for expander in group {
                    result = combine(&result, &self.visit_value(expander));
                }

                result
            }
        }
    }
}

fn combine(left: &[String], right: &[String]) -> Vec<String> {
    // Check if either vector is empty
    if left.is_empty() {
        return right.to_vec();
    }
    if right.is_empty() {
        return left.to_vec();
    }

    // Combine the elements of both vectors
    let mut combined = Vec::new();
    for r in right {
        for l in left {
            combined.push(format!("{}{}", l, r));
        }
    }
    combined
}