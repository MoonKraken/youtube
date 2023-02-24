module main

fn supported_types(a bool, b i16, c i32) i32 {
	if a {
		return b - c
	} else {
		return c - b
	}
}

fn main() {
	a := supported_types(false, 1, 2)
	println(a)
}
