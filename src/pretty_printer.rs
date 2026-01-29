use colored::{ColoredString, Colorize};

pub fn pretty_print_json(
    json_str: &str,
    indent_size: usize,
    use_colors: bool,
) -> anyhow::Result<Vec<ColoredString>> {

    let json_value = crate::parser::parse_json(json_str)?;

    let mut printer = PrettyPrinter::new(indent_size, use_colors);
    Ok(printer.pretty_print(&json_value))
}

enum ColorCategory {
    Key,
    Delimiter,
    String,
    Normal
}

pub struct PrettyPrinter {
    indent_size: usize,
    use_colors: bool,
    level: usize,
    start_line: bool,
    output: Vec<ColoredString>,
}

impl PrettyPrinter {
    pub fn new(indent_size: usize, use_colors: bool) -> Self {
        PrettyPrinter {
            indent_size,
            use_colors,
            level: 0,
            start_line: true,
            output: Vec::new(),
        }
    }

    pub fn pretty_print(&mut self, value: &crate::json_value::JsonValue) -> Vec<ColoredString> {
        self.output = Vec::new();
        self.level = 0;
        self.start_line = true;

        self.print_value(value);
        self.println("", ColorCategory::Normal); // Final newline

        self.output.clone()
    }

    fn print_value(&mut self, value: &crate::json_value::JsonValue) {
        match value {
            crate::json_value::JsonValue::Null => self.print("null", ColorCategory::Normal),
            crate::json_value::JsonValue::Bool(b) => self.print(&b.to_string(), ColorCategory::Normal),
            crate::json_value::JsonValue::Number(n) => self.print(&n.to_string(), ColorCategory::Normal),
            crate::json_value::JsonValue::String(s) => self.print(&format!("\"{}\"", s), ColorCategory::String),
            crate::json_value::JsonValue::Array(arr) => self.print_array(arr),
            crate::json_value::JsonValue::Object(names, members) => {
                self.print_object(names, members)
            }
        }
    }

    fn print_array(&mut self, arr: &Vec<crate::json_value::JsonValue>) {
        if arr.len() == 0 {
            self.print("[]", ColorCategory::Delimiter);
            return;
        }

        self.println("[", ColorCategory::Delimiter);
        self.indent();
        for (i, item) in arr.iter().enumerate() {
            self.print_value(&item);
            if i < arr.len() - 1 {
                self.println(",", ColorCategory::Delimiter);
            } else {
                self.println("", ColorCategory::Normal);
            }
        }
        self.dedent();
        self.print("]", ColorCategory::Delimiter);
    }

    fn print_object(
        &mut self,
        names: &Vec<String>,
        members: &std::collections::HashMap<String, crate::json_value::JsonValue>,
    ) {
        if names.len() == 0 {
            self.print("{}", ColorCategory::Delimiter);
            return;
        }

        self.println("{", ColorCategory::Delimiter);
        self.indent();
        for (i, name) in names.iter().enumerate() {
            self.print(&format!("\"{}\"", name), ColorCategory::Key);
            self.print(": ", ColorCategory::Delimiter);
            if let Some(value) = members.get(name) {
                self.print_value(value);
            }
            if i < names.len() - 1 {
                self.println(",", ColorCategory::Delimiter);
            } else {
                self.println("", ColorCategory::Delimiter);
            }
        }
        self.dedent();
        self.print("}", ColorCategory::Delimiter);
    }

    fn print(&mut self, text: &str, category: ColorCategory) {
        let mut text = ColoredString::from(text);
        if self.use_colors {
            text = match category {
                ColorCategory::Key => text.blue(),
                ColorCategory::Delimiter => text.yellow(),
                ColorCategory::String => text.green(),
                ColorCategory::Normal => text.normal(),
            };
        }

        if self.start_line {
            self.start_line = false;
            let indent_str = " ".repeat(self.indent_size * self.level);
            self.output.push(ColoredString::from(indent_str));
            self.output.push(text);
        } else {
            self.output.push(text);
        }
    }

    fn println(&mut self, text: &str, category: ColorCategory) {
        let mut line = String::from(text);
        line.push('\n');
        self.print(&line, category);
        self.start_line = true;
    }

    fn indent(&mut self) {
        self.level += 1;
    }

    fn dedent(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pretty_print_json() {
        let json = r#"{"name":"Thomas","age":59,"is_student":false,"courses":["Math","Physics"],"address":{"country":"Germany","state":"Lower Saxony","city":"Hannover","zip":"31000"}}"#;
        let expected_json = r#"{
    "name": "Thomas",
    "age": 59,
    "is_student": false,
    "courses": [
        "Math",
        "Physics"
    ],
    "address": {
        "country": "Germany",
        "state": "Lower Saxony",
        "city": "Hannover",
        "zip": "31000"
    }
}"#;

        let pretty_json = pretty_print_json(json, 4, false).unwrap();
        let pretty_json: String = pretty_json.iter().map(|cs| cs.to_string()).collect();

        assert_eq!(pretty_json, expected_json);
    }
}



