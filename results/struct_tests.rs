fn main() {

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
    field1: 59873,
    field2: 59874,
    field3: false,
    field4: [5672, 38365, 57837, 33761, 11744],
    field5: [true, true, true],
    field6: 27165,
    field7: [38845, 39891, 45180, 51952],
    field8: false,
    field9: 44470,
    field10: [true, true],
};



fn recursive_sum(arr: &[u32], index: usize) -> u32 {
    if index == arr.len() {
        0
    } else {
        arr[index] + recursive_sum(arr, index + 1)
    }
}


let sum = recursive_sum(&rust_struct.field4, 0);
assert_eq!(sum, rust_struct.field4.iter().cloned().fold(0u32, |acc, x| acc + x));

}