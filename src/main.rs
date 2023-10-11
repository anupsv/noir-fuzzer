use std::error::Error;
mod random_data_creator;
mod conditionals;
use strum::IntoEnumIterator;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::random_data_creator::DataType;


fn main() {
    let conditionals: [&str; 2] = ["if", "assert"];
    let file_name_prefix: String = SystemTime::now().duration_since(UNIX_EPOCH).expect("guess we can go to the past huh!").as_secs().to_string();

    for eachConditional in conditionals {
        for eachVarType in DataType::iter() {
            let random_data = random_data_creator::generate_random(eachVarType, 2000);
            match eachConditional {
                "if" => {
                    if eachVarType == DataType::String {
                        continue
                    }
                    let conditional: conditionals::Conditional = conditionals::Conditional {
                        name: String::from("if"),
                        input_type: eachVarType
                    };
                    conditional.print_with_data(&random_data, &file_name_prefix);
                }
                "assert" => {
                    let conditional: conditionals::Conditional = conditionals::Conditional {
                        name: String::from("assert"),
                        input_type: eachVarType
                    };
                    conditional.print_with_data(&random_data, &file_name_prefix);
                }
                _ => {}
            }
        }
    }
}
