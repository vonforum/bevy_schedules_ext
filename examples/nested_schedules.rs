use bevy_app::prelude::*;
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

use bevy_mod_schedules::prelude::*;

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
}

fn main() {
	let mut app = App::new();

	// Create our schedule tree
	app.add_schedules(Update, (A, B)); // Chained schedules
	app.add_schedules(A, (AChildren::A, AChildren::B)); // Enums work also
	app.add_schedules(B, BA); // No need for tuples if there's only one schedule

	// Add example systems
	app.add_systems(A, a);
	app.add_systems(B, b);
	app.add_systems(AChildren::A, aa);
	app.add_systems(AChildren::B, ab);
	app.add_systems(BA, ba);

	// Reuse a schedule
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

	println!("---");

	// Equivalent code using vanilla Bevy:
	sets();
}

// - Equivalent code in vanilla Bevy

// Define sets
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct ASet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct BSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct BASet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum AChildrenSet {
	A,
	B,
}

// Can't reuse sets, so we need to define them again
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct ASet2;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum AChildrenSet2 {
	A,
	B,
}

fn sets() {
	let mut app = App::new();

	// Create set tree
	app.configure_sets(Update, (ASet, BSet).chain());
	app.configure_sets(
		Update,
		(AChildrenSet::A, AChildrenSet::B).chain().in_set(ASet),
	);
	app.configure_sets(Update, BASet.in_set(BSet));

	// Add example systems
	app.add_systems(Update, a.in_set(ASet));
	app.add_systems(Update, b.in_set(BSet));
	app.add_systems(Update, aa.in_set(AChildrenSet::A));
	app.add_systems(Update, ab.in_set(AChildrenSet::B));
	app.add_systems(Update, ba.in_set(BASet));

	// "Reuse" a set
	app.configure_sets(Update, ASet2.after(BSet));
	app.configure_sets(
		Update,
		(AChildrenSet2::A, AChildrenSet2::B).chain().in_set(ASet2),
	);
	app.add_systems(Update, a.in_set(ASet2));
	app.add_systems(Update, aa.in_set(AChildrenSet2::A));
	app.add_systems(Update, ab.in_set(AChildrenSet2::B));

	app.run();
}

// - Example systems

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
