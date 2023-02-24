module main

interface Person {
	age u8
}

fn (p Person) is_adult() bool {
	return p.age >= 18
}

struct Programmer {
	age         u8
	loc_per_day u32
}

fn main() {
	p := Programmer{
		age: 30
		loc_per_day: 100000
	}

	println(Person(p).is_adult())
	person_param(p)
}

fn person_param(p Person) {
	println('Dont try to pass in a Programmer, they arent people!')
}
