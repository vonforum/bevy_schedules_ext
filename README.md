# Bevy Schedules, improved

Adds functionality to Bevy's existing Schedules, allowing nesting and using schedules as a replacement
for Sets for system grouping and ordering.

Extends Bevy's existing structures, no `.add_plugin` or managing new Resources.

## Usage

Nest one or more schedules:

<table>
<tr>
<td>With <code>bevy_schedules_ext</code></td>
<td>Vanilla bevy</td>
</tr>

<tr>
<td>

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

</td>

<td>

```rust
// Sets configured manually, Update must be prepended to everything
app.configure_sets(Update, Child);
app.configure_sets(Update, (GrandchildOne, GrandchildTwo).chain().in_set(Child));

// Adding systems to sets requires `.in_set(...)`
app.add_systems(Update, update_system);
app.add_systems(Update, child_system.in_set(Child));
app.add_systems(Update, grandchild_system_one.in_set(GrandchildOne));
app.add_systems(Update, grandchild_system_two.in_set(GrandchildTwo));
```

</td>
<tr>
</table>

All systems will run in Bevy's update loop without having to manually call `run` on the custom schedules.

A full example is available in [examples/nested_schedules.rs](examples/nested_schedules.rs).

## When not to use

Since running a schedule requires exclusive world access, schedules can't run in parallel. The systems in them
will follow all the regular Bevy rules for system execution order, but if you need to group systems and those groups
to potentially run in parallel, you should use Bevy's Sets instead.

# Bevy compatibility

| Bevy version | `bevy_schedules_ext` version |
|-------------:|:-----------------------------|
|       `main` | [`bevy_main` branch][1]      |
|       `0.13` | `0.13.x`                     |

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[1]: https://github.com/vonforum/bevy_schedules_ext/tree/bevy_main
