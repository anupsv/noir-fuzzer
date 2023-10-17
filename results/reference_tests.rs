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
    field1: 10484,
    field2: 16373,
    field3: true,
    field4: [52832, 58444, 53655, 54753, 42506],
    field5: [true, true, true],
    field6: 47191,
    field7: [11235, 38135, 29938, 20908],
    field8: false,
    field9: 59994,
    field10: [false, false],
};



fn multiply_by2(x: &mut u32) {
    *x = *x * 2;
}

let mut field6 = rust_struct.field6;
multiply_by2(&mut field6);
assert_eq!(field6, rust_struct.field6*2);

}