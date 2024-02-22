#[cfg(feature = "nesting")]
pub mod nesting;

pub mod prelude {
	pub use bevy_ecs::schedule::ScheduleLabel;

	#[cfg(all(feature = "nesting", feature = "app_ext"))]
	pub use crate::nesting::app_ext::AppExt as NestingAppExt;
}
