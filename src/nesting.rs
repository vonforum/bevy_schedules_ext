use bevy_ecs::{
	all_tuples,
	prelude::*,
	schedule::{InternedScheduleLabel, ScheduleLabel},
};
use bevy_utils::HashMap;

/// A container of sub-schedules for a given schedule.
/// Initialized by default for the parent schedule when you add child schedules.
#[derive(Resource)]
pub struct ScheduleContainer<T: ScheduleLabel> {
	pub inner: HashMap<InternedScheduleLabel, Vec<InternedScheduleLabel>>,
	phantom: std::marker::PhantomData<T>,
}

impl<T: ScheduleLabel> Default for ScheduleContainer<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: ScheduleLabel> ScheduleContainer<T> {
	pub fn new() -> Self {
		Self {
			inner: HashMap::default(),
			phantom: std::marker::PhantomData,
		}
	}
}

/// A trait for converting a schedule or a tuple of schedules into a list of schedule labels.
pub trait SchedulesIntoConfigs<Marker> {
	fn into_configs(self) -> Vec<InternedScheduleLabel>;
}

impl<T> SchedulesIntoConfigs<()> for T
where
	T: ScheduleLabel,
{
	fn into_configs(self) -> Vec<InternedScheduleLabel> {
		vec![self.intern()]
	}
}

#[doc(hidden)]
pub struct ScheduleConfigTupleMarker;

macro_rules! impl_schedules_into_configs {
    ($(($sys: ident, $name: ident)),*) => {
        impl<$($sys),*> SchedulesIntoConfigs<ScheduleConfigTupleMarker> for ($($sys,)*)
        where
            $($sys: ScheduleLabel),*
        {
            #[allow(non_snake_case)]
            fn into_configs(self) -> Vec<InternedScheduleLabel> {
                let ($($name,)*) = self;
                vec![$($name.intern(),)*]
            }
        }
    }
}

all_tuples!(impl_schedules_into_configs, 1, 20, S, s);

/// Adds the [`init_schedule_container`](WorldExt::init_schedule_container) method to the `World` type.
pub trait WorldExt {
	/// Initialize a schedule container (to add subschedules to) for a given schedule.
	/// Also adds a system to run the subschedules.
	fn init_schedule_container<T: ScheduleLabel>(&mut self, schedule: T) -> &mut Self;
}

impl WorldExt for World {
	fn init_schedule_container<T: ScheduleLabel>(&mut self, schedule: T) -> &mut Self {
		let mut container = self.get_resource_or_insert_with(ScheduleContainer::<T>::default);
		let schedule_label = schedule.intern();
		if !container.inner.contains_key(&schedule_label) {
			container.inner.insert(schedule_label, Vec::new());

			let mut schedules = self.get_resource_or_insert_with(Schedules::default);

			let system = move |world: &mut World| {
				world.resource_scope(
					|world: &mut World, container: Mut<ScheduleContainer<T>>| {
						for &label in container.inner.get(&schedule_label).unwrap() {
							world.run_schedule(label);
						}
					},
				);
			};

			if let Some(schedule) = schedules.get_mut(schedule_label) {
				schedule.add_systems(system);
			} else {
				let mut new_schedule = Schedule::new(schedule_label);
				new_schedule.add_systems(system);
				schedules.insert(new_schedule);
			}
		}

		self
	}
}

/// Adds the [`add_schedules`](App::add_schedules) method to the `App` type.
#[cfg(feature = "app_ext")]
pub mod app_ext {
	use bevy_app::prelude::*;
	use bevy_ecs::prelude::*;

	use super::*;

	pub trait AppExt {
		/// Add subschedules to a given schedule in this app.
		///
		/// # Examples
		///
		/// ```
		/// # use bevy_app::prelude::*;
		/// # use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
		/// #
		/// # use bevy_mod_schedules::prelude::*;
		/// #
		/// # #[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
		/// # struct Child;
		/// #
		/// # #[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
		/// # struct GrandchildA;
		/// #
		/// # #[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
		/// # struct GrandchildB;
		/// #
		/// # let mut app = App::new();
		/// #
		/// app.add_schedules(Update, Child);
		/// app.add_schedules(Child, (GrandchildA, GrandchildB));
		/// ```
		fn add_schedules<Marker, P: ScheduleLabel, S: SchedulesIntoConfigs<Marker>>(
			&mut self,
			parent: P,
			children: S,
		);
	}

	impl AppExt for App {
		fn add_schedules<Marker, P: ScheduleLabel, S: SchedulesIntoConfigs<Marker>>(
			&mut self,
			parent: P,
			children: S,
		) {
			let parent_label = parent.intern();
			self.world.init_schedule_container(parent);

			let config = children.into_configs();
			config.iter().for_each(|&label| {
				self.init_schedule(label);
			});

			self.world.resource_scope(
				move |_world: &mut World, mut container: Mut<ScheduleContainer<P>>| {
					for label in config {
						container.inner.get_mut(&parent_label).unwrap().push(label);
					}
				},
			);
		}
	}
}
