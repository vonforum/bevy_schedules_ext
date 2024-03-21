//! Extends Bevy's World with helpers for managing containers for schedules.

use bevy_ecs::{prelude::*, schedule::InternedScheduleLabel};
use bevy_utils::HashMap;

/// Exports the container world extension, plus the [`Container`] trait.
pub mod prelude {
	pub use super::{Container, ScheduleContainers, WorldExt as ContainersWorldExt};
}

/// A container of schedules that can be run.
pub trait Container: Send + Sync + 'static {
	/// Runs the container of schedules.
	fn run(&self, world: &mut World);
}

/// A resource containing a map of a parent schedule to a container of child schedule(s).
#[derive(Resource)]
pub struct ScheduleContainers<S: Container> {
	/// The map of parent schedules to containers.
	pub inner: HashMap<InternedScheduleLabel, S>,
}

impl<S: Container> Default for ScheduleContainers<S> {
	fn default() -> Self {
		Self {
			inner: HashMap::default(),
		}
	}
}

/// Helper to create a system that runs a container.
pub fn run_container_system<S: Container>(label: InternedScheduleLabel) -> impl FnMut(&mut World) {
	move |world: &mut World| {
		// Remove the children from the container. This is so we don't break ownership rules.
		let children = {
			let mut container = world.resource_mut::<ScheduleContainers<S>>();
			container.inner.remove(&label).unwrap()
		};

		// Run the children
		children.run(world);

		// Add the children back to the container
		// Unchecked, because we just removed it before
		world
			.resource_mut::<ScheduleContainers<S>>()
			.inner
			.insert_unique_unchecked(label, children);
	}
}

/// Adds methods to [`World`] for managing schedule containers.
pub trait WorldExt {
	/// Initializes the [`ScheduleContainers`] resource and inserts a new default container for the given label.
	///
	/// Returns `true` if the container was inserted, `false` if it was already present.
	fn init_schedule_container<S: FromWorld + Container>(
		&mut self,
		label: InternedScheduleLabel,
	) -> bool;

	/// Initializes the [`ScheduleContainers`] resource and inserts a container for the given label, if not yet present.
	///
	/// Returns `true` if the container was inserted, `false` if it was already present.
	fn insert_schedule_container<S: Container>(
		&mut self,
		label: InternedScheduleLabel,
		container: S,
	) -> bool;
}

impl WorldExt for World {
	fn init_schedule_container<S: FromWorld + Container>(
		&mut self,
		label: InternedScheduleLabel,
	) -> bool {
		let container = S::from_world(self);
		self.insert_schedule_container(label, container)
	}

	fn insert_schedule_container<S: Container>(
		&mut self,
		label: InternedScheduleLabel,
		container: S,
	) -> bool {
		let mut containers = self.get_resource_or_insert_with(ScheduleContainers::<S>::default);
		if !containers.inner.contains_key(&label) {
			containers.inner.insert(label, container);

			true
		} else {
			false
		}
	}
}
