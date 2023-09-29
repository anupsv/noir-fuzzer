extern crate rand;

use std::iter;
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::any::Any;
use std::any::TypeId;
#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub(crate) enum DataType {
    U32,
    String
}
pub(crate) fn generate_random(data_type: DataType, count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut results = Vec::new();

    match data_type {
        DataType::U32 => {
            for _ in 0..count {
                let random_u32: u32 = rng.gen_range(0..u32::MAX);
                results.push(random_u32.to_string());
            }
        }
        DataType::String => {
            for _ in 0..count {
                let random_string: String = iter::repeat(())
                    .map(|_| rng.sample(rand::distributions::Alphanumeric))
                    .map(char::from)
                    .take(30)
                    .collect();
                results.push(random_string);
            }
        }
    }
    results
}

pub(crate) fn vec_to_json(inputs: Vec<String>) -> serde_json::Value {
    let mut json_map = serde_json::Map::new();

    for (index, value) in inputs.iter().enumerate() {
        let key = format!("input{}", index + 1);
        json_map.insert(key, serde_json::Value::String(value.clone()));
    }

    serde_json::Value::Object(json_map)
}

pub(crate) fn is_vec_u32<T: ?Sized + Any>(_: &T) -> bool {
    TypeId::of::<T>() == TypeId::of::<Vec<u32>>()
}

pub(crate) fn is_vec_string<T: ?Sized + Any>(_: &T) -> bool {
    TypeId::of::<T>() == TypeId::of::<Vec<String>>()
}