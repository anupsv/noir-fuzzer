
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


fn recursive_sum(arr: [Field; 5], index: Field) -> Field {
    if index == 5 {
        0
    } else {
        arr[index] + recursive_sum(arr, index + 1)
    }
}

fn main() {

let noir_struct = NoirStruct {
    field1: 44965,
    field2: 43796,
    field3: false,
    field4: [7200, 62801, 3583, 57463, 40606],
    field5: [false, false, true],
    field6: 9788,
    field7: [31165, 42769, 37118, 54589],
    field8: false,
    field9: 25149,
    field10: [false, true],
};


let sum = recursive_sum(noir_struct.field4, 0);
let len = noir_struct.field4.len();
let mut expected_sum = 0;
for i in 0..len {
    expected_sum = expected_sum + noir_struct.field4[i];
}
assert(sum == expected_sum);

}