use std::error::Error;
use handlebars::Handlebars;
use serde_json::json;
use crate::random_data_creator::DataType;

pub(crate) struct Conditional {
    pub(crate) name: String,
    pub(crate) input_type: DataType
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