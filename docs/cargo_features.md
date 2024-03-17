# Cargo features

The following are the cargo features available for this library (all enabled by default, unless noted otherwise):

- **nesting**: Enables nesting of schedules.
- **app_ext**: Extends `App` with convenience methods (`add_schedules`). Disable this if you don't want to depend on `bevy_app`.
- **nesting_containers** *(deprecated)*: Nested schedules are added to a container, instead of as systems directly.
    This will be the default, so if you want to use the old behavior, see history for a way to do it yourself.
- **containers**: Enabled by `nesting_containers`. Adds the `ScheduleContainers` type and methods on `World` to manage them.
- **states**: Enables using `States` as schedules, so you can add systems to your states and have them run when the state is active.
