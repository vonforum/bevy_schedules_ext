use bevy_ecs::schedule::ScheduleLabel;
use bevy_app::prelude::*;
use bevy_schedules_mod::*;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct A;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct B;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct AA;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct AB;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct BA;

fn main() {
	let mut app = App::new();

	app.add_schedules(Update, (A, B));
	app.add_schedules(A, (AA, AB));
	app.add_schedules(B, BA);

	app.add_systems(A, || {
		println!("A");
	});

	app.add_systems(B, || {
		println!("B");
	});

	app.add_systems(AA, || {
		println!("AA");
	});

	app.add_systems(AB, || {
		println!("AB");
	});

	app.add_systems(BA, || {
		println!("BA");
	});

	app.run();
}

