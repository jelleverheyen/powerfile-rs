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
    let mut combined = Vec::with_capacity(left.len() * right.len());
    for l in &left {
        for r in &right {
            let mut s = String::with_capacity(l.len() + r.len());
            s.push_str(l);
            s.push_str(r);
            combined.push(s);
        }
    }

    combined
}

pub struct AstPrettyPrintInterpreter;
impl<'source> Interpreter<'source, Vec<String>> for AstPrettyPrintInterpreter {
    fn interpret(&self, value: &Value<'source>) -> Vec<String> {
        let mut out = Vec::new();
        out.push(self.label(&value));

        // recurse into children *as children*
        let children: &[Value<'source>] = match value {
            Value::TextGroup(v) | Value::ExpandableGroup(v) => v,
            _ => return out,
        };

        let len = children.len();
        for (i, child) in children.iter().enumerate() {
            self.print_node(child, "", i == len - 1, &mut out);
        }

        return out;
    }
}

impl AstPrettyPrintInterpreter {
    fn label<'source>(&self, value: &Value<'source>) -> String {
        match value {
            Value::Text(s) => format!("Text({:?})", s),
            Value::TextGroup(_) => "TextGroup".to_string(),
            Value::ExpandableGroup(_) => "ExpandableGroup".to_string(),
            Value::CharRange(a, b) => format!("CharRange({:?}..{:?})", a, b),
            Value::NumberRange(a, b) => format!("NumberRange({:?}..{:?})", a, b),
        }
    }

    fn print_node<'source>(
        &self,
        value: &Value<'source>,
        prefix: &str,
        is_last: bool,
        out: &mut Vec<String>,
    ) {
        let branch = if is_last { "└── " } else { "├── " };
        let label = self.label(&value);

        out.push(format!("{}{}{}", prefix, branch, label));

        let children: &[Value<'source>] = match value {
            Value::TextGroup(v) | Value::ExpandableGroup(v) => v,
            _ => return,
        };

        let next_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let len = children.len();
        for (i, child) in children.iter().enumerate() {
            self.print_node(child, &next_prefix, i == len - 1, out);
        }
    }
}
