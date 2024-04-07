//! # Bevy Schedules, improved
//!
//! Adds functionality to Bevy's existing Schedules to allow for nesting and using schedules as a
//! replacement for Sets for system ordering.
//!
//! ## Example
//!
//! ```rust
//! # use bevy_app::prelude::*;
//! # use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
//! #
//! # use bevy_schedules_ext::prelude::*;
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
//! # fn child_system() {}
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
#![warn(missing_docs)]

#[cfg(feature = "containers")]
pub mod containers;

#[cfg(feature = "nesting")]
pub mod nesting;

#[cfg(feature = "states")]
pub mod states;

/// Exposes all enabled app extensions and re-exports Bevy's [`ScheduleLabel`].
///
/// [`ScheduleLabel`]: bevy_ecs::schedule::ScheduleLabel
pub mod prelude {
	pub use bevy_ecs::schedule::ScheduleLabel;

	#[cfg(feature = "containers")]
	pub use crate::containers::WorldExt as ContainersWorldExt;
	#[cfg(feature = "nesting")]
	pub use crate::nesting::prelude::*;
	#[cfg(feature = "states")]
	pub use crate::states::prelude::*;
}
