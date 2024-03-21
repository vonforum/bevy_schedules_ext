# Cargo features

The following are the cargo features available for this library (all enabled by default, unless noted otherwise):

- **nesting**: Enables nesting of schedules.
- **states**: Enables using `States` as schedules, so you can add systems to your states and have them run when the state is active.
- **app_ext**: Extends `App` with convenience methods (`add_schedules` for **nesting**, state schedule related methods for **states**).
    Disable this if you don't want to depend on `bevy_app`.
- **containers**: Adds the `ScheduleContainers` type and methods on `World` to manage them. Used by **nesting**.
