module main

fn add_urgency(s string) string {
	return s + '!'
}

fn main() {
	a := add_urgency('hello')
	println(a)
}
