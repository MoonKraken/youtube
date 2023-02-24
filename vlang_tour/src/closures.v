module main

fn main() {
	n := 3
	a := fn [n] () int {
		return n + 1
	}

	println(a())
}
