
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

fn recursive_sum_with_pow(arr: [Field; 5], index: Field) -> (Field, Field) {
    let pow2 = 2;
    if index == 5 {
        (0, pow2.pow_32(index))
    } else {
        let (sum, pow_val) = recursive_sum_with_pow(arr, index + 1);
        (arr[index] + sum, pow2.pow_32(index) + pow_val)
    }
}

fn multiply_by2(x: &mut Field) {
    *x = *x * 2;
}

fn main() {

let noir_struct = NoirStruct {
    field1: 41252,
    field2: 24492,
    field3: true,
    field4: [11253, 59586, 6705, 19191, 17944],
    field5: [false, false, false],
    field6: 12194,
    field7: [62083, 27223, 54759, 40538],
    field8: true,
    field9: 12865,
    field10: [false, true],
};


let mut field6 = noir_struct.field6;
multiply_by2(&mut field6);
assert(field6 == noir_struct.field6*2);

}