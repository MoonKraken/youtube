module main

fn main() {
	mut a := [1, 2, 3]
	a << 4
	println(a)
	println(a.len)
	println(a.cap)
}
