module main

fn gimme_three_vals() (string, i32, bool) {
	return 'thing1', 5, true
}

fn main() {
	println(gimme_three_vals())
}
