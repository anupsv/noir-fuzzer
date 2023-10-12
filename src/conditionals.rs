use std::error::Error;
use std::fs;
use handlebars::{Handlebars, RenderError};
use serde_json::json;
use crate::random_data_creator::DataType;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use rand::Rng;

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
            "array" => {
                self.print_array_statements(input, file_name_prefix).expect("Couldn't print 'array' statements");
            }
            _ => {}
        }
    }

    pub fn print_array_statements(&self, input: &Vec<String>, file_name_prefix: &String) -> Result<(), Box<dyn Error>> {
        let mut reg = Handlebars::new();
        let rust_file_name = format!("/Users/asv/Projects/noir-projects/noir-fuzzer/results/{}-array-{}.rs", file_name_prefix, self.input_type);
        let noir_file_name = format!("/Users/asv/Projects/noir-projects/noir-fuzzer/results/{}-array-{}.nr", file_name_prefix, self.input_type);
        let mut file_writer_rust = File::create(rust_file_name.clone()).expect("creating new file failed.");
        let mut file_writer_noir = File::create(noir_file_name.clone()).expect("creating new file failed.");

        let mut rust_data = "".to_string();
        let mut noir_data = "".to_string();



        match self.input_type {
            DataType::U32 => {
                let joined = input.join(", ");
                rust_data.push_str(&*(reg.render_template("\n\tlet mut a: [u32; {{input1}}] = [{{input0}}];", &json!({"input0": joined, "input1": input.len() })).expect("could render data for joined array list") + "\n"));
                noir_data.push_str(&*(reg.render_template("\n\tlet mut a = [{{input0}}];", &json!({"input0": joined })).expect("could render data for joined array list") + "\n"));
                let mut rng = rand::thread_rng();

                for i in 0..(input.len()){
                    let random_index = rng.gen_range(0..(input.len()));
                    rust_data.push_str(&*(reg.render_template("\n\tif a[{{input0}}] == {{input1}} {println!(\"{}\", format!(\"\\\"0x{:x}\\\"\", {{input1}} as u32));}", &json!({"input0": random_index, "input1": input[random_index] })).expect("Couldn't render if for U32 data in array.") + "\n"));
                    rust_data.push_str(&*(reg.render_template("\tassert!(a[{{input0}}] == {{input1}});", &json!({"input0": random_index, "input1": input[random_index] })).expect("Couldn't render assert for u32 data.") + "\n"));
                    noir_data.push_str(&*(reg.render_template("\n\tif a[{{input0}}] == {{input1}} {std::println({{input1}});}", &json!({"input0": random_index, "input1": input[random_index] })).expect("Couldn't render if for U32 data in array.") + "\n"));
                    noir_data.push_str(&*(reg.render_template("\n\tassert(a[{{input0}}] == {{input1}});", &json!({"input0": random_index, "input1": input[random_index] })).expect("Couldn't render if for U32 data in array.") + "\n"));
                }

                let random_index = rng.gen_range(0..(input.len()));
                rust_data.push_str(&*(reg.render_template("\n\ta[{{input0}}] = {{input1}};", &json!({"input0": random_index, "input1": input[input.len()-1]})).expect("Couldn't render if for U32 data in array.") + "\n"));
                noir_data.push_str(&*(reg.render_template("\n\ta[{{input0}}] = {{input1}};", &json!({"input0": random_index, "input1": input[input.len()-1] })).expect("Couldn't render if for U32 data in array.") + "\n"));

                rust_data.push_str(&*(reg.render_template("\n\tif a[{{input0}}] == {{input1}} {println!(\"{}\", format!(\"\\\"0x{:x}\\\"\", {{input0}} as u32));}", &json!({"input0": random_index, "input1": input[input.len()-1] })).expect("Couldn't render if for U32 data in array.") + "\n"));
                noir_data.push_str(&*(reg.render_template("\n\tif a[{{input0}}] == {{input1}} {std::println({{input1}});}", &json!({"input0": random_index, "input1": input[input.len()-1] })).expect("Couldn't render if for U32 data in array.") + "\n"));;

            }
            DataType::Bool => {
                rust_data.push_str(("\n\tlet a = [true, false];"));
                noir_data.push_str(("\n\tlet a = [true, false];"));

                rust_data.push_str(&*(reg.render_template("\n\tif {{input1}} {println!(\"{}\", {{input1}});}", &json!({"input1": "a[0]" })).expect("Couldn't render if for bool data.") + "\n"));
                noir_data.push_str(&*(reg.render_template("\n\tif {{input1}} {std::println({{input1}});}", &json!({"input1": "a[0]" })).expect("Couldn't render if for bool data.") + "\n"));
                rust_data.push_str(&*(reg.render_template("\n\tassert!({{input1}} == true);", &json!({"input1": "a[0]" })).expect("Couldn't render if for bool data.") + "\n"));
                noir_data.push_str(&*(reg.render_template("\n\tassert({{input1}} == true);", &json!({"input1": "a[0]" })).expect("Couldn't render if for bool data.") + "\n"));

                rust_data.push_str(&*(reg.render_template("\n\tif {{input1}} {println!(\"false\");}", &json!({"input1": "!a[1]" })).expect("Couldn't render if for bool data.") + "\n"));
                noir_data.push_str(&*(reg.render_template("\n\tif {{input1}} {std::println(false);}", &json!({"input1": "!a[1]" })).expect("Couldn't render if for bool data.") + "\n"));
                rust_data.push_str(&*(reg.render_template("\tassert!({{input1}} == false);", &json!({"input1": "a[1]" })).expect("Couldn't render if for bool data.") + "\n"));
                noir_data.push_str(&*(reg.render_template("\n\tassert({{input1}} == false);", &json!({"input1": "a[1]" })).expect("Couldn't render if for bool data.") + "\n"));
            }
            _ => {}
        }

        match file_writer_rust.write_all(rust_data.as_bytes()) {
            Ok(_) => {}
            Err(_) => {println!("couldn't write data for if statements for rust file");}
        }

        match file_writer_noir.write_all(noir_data.as_bytes()) {
            Ok(_) => {}
            Err(_) => {println!("couldn't write data for if statements for rust file");}
        }

        // optimize this later, for now it works.
        self.post_write_process(&rust_file_name);
        self.post_write_process(&noir_file_name);
        Ok(())
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
        let mut data = "".to_string();

        for eachInput in input {
            match self.input_type {
                DataType::U32 => {
                    data = reg.render_template("\tassert({{input1}} == {{input1}});", &json!({"input1": eachInput })).expect("Couldn't render assert for u32 data.") + "\n";
                }
                // DataType::String => {
                //     data = reg.render_template("\tassert(\"{{input1}}\" == \"{{input1}}\");", &json!({"input1": eachInput })).expect("Couldn't render assert for String data.") + "\n";
                // }
                DataType::Bool => {
                    data = reg.render_template("\tassert({{input1}} == {{input1}});", &json!({"input1": eachInput })).expect("Couldn't render assert for Bool data.") + "\n";
                }
                _ => {}
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

        let mut wrapped_content = format!("fn main() {{\n{}\n}}", content);

        if file_path.ends_with(".nr"){
            wrapped_content = format!("use dep::std;\n\nfn main() {{\n{}\n}}", content);
        }

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