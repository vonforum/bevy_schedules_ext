use bevy_ecs::{
	prelude::*,
	schedule::{InternedScheduleLabel, ScheduleLabel},
};

pub mod prelude {
	pub use bevy_ecs::prelude::States;

	#[cfg(feature = "app_ext")]
	pub use super::app_ext::AppExt as StatesAppExt;
	pub use super::NextScheduleState;
}

pub struct StateContainer<S: ScheduleLabel> {
	pub inner: InternedScheduleLabel,
	phantom: std::marker::PhantomData<S>,
}

impl<S: Default + ScheduleLabel> Default for StateContainer<S> {
	fn default() -> Self {
		Self {
			inner: S::default().intern(),
			phantom: std::marker::PhantomData,
		}
	}
}

impl<S: ScheduleLabel> From<InternedScheduleLabel> for StateContainer<S> {
	fn from(inner: InternedScheduleLabel) -> Self {
		Self {
			inner,
			phantom: std::marker::PhantomData,
		}
	}
}

pub fn create_run_state_system<S: ScheduleLabel>(
	label: InternedScheduleLabel,
) -> impl FnMut(&mut World) {
	use crate::containers::ScheduleContainers as Containers;

	move |world: &mut World| {
		// Remove the state from the container. This is so we don't break ownership rules.
		let state = {
			let mut container = world.resource_mut::<Containers<StateContainer<S>>>();
			container.inner.remove(&label).unwrap()
		};

		// Run the state
		world.run_schedule(state.inner);

		// Add the state back to the container
		// Unchecked, because we just removed it before
		world
			.resource_mut::<Containers<StateContainer<S>>>()
			.inner
			.insert_unique_unchecked(label, state);
	}
}

#[derive(Resource)]
pub struct NextScheduleState<S: ScheduleLabel>(pub Option<(InternedScheduleLabel, S)>);

impl<S: ScheduleLabel> Default for NextScheduleState<S> {
	fn default() -> Self {
		Self(None)
	}
}

impl<S: ScheduleLabel> NextScheduleState<S> {
	pub fn set<P: ScheduleLabel>(&mut self, parent: P, state: S) {
		self.0 = Some((parent.intern(), state));
	}
}

pub fn apply_state_transition<S: ScheduleLabel>(world: &mut World) {
	use crate::containers::ScheduleContainers as Containers;

	let Some(mut next_state_resource) = world.get_resource_mut::<NextScheduleState<S>>() else {
		return;
	};

	if let Some((parent, state)) = next_state_resource.bypass_change_detection().0.take() {
		next_state_resource.set_changed();

		// TODO: Send a relevant StateTransitionEvent
		world
			.resource_mut::<Containers<StateContainer<S>>>()
			.inner
			.insert(parent, state.intern().into());
	}
}

#[cfg(feature = "app_ext")]
pub mod app_ext {
	use bevy_app::prelude::*;

	use super::*;
	use crate::containers::*;

	pub trait AppExt {
		fn init_state_schedule<S: Default + ScheduleLabel>(
			&mut self,
			parent: impl ScheduleLabel,
		) -> &mut Self {
			self.insert_state_schedule(parent, S::default())
		}

		fn insert_state_schedule<P: ScheduleLabel, S: ScheduleLabel>(
			&mut self,
			parent: P,
			state: S,
		) -> &mut Self;
	}

	impl AppExt for App {
		fn insert_state_schedule<P: ScheduleLabel, S: ScheduleLabel>(
			&mut self,
			parent: P,
			state: S,
		) -> &mut Self {
			let label = parent.intern();
			let state_label = state.intern();

			self.init_resource::<NextScheduleState<S>>();

			self.world
				.insert_schedule_container::<StateContainer<S>>(label, state_label.into()); // Insert initial state
			if self
				.world
				.insert_schedule_container_system_marker::<StateContainer<S>>(label)
			{
				// Add the system to run the state if not yet present
				self.add_systems(label, create_run_state_system::<S>(label));
			}

			self.add_systems(StateTransition, apply_state_transition::<S>);

			self
		}
	}
}
