use std::error::Error;
mod random_data_creator;
mod conditionals;
mod struct_generator;
mod function_generator;

use strum::IntoEnumIterator;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::random_data_creator::DataType;

fn main() {
    let conditionals: [&str; 1] = ["array"];
    // let conditionals: [&str; 2] = ["if", "assert", "array"];
    // let file_name_prefix: String = SystemTime::now().duration_since(UNIX_EPOCH).expect("guess we can go to the past huh!").as_secs().to_string();
    //
    // for eachConditional in conditionals {
    //     for eachVarType in DataType::iter() {
    //         let random_data = random_data_creator::generate_random(eachVarType, 2000);
    //         match eachConditional {
    //             "if" => {
    //                 let conditional: conditionals::Conditional = conditionals::Conditional {
    //                     name: String::from("if"),
    //                     input_type: eachVarType
    //                 };
    //                 conditional.print_with_data(&random_data, &file_name_prefix);
    //             }
    //             "assert" => {
    //                 let conditional: conditionals::Conditional = conditionals::Conditional {
    //                     name: String::from("assert"),
    //                     input_type: eachVarType
    //                 };
    //                 conditional.print_with_data(&random_data, &file_name_prefix);
    //             }
    //             "array" => {
    //                 let conditional: conditionals::Conditional = conditionals::Conditional {
    //                     name: String::from("array"),
    //                     input_type: eachVarType
    //                 };
    //                 conditional.print_with_data(&random_data, &file_name_prefix);
    //             }
    //             "loop" => {
    //                 let conditional: conditionals::Conditional = conditionals::Conditional {
    //                     name: String::from("loop"),
    //                     input_type: eachVarType
    //                 };
    //                 conditional.print_with_data(&random_data, &file_name_prefix);
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    // structs tests
    let struct_generator = struct_generator::StructGenerator::new();
    let rust_code = struct_generator.create_checks_and_write();

}
