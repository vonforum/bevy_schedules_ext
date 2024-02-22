#[cfg(feature = "nesting")]
pub mod nesting;

pub mod prelude {
	pub use bevy_ecs::schedule::ScheduleLabel;

	#[cfg(feature = "nesting")]
	pub use crate::nesting::AppExt as NestingAppExt;
}
