use bevy_ecs::{prelude::*, schedule::InternedScheduleLabel};
use bevy_utils::{HashMap, HashSet};

#[derive(Resource)]
pub struct ScheduleContainers<S> {
	pub inner: HashMap<InternedScheduleLabel, S>,
}

impl<S> Default for ScheduleContainers<S> {
	fn default() -> Self {
		Self {
			inner: HashMap::default(),
		}
	}
}

#[derive(Resource)]
pub struct ScheduleSystems<S> {
	pub inner: HashSet<InternedScheduleLabel>,
	phantom: std::marker::PhantomData<S>,
}

impl<S> Default for ScheduleSystems<S> {
	fn default() -> Self {
		Self {
			inner: HashSet::default(),
			phantom: std::marker::PhantomData,
		}
	}
}

pub trait WorldExt {
	/// Initializes the [`ScheduleContainers`] resource and inserts a new default container for the given label.
	fn init_schedule_container<S: FromWorld + Send + Sync + 'static>(
		&mut self,
		label: InternedScheduleLabel,
	);

	/// Initializes the [`ScheduleContainers`] resource and inserts a container for the given label, if not yet present.
	fn insert_schedule_container<S: Send + Sync + 'static>(
		&mut self,
		label: InternedScheduleLabel,
		container: S,
	) -> bool;

	/// Helper to mark whether a system to run the container of a schedule has been added.
	/// Returns `true` if the marker was inserted, `false` if it was already present.
	fn insert_schedule_container_system_marker<S: Send + Sync + 'static>(
		&mut self,
		label: InternedScheduleLabel,
	) -> bool;
}

impl WorldExt for World {
	fn init_schedule_container<S: FromWorld + Send + Sync + 'static>(
		&mut self,
		label: InternedScheduleLabel,
	) {
		let container = S::from_world(self);
		self.insert_schedule_container(label, container);
	}

	fn insert_schedule_container<S: Send + Sync + 'static>(
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

	fn insert_schedule_container_system_marker<S: Send + Sync + 'static>(
		&mut self,
		label: InternedScheduleLabel,
	) -> bool {
		let mut systems = self.get_resource_or_insert_with(ScheduleSystems::<S>::default);

		systems.inner.insert(label)
	}
}
