//! # Bevy Schedules, improved
//!
//! Adds functionality to Bevy's existing Schedules to allow for nesting and using schedules as a
//! replacement for Sets for system ordering.
//!
//! ## Example
//!
//! ```rust
//!# use bevy_app::prelude::*;
//! # use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
//! #
//! # use bevy_mod_schedules::prelude::*;
//! #
//! # #[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
//! # struct Child;
//! #
//! # #[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
//! # struct GrandchildOne;
//! #
//! # #[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
//! # struct GrandchildTwo;
//! #
//! # fn update_system() {}
//! # fn child_system() {}{}
//! # fn grandchild_system_one() {}
//! # fn grandchild_system_two() {}
//! #
//! # let mut app = App::new();
//! #
//! // Schedules can be added to other schedules
//! app.add_schedules(Update, Child);
//! app.add_schedules(Child, (GrandchildOne, GrandchildTwo));
//!
//! // Add systems to schedules directly, no appending `.in_set(...)` to everything!
//! app.add_systems(Update, update_system);
//! app.add_systems(Child, child_system);
//! app.add_systems(GrandchildOne, grandchild_system_one);
//! app.add_systems(GrandchildTwo, grandchild_system_two);
//! ```
#![doc = include_str!("../docs/cargo_features.md")]

#[cfg(feature = "nesting")]
pub mod nesting;

pub mod prelude {
	pub use bevy_ecs::schedule::ScheduleLabel;

	#[cfg(all(feature = "nesting", feature = "app_ext"))]
	pub use crate::nesting::app_ext::AppExt as NestingAppExt;
}
