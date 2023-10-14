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

    pub(crate) fn generate_recursive_sum(&self, language: &str) -> String {
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
"#
            }
            "noir" => {
                r#"
fn recursive_sum(arr: [Field; 5], index: Field) -> Field {
    if index == arr.len() {
        0
    } else {
        arr[index] + recursive_sum(arr, index + 1)
    }
}
"#
            }
            _ => panic!("Unsupported language"),
        };

        let data: BTreeMap<&str, String>  = BTreeMap::new(); // No dynamic data to insert for this template
        self.handlebars.render_template(template, &data).unwrap()
    }
}

