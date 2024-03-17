use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

pub mod prelude {
	pub use bevy_ecs::schedule::States;

	#[cfg(feature = "app_ext")]
	pub use super::app_ext::AppExt as StatesAppExt;
}

pub fn create_run_state_system<S: States + ScheduleLabel>() -> impl FnMut(&mut World) {
	move |world: &mut World| {
		let state = {
			let Some(state) = world.get_resource::<State<S>>() else {
				return;
			};

			state.intern()
		};

		world.run_schedule(state);
	}
}

#[cfg(feature = "app_ext")]
pub mod app_ext {
	use bevy_app::prelude::*;

	use super::*;

	pub trait AppExt {
		fn init_schedule_state<S: FromWorld + States + ScheduleLabel>(&mut self) -> &mut Self;

		fn insert_schedule_state<S: States + ScheduleLabel>(&mut self, state: S) -> &mut Self;

		fn add_state_to_schedule<S: States + ScheduleLabel>(
			&mut self,
			parent: impl ScheduleLabel,
		) -> &mut Self;
	}

	impl AppExt for App {
		fn init_schedule_state<S: FromWorld + States + ScheduleLabel>(&mut self) -> &mut Self {
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
			self.add_systems(parent, create_run_state_system::<S>())
		}
	}
}
