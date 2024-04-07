use bevy_app::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_schedules_ext::prelude::*;

// Define our schedules
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct A;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct B;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct BA;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum AChildren {
	A,
	B,
	C,
}

fn main() {
	let mut app = App::new();

	// Create our schedule tree
	app.add_schedules(Update, (A, B)); // Chained schedules
	app.add_schedules(A, (AChildren::A, AChildren::B)); // Enums work also
	app.add_schedules(AChildren::A, AChildren::C); // Same type children
	app.add_schedules(B, BA); // No need for tuples if there's only one schedule

	// Reuse a schedule
	app.add_schedules(B, A);

	/* - Current schedule tree:
	  Update - + - A - + - AA
			   |       |
			   +       + - AB
			   |
			   + - B - + - BA
					   |
					   + - A - ..
	*/

	// Add example systems
	app.add_systems(A, a);
	app.add_systems(B, b);
	app.add_systems(AChildren::A, aa);
	app.add_systems(AChildren::B, ab);
	app.add_systems(BA, ba);

	app.run();
}

// Example systems

fn a() {
	println!("A");
}

fn b() {
	println!("B");
}

fn aa() {
	println!("AA");
}

fn ab() {
	println!("AB");
}

fn ba() {
	println!("BA");
}
