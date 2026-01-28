pub fn pretty_print_json(
    json_str: &str,
    indent_size: usize,
) -> anyhow::Result<String> {

    let json_value = crate::parser::parse_json(json_str)?;

    let mut printer = PrettyPrinter::new(indent_size);
    Ok(printer.pretty_print(&json_value))
}

pub struct PrettyPrinter {
    indent_size: usize,
    level: usize,
    start_line: bool,
    output: String,
}

impl PrettyPrinter {
    pub fn new(indent_size: usize) -> Self {
        PrettyPrinter {
            indent_size,
            level: 0,
            start_line: true,
            output: String::new(),
        }
    }

    pub fn pretty_print(&mut self, value: &crate::json_value::JsonValue) -> String {
        self.output = String::new();
        self.level = 0;
        self.start_line = true;

        self.print_value(value);

        self.output.clone()
    }

    fn print_value(&mut self, value: &crate::json_value::JsonValue) {
        match value {
            crate::json_value::JsonValue::Null => self.print("null"),
            crate::json_value::JsonValue::Bool(b) => self.print(&b.to_string()),
            crate::json_value::JsonValue::Number(n) => self.print(&n.to_string()),
            crate::json_value::JsonValue::String(s) => self.print(&format!("\"{}\"", s)),
            crate::json_value::JsonValue::Array(arr) => self.print_array(arr),
            crate::json_value::JsonValue::Object(names, members) => {
                self.print_object(names, members)
            }
        }
    }

    fn print_array(&mut self, arr: &Vec<crate::json_value::JsonValue>) {
        if arr.len() == 0 {
            self.print("[]");
            return;
        }

        self.println("[");
        self.indent();
        for (i, item) in arr.iter().enumerate() {
            self.print_value(&item);
            if i < arr.len() - 1 {
                self.println(",");
            } else {
                self.println("");
            }
        }
        self.dedent();
        self.print("]");
    }

    fn print_object(
        &mut self,
        names: &Vec<String>,
        members: &std::collections::HashMap<String, crate::json_value::JsonValue>,
    ) {
        if names.len() == 0 {
            self.print("{}");
            return;
        }

        self.println("{");
        self.indent();
        for (i, name) in names.iter().enumerate() {
            self.print(&format!("\"{}\": ", name));
            if let Some(value) = members.get(name) {
                self.print_value(value);
            }
            if i < names.len() - 1 {
                self.println(",");
            } else {
                self.println("");
            }
        }
        self.dedent();
        self.print("}");
    }

    fn print(&mut self, text: &str) {
        if self.start_line {
            self.start_line = false;
            let indent_str = " ".repeat(self.indent_size * self.level);
            self.output += &format!("{}{}", indent_str, text);
        } else {
            self.output += text;
        }
    }

    fn println(&mut self, text: &str) {
        self.print(text);
        self.output += "\n";
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

        let pretty_json = pretty_print_json(json, 4).unwrap();
        assert_eq!(pretty_json, expected_json);
    }
}



