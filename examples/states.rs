use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_schedules_ext::prelude::*;
use bevy_state::{app::StatesPlugin, prelude::*};

#[derive(ScheduleLabel, States, Debug, Default, Hash, PartialEq, Eq, Clone)]
enum GeneralState {
	#[default]
	Menu,
	Game,
}

#[derive(ScheduleLabel, States, Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
	Ingame,
	Paused,
}

fn main() {
	let mut app = App::new();

	// Adds the state transition schedule - available by default when using bevy normally
	app.add_plugins(StatesPlugin);

	// Add the general state schedule to update, using the default value
	app.init_state_to_schedule::<GeneralState>(Update);

	// Add the game state schedule to the general schedule with a specific value
	app.insert_state_to_schedule(GeneralState::Game, GameState::Paused);

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
