use bevy_app::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

use bevy_mod_schedules::prelude::*;

// - Define our schedules
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
}

fn main() {
	let mut app = App::new();

	// - Create our schedule tree
	app.add_schedules(Update, (A, B)); // Chained schedules
	app.add_schedules(A, (AChildren::A, AChildren::B)); // Enums work also
	app.add_schedules(B, BA); // No need for tuples if there's only one schedule

	// - Add example systems
	app.add_systems(A, || {
		println!("A");
	});

	app.add_systems(B, || {
		println!("B");
	});

	app.add_systems(AChildren::A, || {
		println!("AA");
	});

	app.add_systems(AChildren::B, || {
		println!("AB");
	});

	app.add_systems(BA, || {
		println!("BA");
	});

	// - Reuse a schedule
	app.add_schedules(Update, A);

	/* - Current schedule tree:
	  Update - + - A - + - AA
			   |       |
			   +       + - AB
			   |
			   + - B - + - BA
			   |
			   + - A - + - AA
					   |
					   + - AB
	*/

	app.run();

	/* - Should print:
	A
	AA
	AB
	B
	BA
	A
	AA
	AB
	*/
}
