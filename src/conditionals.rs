use std::error::Error;
use handlebars::{Handlebars, RenderError};
use serde_json::json;
use crate::random_data_creator::DataType;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
pub(crate) struct Conditional {
    pub(crate) name: String,
    pub(crate) input_type: DataType
}

impl Conditional {
    pub fn print_with_data(&self, input: &Vec<String>, file_name_prefix: &String) {
        match self.name.as_str() {
            "if" => {
                self.print_if_statements(input, file_name_prefix).expect("Couldn't print 'if' statements");
            }
            "assert" => {
                self.print_assert_statements(input, file_name_prefix).expect("Couldn't print 'assert' statements");
            }
            _ => {}
        }
    }

    pub fn print_if_statements(&self, input: &Vec<String>, file_name_prefix: &String) -> Result<(), Box<dyn Error>> {
        let mut reg = Handlebars::new();
        let mut file = File::create(format!("/Users/asv/Projects/noir-projects/noir-fuzzer/results/{}-if.rust", file_name_prefix)).expect("creating new file failed.");

        for eachInput in input {
            let mut data;
            match self.input_type {
                DataType::U32 => {
                    data = reg.render_template("\nif checkVariable == {{input1}} {println!(\"{}\", {{input1}});}", &json!({"input1": eachInput })).expect("Couldn't render if for U32 data.") + "\n";
                }
                DataType::String => {
                    data = reg.render_template("\nif checkVariable == \"{{input1}}\" {println!(\"{}\", \"{{input1}}\");}", &json!({"input1": eachInput })).expect("couldn't render String data type") + "\n";
                }
            }

            match file.write_all(data.as_bytes()) {
                Ok(_) => {}
                Err(_) => {println!("couldn't write data for if statements");}
            }
        }
        Ok(())
    }

    pub fn print_assert_statements(&self, input: &Vec<String>, file_name_prefix: &String) -> Result<(), Box<dyn Error>> {
        let mut reg = Handlebars::new();
        let mut file = File::create(format!("/Users/asv/Projects/noir-projects/noir-fuzzer/results/{}-assert.rust", file_name_prefix)).expect("creating new file failed.");

        for eachInput in input {
            let mut data;
            match self.input_type {
                DataType::U32 => {
                    data = reg.render_template("assert(checkVariable == {{input1}});", &json!({"input1": eachInput })).expect("Couldn't render assert for u32 data.") + "\n";
                }
                DataType::String => {
                    data = reg.render_template("assert(checkVariable == \"{{input1}}\");", &json!({"input1": eachInput })).expect("Couldn't render assert for String data.") + "\n";
                }
            }
            match file.write_all(data.as_bytes()) {
                Ok(_) => {}
                Err(_) => {println!("couldn't write data for assert statements");}
            }
        }
        Ok(())
    }
}