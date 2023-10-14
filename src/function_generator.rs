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

    pub(crate) fn generate_recursive_sum_with_pow(&self, language: &str, array_size: usize) -> String {
        let template = match language {
            "rust" => {
                r#"
fn recursive_sum_with_pow(arr: &[u32], index: usize) -> (u32, u32) {
    if index == arr.len() {
        (0, 1 << index)
    } else {
        let (sum, pow_val) = recursive_sum_with_pow(arr, index + 1);
        (arr[index] + sum, 1 << index)
    }
}
"#.to_string()
            }
            "noir" => {
                format!(r#"
fn recursive_sum_with_pow(arr: [Field; {}], index: Field) -> (Field, Field) {{
    let pow2 = 2;
    if index == {} {{
        (0, pow2.pow_32(index))
    }} else {{
        let (sum, pow_val) = recursive_sum_with_pow(arr, index + 1);
        (arr[index] + sum, pow2.pow_32(index) + pow_val)
    }}
}}
"#, array_size, array_size)
            }
            _ => panic!("Unsupported language"),
        };
        template
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

