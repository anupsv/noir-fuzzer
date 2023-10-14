use handlebars::Handlebars;
use rand::Rng;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::function_generator::FunctionGenerators;

pub struct StructGenerator<'a> {
    handlebars: Handlebars<'a>,
}

impl<'a> StructGenerator<'a> {
    pub(crate) fn new() -> Self {
        StructGenerator {
            handlebars: Handlebars::new(),
        }
    }

    pub(crate) fn generate_checks(&self, language: &str) -> String {
        let function_generator = FunctionGenerators::new();
        let recursive_sum = function_generator.generate_recursive_sum(language, 5);
        let recursive_sum_pow = function_generator.generate_recursive_sum_with_pow(language, 5);
        let checks = match language {
            "rust" => {
                r#"
assert_eq!(recursive_sum(&rust_struct.field4, 0), rust_struct.field4.iter().cloned().fold(0u32, |acc, x| acc + x));
"#
            }
            "noir" => {
                r#"
let len = noir_struct.field4.len();
let mut expected_sum = 0;
for i in 0..len {
    expected_sum = expected_sum + noir_struct.field4[i];
}
assert(recursive_sum(noir_struct.field4, 0) == expected_sum);
assert(recursive_sum_pow(noir_struct.field4, 0).0 == expected_sum);
assert(recursive_sum_with_pow(noir_struct.field4, 0).1 == 63);
"#
            }
            _ => panic!("Unsupported language"),
        };

        if language == "rust" {
            let first_form = format!("{}\n{}", recursive_sum, recursive_sum_pow);
            format!("{}\n{}", first_form, checks)
        } else {
            checks.to_string()
        }
    }

    fn generate_random_values(&self) -> BTreeMap<&'static str, String> {
        let mut rng = rand::thread_rng();
        let mut values = BTreeMap::new();

        values.insert("field1", rng.gen::<u16>().to_string());
        values.insert("field2", rng.gen::<u16>().to_string());
        values.insert("field3", rng.gen::<bool>().to_string());
        values.insert("field4_0", rng.gen::<u16>().to_string());
        values.insert("field4_1", rng.gen::<u16>().to_string());
        values.insert("field4_2", rng.gen::<u16>().to_string());
        values.insert("field4_3", rng.gen::<u16>().to_string());
        values.insert("field4_4", rng.gen::<u16>().to_string());
        values.insert("field5_0", rng.gen::<bool>().to_string());
        values.insert("field5_1", rng.gen::<bool>().to_string());
        values.insert("field5_2", rng.gen::<bool>().to_string());
        values.insert("field6", rng.gen::<u16>().to_string());
        values.insert("field7_0", rng.gen::<u16>().to_string());
        values.insert("field7_1", rng.gen::<u16>().to_string());
        values.insert("field7_2", rng.gen::<u16>().to_string());
        values.insert("field7_3", rng.gen::<u16>().to_string());
        values.insert("field8", rng.gen::<bool>().to_string());
        values.insert("field9", rng.gen::<u16>().to_string());
        values.insert("field10_0", rng.gen::<bool>().to_string());
        values.insert("field10_1", rng.gen::<bool>().to_string());

        values
    }

    fn generate_struct(&self, language: &str) -> (String, Option<String>) {
        let mut handlebars = Handlebars::new();
        let values = self.generate_random_values();

        let (template, init_template) = match language {
            "rust" => {
                (r#"
struct RustStruct {
    field1: u32,
    field2: u32,
    field3: bool,
    field4: [u32; 5],
    field5: [bool; 3],
    field6: u32,
    field7: [u32; 4],
    field8: bool,
    field9: u32,
    field10: [bool; 2],
}
let rust_struct = RustStruct {
    field1: {{field1}},
    field2: {{field2}},
    field3: {{field3}},
    field4: [{{field4_0}}, {{field4_1}}, {{field4_2}}, {{field4_3}}, {{field4_4}}],
    field5: [{{field5_0}}, {{field5_1}}, {{field5_2}}],
    field6: {{field6}},
    field7: [{{field7_0}}, {{field7_1}}, {{field7_2}}, {{field7_3}}],
    field8: {{field8}},
    field9: {{field9}},
    field10: [{{field10_0}}, {{field10_1}}],
};

{{checks}}
"#, None)
            }
            "noir" => {
                (r#"
struct NoirStruct {
    field1: Field,
    field2: Field,
    field3: bool,
    field4: [Field; 5],
    field5: [bool; 3],
    field6: Field,
    field7: [Field; 4],
    field8: bool,
    field9: Field,
    field10: [bool; 2],
}

"#, Some(r#"
let noir_struct = NoirStruct {
    field1: {{field1}},
    field2: {{field2}},
    field3: {{field3}},
    field4: [{{field4_0}}, {{field4_1}}, {{field4_2}}, {{field4_3}}, {{field4_4}}],
    field5: [{{field5_0}}, {{field5_1}}, {{field5_2}}],
    field6: {{field6}},
    field7: [{{field7_0}}, {{field7_1}}, {{field7_2}}, {{field7_3}}],
    field8: {{field8}},
    field9: {{field9}},
    field10: [{{field10_0}}, {{field10_1}}],
};
{{checks}}
"#))
            }
            _ => panic!("Unsupported language"),
        };

        let struct_code = self.handlebars.render_template(template, &values).unwrap();
        let init_code = init_template.map(|t| self.handlebars.render_template(t, &values).unwrap());

        (struct_code, init_code)
    }
    fn write_to_file(&self, language: &str, struct_code: &str, init_code: &str, checks_code: &str) -> std::io::Result<()> {
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
        let filename = format!("results/{}_struct_tests.{}", timestamp, extension);

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
            file.write_all(b"\nfn main() {\n")?;
            file.write_all(init_code.as_bytes())?;
            file.write_all(checks_code.as_bytes())?;
            file.write_all(b"\n}")?;
        } else {
            // For Rust, wrap everything inside the main function
            file.write_all(b"fn main() {\n")?;
            file.write_all(struct_code.as_bytes())?;
            file.write_all(init_code.as_bytes())?;
            file.write_all(checks_code.as_bytes())?;
            file.write_all(b"\n}")?;
        }

        println!("Written to file: {}", filename);

        Ok(())
    }


    pub(crate) fn create_checks_and_write(&self) -> std::io::Result<()> {
        for &language in &["rust", "noir"] {
            let (struct_code, init_code_opt) = self.generate_struct(language);
            let init_code = init_code_opt.unwrap_or_default();
            let checks_code = self.generate_checks(language);
            self.write_to_file(language, &struct_code, &init_code, &checks_code)?;
        }
        Ok(())
    }
}