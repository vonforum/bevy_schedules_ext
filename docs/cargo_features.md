# Cargo features

The following are the cargo features available for this library (all enabled by default, unless noted otherwise):

- **nesting**: Enables nesting of schedules.
- **app_ext**: Extends `App` with convenience methods (`add_schedules`). Disable this if you don't want to depend on `bevy_app`.
    See [examples/nested_schedules_no_app](https://github.com/vonforum/bevy_schedules_ext/blob/master/examples/nested_schedules_no_app.rs)
    for an example of how to use the library without `bevy_app` (both with and without `nesting_containers`).
- **nesting_containers**: Nested schedules are added to a container, instead of as systems directly.
    Pros: guarantees schedule running order, schedule tree can be modified, only one system for each schedule with children.
    Cons: each parent schedule adds an overhead of two resource queries and one hashmap removal and insertion per parent.
- **containers**: Enabled by `nesting_containers`. Adds the `ScheduleContainers` type and methods on `World` to manage them.
