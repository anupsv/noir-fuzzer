use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use handlebars::Handlebars;
use crate::function_generator::FunctionGenerators;
use crate::struct_generator::StructGenerator;

pub(crate) struct ReferenceGenerator<'a> {
    handlebars: Handlebars<'a>,
}

impl<'a> ReferenceGenerator<'a> {
    pub(crate) fn new() -> Self {
        ReferenceGenerator {
            handlebars: Handlebars::new(),
        }
    }

    fn generate_test_function(&self, language: &str) -> String {
        let template = match language {
            "rust" => {
                r#"
fn multiply_by2(x: &mut u32) {
    *x = *x * 2;
}
"#
            }
            "noir" => {
                r#"
fn multiply_by2(x: &mut Field) {
    *x = *x * 2;
}
"#
            }
            _ => panic!("Unsupported language"),
        };
        template.to_string()
    }

    fn generate_checks(&self, language: &str) -> String {
        let checks_template = match language {
            "rust" => {
                r#"
let mut field6 = rust_struct.field6;
multiply_by2(&mut field6);
assert_eq!(field6, rust_struct.field6*2);
"#
            }
            "noir" => {
                r#"
let mut field6 = noir_struct.field6;
multiply_by2(&mut field6);
assert(field6 == noir_struct.field6*2);
"#
            }
            _ => panic!("Unsupported language"),
        };

        checks_template.to_string()
    }


    fn write_to_file(&self, language: &str, struct_code: &str, init_code: &str, checks_code: &str, test_functions: &str) -> std::io::Result<()> {
        // Get the current Unix timestamp in seconds
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();

        // Determine the file extension based on the language
        let extension = match language {
            "rust" => "rs",
            "noir" => "nr",
            _ => panic!("Unsupported language"),
        };

        // Create the filename
        let filename = format!("results/{}_reference_tests.{}", timestamp, extension);

        // Open the file (create or overwrite)
        let mut file = File::create(&filename)?;

        if language == "noir" {
            // For Noir, place the struct outside the main function and initialization and checks inside
            file.write_all(struct_code.as_bytes())?;
            let function_generator = FunctionGenerators::new();
            let recursive_sum = function_generator.generate_recursive_sum(language, 5);
            let recursive_sum_pow = function_generator.generate_recursive_sum_with_pow(language, 5);
            file.write_all(recursive_sum.as_bytes())?;
            file.write_all(recursive_sum_pow.as_bytes())?;
            file.write_all(test_functions.as_bytes())?;
            file.write_all(b"\nfn main() {\n")?;
            file.write_all(init_code.as_bytes())?;
            file.write_all(checks_code.as_bytes())?;
            file.write_all(b"\n}")?;
        } else {
            // For Rust, wrap everything inside the main function
            file.write_all(b"fn main() {\n")?;
            file.write_all(struct_code.as_bytes())?;
            file.write_all(test_functions.as_bytes())?;
            file.write_all(init_code.as_bytes())?;
            file.write_all(checks_code.as_bytes())?;
            file.write_all(b"\n}")?;
        }

        println!("Written to file: {}", filename);

        Ok(())
    }

    pub fn create_checks_and_write(&self) -> std::io::Result<()> {

        let struct_generator = StructGenerator::new();

        for &language in &["rust", "noir"] {
            let (struct_code, init_code_opt) = struct_generator.generate_struct(language);
            let init_code = init_code_opt.unwrap_or_default();
            let checks_code = self.generate_checks(language);
            let test_functions = self.generate_test_function(language);
            self.write_to_file(language, &struct_code, &init_code, &checks_code, &test_functions)?;
        }
        Ok(())
    }
}

