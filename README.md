# Bevy Schedules, improved

Adds functionality to Bevy's existing Schedules to allow for nesting and using schedules as a replacement
for Sets for system ordering.

## Usage

Nest one or more schedules:

```rust
// Schedules can be added to other schedules
app.add_schedules(Update, Child);
app.add_schedules(Child, (GrandchildOne, GrandchildTwo));

// Add systems to schedules directly, no appending `.in_set(...)` to everything!
app.add_systems(Update, update_system);
app.add_systems(Child, child_system);
app.add_systems(GrandchildOne, grandchild_system_one);
app.add_systems(GrandchildTwo, grandchild_system_two);
```

All systems will run in Bevy's update loop without having to manually call `run` on the custom schedules.

A full example is available in [examples/nested_schedules.rs](examples/nested_schedules.rs).

## When not to use

Since running a schedule requires exclusive world access, schedules can't run in parallel. The systems in them
will follow all the regular Bevy rules for system execution order, but if you need to group systems and those groups
to potentially run in parallel, you should use Bevy's Sets instead.

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
