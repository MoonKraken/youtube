module main

struct Spaceship {
	max_warp u8     [required]
	name     string
}

fn main() {
	s := Spaceship{
		max_warp: 9
		name: 'Enterprise'
	}

	println(s)
}
