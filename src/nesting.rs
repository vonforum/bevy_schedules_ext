use bevy_ecs::{
	all_tuples,
	prelude::*,
	schedule::ScheduleLabel,
};
use bevy_ecs::schedule::{InternedScheduleLabel, SystemConfigs};

/// A trait for converting a schedule or a tuple of schedules into systems.
pub trait SchedulesIntoConfigs<Marker> where Self: Sized {
	fn into_systems(self) -> SystemConfigs;
	fn into_vec(&self) -> Vec<InternedScheduleLabel>;
}

impl<T> SchedulesIntoConfigs<()> for T
where
	T: ScheduleLabel,
{
	fn into_systems(self) -> SystemConfigs {
		let label = self.intern();

		(move |world: &mut World| {
			world.run_schedule(label);
		}).into_configs()
	}

	fn into_vec(&self) -> Vec<InternedScheduleLabel> {
		vec![self.intern()]
	}
}

#[doc(hidden)]
pub struct ScheduleConfigTupleMarker;

macro_rules! impl_schedules_into_configs {
    ($(($sys: ident, $name: ident, $label: ident)),*) => {
        impl<$($sys),*> SchedulesIntoConfigs<ScheduleConfigTupleMarker> for ($($sys,)*)
        where
            $($sys: ScheduleLabel),*
        {
            #[allow(non_snake_case)]
            fn into_systems(self) -> SystemConfigs {
                let ($($name,)*) = self;
				let ($($label,)*) = ($($name.intern(),)*);

				($(move |world: &mut World| {
					world.run_schedule($label);
				}, )*).into_configs().chain()
            }

			#[allow(non_snake_case)]
			fn into_vec(&self) -> Vec<InternedScheduleLabel> {
				let ($($name,)*) = self;
				vec![$($name.intern(),)*]
			}
        }
    }
}

all_tuples!(impl_schedules_into_configs, 1, 20, S, s, l);

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
		/// # use bevy_schedules_ext::prelude::*;
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
		) -> &mut Self;
	}

	impl AppExt for App {
		fn add_schedules<Marker, P: ScheduleLabel, S: SchedulesIntoConfigs<Marker>>(
			&mut self,
			parent: P,
			children: S,
		) -> &mut Self {
			// Initialize child schedules in the world
			children.into_vec().iter().for_each(|&label| {
				self.init_schedule(label);
			});

			let label = parent.intern();

			#[cfg(not(feature = "nesting_containers"))]
			{
				// Convert the children into systems and add them to the parent schedule
				let mut schedules = self.world.resource_mut::<Schedules>();
				if let Some(schedule) = schedules.get_mut(label) {
					schedule.add_systems(children.into_systems());
				} else {
					let mut new_schedule = Schedule::new(label);
					new_schedule.add_systems(children.into_systems());
					schedules.insert(new_schedule);
				}
			}

			#[cfg(feature = "nesting_containers")]
			{
				use crate::containers::ScheduleContainers;
				type NestedSchedulesContainer = ScheduleContainers<Vec<InternedScheduleLabel>>;

				let mut container = self.world.get_resource_or_insert_with(NestedSchedulesContainer::default);

				if !container.inner.contains_key(&label) {
					container.inner.insert(label, children.into_vec());

					// System that runs the child schedules
					let system = move |world: &mut World| {
						// Remove the children from the container. This is so we don't break ownership rules.
						let children = {
							let mut container = world.resource_mut::<NestedSchedulesContainer>();
							container.inner.remove(&label).unwrap()
						};

						// Run the children
						children.iter().for_each(|&label| {
							world.run_schedule(label);
						});

						// Add the children back to the container
						// Unchecked, because we just removed it before
						world.resource_mut::<NestedSchedulesContainer>().inner.insert_unique_unchecked(label, children);
					};

					// Add the system to the parent schedule
					let mut schedules = self.world.resource_mut::<Schedules>();
					if let Some(schedule) = schedules.get_mut(label) {
						schedule.add_systems(system);
					} else {
						let mut new_schedule = Schedule::new(label);
						new_schedule.add_systems(system);
						schedules.insert(new_schedule);
					}
				} else {
					container.inner.get_mut(&label).unwrap().extend(children.into_vec());
				}
			}

			self
		}
	}
}
