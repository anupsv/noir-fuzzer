use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fmt::format;

mod random_data_creator;
use strum::IntoEnumIterator;
use crate::random_data_creator::DataType;
use std::borrow::Cow;



struct Conditional {
    name: String,
    input_type: DataType
}

impl Conditional {
    pub fn print_with_data(&self, input: &Vec<String>) {
        match self.name.as_str() {
            "if" => {
                self.print_if_statements(input).expect("Couldn't print 'if' statements");
            }
            "assert" => {
                self.print_assert_statements(input).expect("Couldn't print 'assert' statements");
            }
            _ => {}
        }
    }

    pub fn print_if_statements(&self, input: &Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut reg = Handlebars::new();

        for eachInput in input {

            match self.input_type {
                DataType::U32 => {
                    println!(
                        "{}",
                        reg.render_template("\nif checkVariable == {{input1}} {println(\"{}\", {{input1}});}", &json!({"input1": eachInput }))?
                    );
                }
                DataType::String => {
                    println!(
                        "{}",
                        reg.render_template("\nif checkVariable == \"{{input1}}\" {println(\"{}\", {{input1}});}", &json!({"input1": eachInput }))?
                    );
                }
            }
        }
        Ok(())
    }

    pub fn print_assert_statements(&self, input: &Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut reg = Handlebars::new();

        for eachInput in input {

            match self.input_type {
                DataType::U32 => {
                    println!(
                        "{}",
                        reg.render_template("assert(checkVariable == {{input1}})", &json!({"input1": eachInput }))?
                    );
                }
                DataType::String => {
                    println!(
                        "{}",
                        reg.render_template("assert(checkVariable == \"{{input1}}\")", &json!({"input1": eachInput }))?
                    );
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let conditionals: [&str; 2] = ["if", "assert"];

    for eachConditional in conditionals {
        for eachVarType in random_data_creator::DataType::iter() {

            match eachConditional {
                "if" => {
                    let random_data = random_data_creator::generate_random(eachVarType, 5);
                    let conditional: Conditional = Conditional {
                        name: String::from("if"),
                        input_type: eachVarType
                    };
                    conditional.print_with_data(&random_data);
                }
                "assert" => {
                    let random_data = random_data_creator::generate_random(eachVarType, 5);
                    let conditional: Conditional = Conditional {
                        name: String::from("assert"),
                        input_type: eachVarType
                    };
                    conditional.print_with_data(&random_data);
                }
                _ => {}
            }
        }
    }
}
