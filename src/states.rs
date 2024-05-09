//! Extends Bevy to allow using states as schedules.

use bevy_ecs::{
	prelude::*,
	schedule::{FreelyMutableState, ScheduleLabel},
};

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
		/// Initializes a state to a schedule. The state also needs to implement [`ScheduleLabel`].
		/// See [`insert_state_to_schedule`] for usage.
		fn init_state_to_schedule<S: FreelyMutableState + ScheduleLabel + FromWorld>(
			&mut self,
			parent: impl ScheduleLabel,
		) -> &mut Self;

		/// Adds a state to a schedule. The state also needs to implement [`ScheduleLabel`].
		///
		/// # Examples
		///
		/// ```
		/// # use bevy_app::prelude::*;
		/// # use bevy_ecs::prelude::*;
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
		/// app.init_state_to_schedule::<State>(Update);
		/// ```
		fn insert_state_to_schedule<S: FreelyMutableState + ScheduleLabel>(
			&mut self,
			parent: impl ScheduleLabel,
			state: S,
		) -> &mut Self;
	}

	impl AppExt for App {
		fn init_state_to_schedule<S: FreelyMutableState + ScheduleLabel + FromWorld>(
			&mut self,
			parent: impl ScheduleLabel,
		) -> &mut Self {
			let state = S::from_world(self.world_mut());
			self.insert_state_to_schedule(parent, state)
		}

		fn insert_state_to_schedule<S: FreelyMutableState + ScheduleLabel>(
			&mut self,
			parent: impl ScheduleLabel,
			state: S,
		) -> &mut Self {
			self.insert_state(state);
			self.add_systems(parent, create_run_state_system::<S>())
		}
	}
}
