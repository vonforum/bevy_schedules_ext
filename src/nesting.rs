//! Extends Bevy to enable nesting schedules.

use bevy_ecs::{
	prelude::*,
	schedule::{InternedScheduleLabel, ScheduleLabel},
};
use bevy_utils::all_tuples;

/// Exports the app extension if the feature is enabled.
pub mod prelude {
	#[cfg(feature = "app_ext")]
	pub use super::app_ext::AppExt as NestingAppExt;
}

/// A trait for converting a schedule or a tuple of schedules into a container compatible form.
pub trait SchedulesIntoConfigs<Marker>
where
	Self: Sized,
{
	/// Converts the schedule or tuple of schedules into a vector of labels.
	fn to_vec(&self) -> Vec<InternedScheduleLabel>;
}

impl<T> SchedulesIntoConfigs<()> for T
where
	T: ScheduleLabel,
{
	fn to_vec(&self) -> Vec<InternedScheduleLabel> {
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
			fn to_vec(&self) -> Vec<InternedScheduleLabel> {
				let ($($name,)*) = self;
				vec![$($name.intern(),)*]
			}
        }
    }
}

all_tuples!(impl_schedules_into_configs, 1, 20, S, s, l);

/// Helper to create a system that runs the children of a schedule.
pub fn create_run_children_system(label: InternedScheduleLabel) -> impl FnMut(&mut World) {
	type Containers = crate::containers::ScheduleContainers<Vec<InternedScheduleLabel>>;
	move |world: &mut World| {
		// Remove the children from the container. This is so we don't break ownership rules.
		let children = {
			let mut container = world.resource_mut::<Containers>();
			container.inner.remove(&label).unwrap()
		};

		// Run the children
		children.iter().for_each(|&label| {
			world.run_schedule(label);
		});

		// Add the children back to the container
		// Unchecked, because we just removed it before
		world
			.resource_mut::<Containers>()
			.inner
			.insert_unique_unchecked(label, children);
	}
}

/// Adds methods to the [`App`] type for working with nested schedules.
#[cfg(feature = "app_ext")]
pub mod app_ext {
	use bevy_app::prelude::*;

	use super::*;

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
			// Initialize child schedules in the world
			children.to_vec().iter().for_each(|&label| {
				self.init_schedule(label);
			});

			let label = parent.intern();

			use crate::containers::*;

			type Inner = Vec<InternedScheduleLabel>;

			self.world.init_schedule_container::<Inner>(label); // Initialize the container if not yet present
			if self
				.world
				.insert_schedule_container_system_marker::<Inner>(label)
			{
				// If the system to run the child schedules isn't present yet, add it
				self.add_systems(label, create_run_children_system(label));
			}

			// Add the children to the container
			self.world
				.resource_mut::<ScheduleContainers<Inner>>()
				.inner
				.get_mut(&label)
				.unwrap()
				.extend(children.to_vec());

			self
		}
	}
}
