use dep::std;

fn main() {

	for test in 0..173{

	let a = [true, false];
	if a[0] {std::println(a[0]);}

	assert(a[0] == true);

	if !a[1] {std::println(false);}

	assert(a[1] == false);

	}
}