# Bevy Schedules, improved

Adds functionality to Bevy's existing Schedules, allowing nesting and using schedules as a replacement
for Sets for system grouping and ordering.

Extends Bevy's existing structures, no `.add_plugin` or managing new Resources.

## Features

### Nesting

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

A full example is available in [examples/nested_schedules.rs][1].

### States

Use Bevy's States as Schedules, so you can add systems to your states and have them run when the state is active,
no run conditions needed.

<table>
<tr>
<td>With <code>bevy_schedules_ext</code></td>
<td>Vanilla bevy</td>
</tr>

<tr>
<td>

```rust
// Initialize the state
app.init_state::<GameState>();

// Add it to our Update loop
app.add_state_to_schedule::<GameState>(Update);

// Add systems to the state
app.add_systems(GameState::Menu, menu_system);
app.add_systems(GameState::Playing, playing_system);
```

</td>

<td>

```rust
// Initialize the state, pretty much the same
app.init_state::<GameState>();

// Add systems to our update loop, but we need to check on every frame if the state is active
app.add_systems(Update, menu_system.run_if(in_state(GameState::Menu)));
app.add_systems(Update, playing_system.run_if(in_state(GameState::Playing)));
```

</td>
<tr>
</table>

A full example is available in [examples/states.rs][2].

## Downsides

Since running a schedule requires exclusive world access, schedules can't run in parallel. So any time systems in
different groupings need to run in parallel, nesting or using schedule states will block that. Ideally, you'd use a
combination of both this crate and vanilla Bevy, with schedules to contain the larger groupings of systems and
vanilla Bevy to handle groups that might overlap.

# Bevy compatibility

|  Bevy version | `bevy_schedules_ext` version |
|--------------:|:-----------------------------|
| `main` branch | [`master` branch][3]         |
|        `0.13` | `0.13`                       |

*Note: A newer version of this crate **may** work on an older version of Bevy and vice versa, however it's not tested
and would require extra work on your part (e.g. patching `Cargo.toml` to match the versions).*

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[1]: https://github.com/vonforum/bevy_schedules_ext/tree/master/examples/nested_schedules.rs
[2]: https://github.com/vonforum/bevy_schedules_ext/tree/master/examples/states.rs
[3]: https://github.com/vonforum/bevy_schedules_ext/tree/master
