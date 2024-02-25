use bevy_ecs::{
	all_tuples,
	prelude::*,
	schedule::ScheduleLabel,
};
use bevy_ecs::schedule::SystemConfigs;

/// A trait for converting a schedule or a tuple of schedules into systems.
pub trait SchedulesIntoSystems<Marker> where Self: Sized {
	fn into_configs(self) -> SystemConfigs;
}

impl<T> SchedulesIntoSystems<()> for T
where
	T: ScheduleLabel,
{
	fn into_configs(self) -> SystemConfigs {
		let label = self.intern();

		(move |world: &mut World| {
			world.run_schedule(label);
		}).into_configs()
	}
}

#[doc(hidden)]
pub struct ScheduleConfigTupleMarker;

macro_rules! impl_schedules_into_configs {
    ($(($sys: ident, $name: ident, $label: ident)),*) => {
        impl<$($sys),*> SchedulesIntoSystems<ScheduleConfigTupleMarker> for ($($sys,)*)
        where
            $($sys: ScheduleLabel),*
        {
            #[allow(non_snake_case)]
            fn into_configs(self) -> SystemConfigs {
                let ($($name,)*) = self;
				let ($($label,)*) = ($($name.intern(),)*);

				($(move |world: &mut World| {
					world.run_schedule($label);
				}, )*).into_configs().chain()
            }
        }
    }
}

all_tuples!(impl_schedules_into_configs, 1, 20, S, s, l);

/// Adds the [`add_schedules`](ScheduleExt::add_schedules) method to the `Schedule` type.
pub trait ScheduleExt {
	/// Add subschedules to this schedule.
	fn add_schedules<Marker, S: SchedulesIntoSystems<Marker>>(
		&mut self,
		children: S,
	);
}

impl ScheduleExt for Schedule {
	fn add_schedules<Marker, S: SchedulesIntoSystems<Marker>>(
		&mut self,
		children: S,
	) {
		self.add_systems(children.into_configs());
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
		fn add_schedules<Marker, P: ScheduleLabel, S: SchedulesIntoSystems<Marker>>(
			&mut self,
			parent: P,
			children: S,
		) -> &mut Self;
	}

	impl AppExt for App {
		fn add_schedules<Marker, P: ScheduleLabel, S: SchedulesIntoSystems<Marker>>(
			&mut self,
			parent: P,
			children: S,
		) -> &mut Self {
			let label = parent.intern();
			let mut schedules = self.world.resource_mut::<Schedules>();

			if let Some(schedule) = schedules.get_mut(label) {
				schedule.add_schedules(children);
			} else {
				let mut new_schedule = Schedule::new(label);
				new_schedule.add_schedules(children);
				schedules.insert(new_schedule);
			}

			self
		}
	}
}
