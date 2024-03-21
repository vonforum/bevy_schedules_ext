//! Extends Bevy to enable nesting schedules.

use bevy_ecs::{
	prelude::*,
	schedule::{InternedScheduleLabel, ScheduleLabel},
};
use bevy_utils::all_tuples;

use crate::containers::prelude::*;

/// Exports the app extension if the feature is enabled.
pub mod prelude {
	#[cfg(feature = "app_ext")]
	pub use super::app_ext::AppExt as NestingAppExt;
}

/// Container for nested schedules.
#[derive(Default)]
pub struct NestedSchedules {
	/// Vector of schedules.
	pub inner: Vec<InternedScheduleLabel>,
}

impl Container for NestedSchedules {
	fn run(&self, world: &mut World) {
		self.inner.iter().for_each(|&label| {
			world.run_schedule(label);
		});
	}
}
/// A trait for converting a schedule or a tuple of schedules into a container compatible form.
pub trait SchedulesIntoConfigs<Marker>
where
	Self: Sized,
{
	/// Converts the schedule or tuple of schedules into a vector of labels.
	fn to_container(&self) -> NestedSchedules;
}

impl<T> SchedulesIntoConfigs<()> for T
where
	T: ScheduleLabel,
{
	fn to_container(&self) -> NestedSchedules {
		NestedSchedules {
			inner: vec![self.intern()],
		}
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
			fn to_container(&self) -> NestedSchedules {
				let ($($name,)*) = self;
				NestedSchedules {
					inner: vec![$($name.intern(),)*]
				}
			}
        }
    }
}

all_tuples!(impl_schedules_into_configs, 1, 20, S, s, l);

/// Adds methods to the [`App`] type for working with nested schedules.
#[cfg(feature = "app_ext")]
pub mod app_ext {
	use bevy_app::prelude::*;

	use super::*;
	use crate::containers::run_container_system;

	/// Adds the [`add_schedules`](App::add_schedules) method to the `App` type.
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
			let container = children.to_container();
			// Initialize child schedules in the world
			container.inner.iter().for_each(|&label| {
				self.init_schedule(label);
			});

			let label = parent.intern();

			// Initialize the container if not yet present
			if self.world.init_schedule_container::<NestedSchedules>(label) {
				// If the container wasn't present, add a system to run the children
				self.add_systems(label, run_container_system::<NestedSchedules>(label));
			}

			// Add the children to the container
			self.world
				.resource_mut::<ScheduleContainers<NestedSchedules>>()
				.inner
				.get_mut(&label)
				.unwrap()
				.inner
				.extend(container.inner);

			self
		}
	}
}
