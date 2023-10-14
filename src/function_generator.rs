extern crate handlebars;

use handlebars::Handlebars;
use std::collections::BTreeMap;

pub struct FunctionGenerators<'a> {
    handlebars: Handlebars<'a>,
}

impl<'a> FunctionGenerators<'a> {
    pub(crate) fn new() -> Self {
        FunctionGenerators {
            handlebars: Handlebars::new(),
        }
    }

    pub(crate) fn generate_recursive_sum(&self, language: &str, array_size: usize) -> String {
        let template = match language {
            "rust" => {
                r#"
fn recursive_sum(arr: &[u32], index: usize) -> u32 {
    if index == arr.len() {
        0
    } else {
        arr[index] + recursive_sum(arr, index + 1)
    }
}
"#.to_string()
            }
            "noir" => {
                format!(r#"
fn recursive_sum(arr: [Field; {}], index: Field) -> Field {{
    if index == {} {{
        0
    }} else {{
        arr[index] + recursive_sum(arr, index + 1)
    }}
}}
"#, array_size, array_size).to_string()
            }
            _ => panic!("Unsupported language"),
        };

        template
    }
}

