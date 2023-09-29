use std::error::Error;
mod random_data_creator;
mod conditionals;
use strum::IntoEnumIterator;





fn main() {
    let conditionals: [&str; 2] = ["if", "assert"];

    for eachConditional in conditionals {
        for eachVarType in random_data_creator::DataType::iter() {

            match eachConditional {
                "if" => {
                    let random_data = random_data_creator::generate_random(eachVarType, 5);
                    let conditional: conditionals::Conditional = conditionals::Conditional {
                        name: String::from("if"),
                        input_type: eachVarType
                    };
                    conditional.print_with_data(&random_data);
                }
                "assert" => {
                    let random_data = random_data_creator::generate_random(eachVarType, 5);
                    let conditional: conditionals::Conditional = conditionals::Conditional {
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
