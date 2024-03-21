//! Extends Bevy to allow using states as schedules.

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

/// Exports the state schedules app extension if the feature is enabled and re-exports Bevy's [`States`].
pub mod prelude {
	pub use bevy_ecs::schedule::States;

	#[cfg(feature = "app_ext")]
	pub use super::app_ext::AppExt as StatesAppExt;
}

/// Helper to create a system to run a state schedule.
pub fn create_run_state_system<S: States + ScheduleLabel>() -> impl FnMut(&mut World) {
	move |world: &mut World| {
		let state = {
			let Some(state) = world.get_resource::<State<S>>() else {
				return;
			};

			// run_schedule also calls intern(), but we need ownership from the resource
			state.intern()
		};

		world.run_schedule(state);
	}
}

/// Adds methods to the [`App`] type for working with state schedules.
#[cfg(feature = "app_ext")]
pub mod app_ext {
	use bevy_app::prelude::*;

	use super::*;

	/// Adds methods for working with state schedules.
	pub trait AppExt {
		/// Initializes a [`State`] schedule with standard starting values.
		///
		/// If the state already exists, nothing happens.
		///
		/// Also adds everything that calling [`App::init_state`] would add.
		fn init_schedule_state<S: FromWorld + States + ScheduleLabel>(&mut self) -> &mut Self;

		/// Inserts a specific [`State`] schedule into the [`App`] and
		/// overrides any existing state of the same type.
		///
		/// Also adds everything that calling [`App::insert_state`] would add.
		fn insert_schedule_state<S: States + ScheduleLabel>(&mut self, state: S) -> &mut Self;

		/// Adds a state to a schedule. The state needs to have been created already either
		/// through [init_schedule_state](`App::init_schedule_state`) or
		/// [insert_schedule_state](`App::insert_schedule_state`).
		///
		/// # Examples
		///
		/// ```
		/// # use bevy_app::prelude::*;
		/// # use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
		/// #
		/// # use bevy_schedules_ext::prelude::*;
		/// #
		/// # #[derive(ScheduleLabel, States, Debug, Default, Hash, PartialEq, Eq, Clone)]
		/// # enum State {
		/// #     #[default]
		/// #     Loading,
		/// #     Playing,
		/// # };
		/// #
		/// # let mut app = App::new();
		/// #
		/// app.init_schedule_state::<State>();
		/// app.add_state_to_schedule::<State>(Update);
		/// ```
		fn add_state_to_schedule<S: States + ScheduleLabel>(
			&mut self,
			parent: impl ScheduleLabel,
		) -> &mut Self;
	}

	impl AppExt for App {
		fn init_schedule_state<S: FromWorld + States + ScheduleLabel>(&mut self) -> &mut Self {
			// init_schedule needs a value
			let schedule = S::from_world(&mut self.world);
			self.init_state::<S>().init_schedule(schedule)
		}

		fn insert_schedule_state<S: States + ScheduleLabel>(&mut self, state: S) -> &mut Self {
			let schedule = state.intern();
			self.insert_state(state).init_schedule(schedule)
		}

		fn add_state_to_schedule<S: States + ScheduleLabel>(
			&mut self,
			parent: impl ScheduleLabel,
		) -> &mut Self {
			// Add the system to run the state
			self.add_systems(parent, create_run_state_system::<S>())
		}
	}
}
