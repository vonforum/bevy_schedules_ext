use bevy_ecs::prelude::*;

use bevy_schedules_ext::prelude::*;

// Define our schedules
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct A;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum AChildren {
	A,
	B,
}

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct Update;

fn main() {
	let mut world = World::new();

	let mut update = Schedule::new(Update);
	let mut parent_a = Schedule::new(A);
	let mut child_a = Schedule::new(AChildren::A);
	let mut child_b = Schedule::new(AChildren::B);

	// Create our schedule tree
	update.add_schedules(parent_a.label());
	parent_a.add_schedules((child_a.label(), child_b.label()));

	/* - Current schedule tree:
	  Update - + - A - + - AA
			           |
			           + - AB
	*/

	// Add example systems
	parent_a.add_systems(a);
	child_a.add_systems(aa);
	child_b.add_systems(ab);

	// Add our schedules to the world
	world.add_schedule(parent_a);
	world.add_schedule(child_a);
	world.add_schedule(child_b);

	// Run the parent schedule
	update.run(&mut world);
}

// Example systems

fn a() {
	println!("A");
}

fn aa() {
	println!("AA");
}

fn ab() {
	println!("AB");
}
