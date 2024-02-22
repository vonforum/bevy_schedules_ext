use bevy_app::prelude::*;
use bevy_ecs::{
	all_tuples,
	prelude::*,
	schedule::{InternedScheduleLabel, ScheduleLabel},
};

/// A container of sub-schedules for a given schedule.
/// Initialized by default when you add child schedules.
#[derive(Resource)]
pub struct SimpleScheduleContainer<T: ScheduleLabel> {
	labels: Vec<InternedScheduleLabel>,
	phantom: std::marker::PhantomData<T>,
}

impl<T: ScheduleLabel> Default for SimpleScheduleContainer<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: ScheduleLabel> SimpleScheduleContainer<T> {
	pub fn new() -> Self {
		Self {
			labels: Vec::new(),
			phantom: std::marker::PhantomData,
		}
	}

	/// Add a schedule to the container. Pushes the schedule to the end of the list.
	pub fn add(&mut self, label: InternedScheduleLabel) {
		self.labels.push(label);
	}
}

impl<'a, T: ScheduleLabel> IntoIterator for &'a SimpleScheduleContainer<T> {
	type Item = &'a InternedScheduleLabel;
	type IntoIter = std::slice::Iter<'a, InternedScheduleLabel>;

	fn into_iter(self) -> Self::IntoIter {
		self.labels.iter()
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

pub trait AppExt {
	fn add_schedules<Marker, P: ScheduleLabel, S: SchedulesIntoConfigs<Marker>>(
		&mut self,
		parent: P,
		children: S,
	);
}

impl AppExt for App {
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
	) {
		if !self.world.contains_resource::<SimpleScheduleContainer<P>>() {
			self.world
				.insert_resource(SimpleScheduleContainer::<P>::new());

			self.add_systems(parent, move |world: &mut World| {
				world.resource_scope(
					|world: &mut World, container: Mut<SimpleScheduleContainer<P>>| {
						for &label in &container {
							world.run_schedule(label);
						}
					},
				);
			});
		}

		let config = children.into_configs();
		config.iter().for_each(|&label| {
			self.init_schedule(label);
		});

		self.world.resource_scope(
			move |_world: &mut World, mut container: Mut<SimpleScheduleContainer<P>>| {
				for label in config {
					container.add(label);
				}
			},
		);
	}
}
