use std::error::Error;
use std::fs;
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
        let rust_file_name = format!("/Users/asv/Projects/noir-projects/noir-fuzzer/results/{}-if-{}.rs", file_name_prefix, self.input_type);
        let noir_file_name = format!("/Users/asv/Projects/noir-projects/noir-fuzzer/results/{}-if-{}.nr", file_name_prefix, self.input_type);
        let mut file_writer_rust = File::create(rust_file_name.clone()).expect("creating new file failed.");
        let mut file_writer_noir = File::create(noir_file_name.clone()).expect("creating new file failed.");

        let mut data = "".to_string();

        for eachInput in input {
            match self.input_type {
                DataType::U32 => {
                    // data = reg.render_template("\n\tif {{input1}} == {{input1}} {println!(\"{}\", {{input1}});}", &json!({"input1": eachInput })).expect("Couldn't render if for U32 data.") + "\n";
                    data.push_str(&*(reg.render_template("\n\tif {{input1}} == {{input1}} {println!(\"{}\", {{input1}});}", &json!({"input1": eachInput })).expect("Couldn't render if for U32 data.") + "\n"));
                }
                DataType::Bool => {
                    // data = reg.render_template("\n\tif {{input1}} {println!(\"{}\", {{input1}});}", &json!({"input1": eachInput })).expect("Couldn't render if for U32 data.") + "\n";
                    data.push_str(&*(reg.render_template("\n\tif {{input1}} {println!(\"{}\", {{input1}});}", &json!({"input1": eachInput })).expect("Couldn't render if for U32 data.") + "\n"));
                }
                _ => {}
            }
        }

        match file_writer_rust.write_all(data.as_bytes()) {
            Ok(_) => {}
            Err(_) => {println!("couldn't write data for if statements for rust file");}
        }

        match file_writer_noir.write_all(data.as_bytes()) {
            Ok(_) => {}
            Err(_) => {println!("couldn't write data for if statements for rust file");}
        }

        // optimize this later, for now it works.
        self.post_write_process(&rust_file_name);
        self.post_write_process(&noir_file_name);
        Ok(())
    }

    pub fn print_assert_statements(&self, input: &Vec<String>, file_name_prefix: &String) -> Result<(), Box<dyn Error>> {
        let mut reg = Handlebars::new();
        let rust_file_name = format!("/Users/asv/Projects/noir-projects/noir-fuzzer/results/{}-assert-{}.rs", file_name_prefix, self.input_type);
        let noir_file_name = format!("/Users/asv/Projects/noir-projects/noir-fuzzer/results/{}-assert-{}.nr", file_name_prefix, self.input_type);
        let mut file_writer_rust = File::create(rust_file_name.clone()).expect("creating new file failed.");
        let mut file_writer_noir = File::create(noir_file_name.clone()).expect("creating new file failed.");

        for eachInput in input {
            let mut data;
            match self.input_type {
                DataType::U32 => {
                    data = reg.render_template("\tassert({{input1}} == {{input1}});", &json!({"input1": eachInput })).expect("Couldn't render assert for u32 data.") + "\n";
                }
                DataType::String => {
                    data = reg.render_template("\tassert(\"{{input1}}\" == \"{{input1}}\");", &json!({"input1": eachInput })).expect("Couldn't render assert for String data.") + "\n";
                }
                DataType::Bool => {
                    data = reg.render_template("\tassert({{input1}} == {{input1}});", &json!({"input1": eachInput })).expect("Couldn't render assert for Bool data.") + "\n";
                }
            }
            match file_writer_rust.write_all(data.as_bytes()) {
                Ok(_) => {}
                Err(_) => {println!("couldn't write data for assert statements for rust file");}
            }
            match file_writer_noir.write_all(data.as_bytes()) {
                Ok(_) => {}
                Err(_) => {println!("couldn't write data for assert statements for noir file");}
            }
        }

        // optimize this later, for now it works.
        self.post_write_process(&rust_file_name);
        self.post_write_process(&noir_file_name);
        Ok(())
    }

    pub fn post_write_process(&self, file_path: &String) {
        let mut content = String::new();
        let mut file = match fs::File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open the file: {}", e);
                return;
            }
        };

        if let Err(e) = file.read_to_string(&mut content) {
            eprintln!("Failed to read the file: {}", e);
            return;
        }
        let wrapped_content = format!("fn main() {{\n{}\n}}", content);

        let mut file = match fs::File::create(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open the file for writing: {}", e);
                return;
            }
        };
        if let Err(e) = file.write_all(wrapped_content.as_bytes()) {
            eprintln!("Failed to write to the file: {}", e);
        }
    }
}