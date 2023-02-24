module main

struct Spaceship {
	max_warp u8     [required]
	name     string
}

fn (s Spaceship) goto_max_warp() {
	println('Traveling at warp ${s.max_warp}')
}

fn main() {
	s := Spaceship{
		max_warp: 9
		name: 'Enterprise'
	}

	s.goto_max_warp()
}
