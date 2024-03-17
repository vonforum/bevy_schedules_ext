use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_schedules_ext::prelude::*;

#[derive(ScheduleLabel, States, Debug, Default, Hash, PartialEq, Eq, Clone)]
enum GeneralState {
	#[default]
	Menu,
	Game,
}

#[derive(ScheduleLabel, States, Debug, Default, Hash, PartialEq, Eq, Clone)]
enum GameState {
	#[default]
	Ingame,
	Paused,
}

fn main() {
	let mut app = App::new();

	// Add the general state schedule to update, using the default value
	app.init_schedule_state::<GeneralState>();
	app.add_state_to_schedule::<GeneralState>(Update);

	// Add the game state schedule to the general schedule
	app.insert_schedule_state(GameState::Paused);
	app.add_state_to_schedule::<GameState>(GeneralState::Game);

	// Add the systems to the state schedules
	app.add_systems(GeneralState::Menu, menu);
	app.add_systems(GeneralState::Game, game);
	app.add_systems(GameState::Ingame, ingame);
	app.add_systems(GameState::Paused, paused);

	// Run the app a few times to see state transitions
	for _ in 0..3 {
		app.update();
	}
}

// Example systems
fn menu(mut next_state: ResMut<NextState<GeneralState>>) {
	println!("Menu");
	next_state.set(GeneralState::Game);
}

fn game() {
	print!("Game");
}

fn ingame() {
	println!(" (ingame)");
}

fn paused(mut next_state: ResMut<NextState<GameState>>) {
	println!(" (paused)");
	next_state.set(GameState::Ingame);
}
