# Cargo features

The following are the cargo features available for this library (all enabled by default):

- **nesting**: Enables nesting of schedules.
- **app_ext**: Extends `App` with convenience methods (`add_schedules`). Disable this if you don't want to depend on `bevy_app`.
    See [examples/nested_schedules_no_app](https://github.com/vonforum/bevy_schedules_ext/blob/master/examples/nested_schedules_no_app.rs)
    for an example of how to use the library without `bevy_app`.
