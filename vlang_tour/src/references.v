module main

struct Person {
	age u8 [required]
}

fn is_adult(p &Person) bool {
	return p.age >= 18
}

fn main() {
	p := Person{
		age: 30
	}

	println(is_adult(p))
}
