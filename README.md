# `seldom_singleton`

[![Crates.io](https://img.shields.io/crates/v/seldom_singleton.svg)](https://crates.io/crates/seldom_singleton)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_singleton#license)
[![Crates.io](https://img.shields.io/crates/d/seldom_singleton.svg)](https://crates.io/crates/seldom_singleton)

`seldom_singleton` adds a helper `SystemParam` for when you have a resource containing a handle. If
you're using Bevy's asset system to, for example, load a list of items in the game and their
properties, you'd end up with a handle in a resource, and you'd need to get `Res<Assets<MyItems>>`
to get the asset you need. This crate adds a helper that lets you avoid the additional system param.

This is a very small crate. You can just copy the source code into your project to avoid adding a
dependency.

Before:

```rust
#[derive(Asset, TypePath)]
struct MyAsset;

#[derive(Resource, Deref)]
struct MySingleton(Handle<MyAsset>);

fn my_system(my_assets: Res<Assets<MyAsset>>, my_singleton: Res<MySingleton>) {
    // Return if the asset doesn't exist
    let Some(my_asset) = my_assets.get(&**my_singleton) else {
        return;
    };

    // or panic
    let my_asset = my_assets.get(&**my_singleton).unwrap();
}
```

After:

```rust
#[derive(Asset, TypePath)]
struct MyAsset;

// Your resource. Add it to the world yourself.
#[derive(Resource, Deref)]
struct MySingleton(Handle<MyAsset>);

// `AssetSingleton` is this crate's `SystemParam`. This type definition can help reduce boilerplate
// a bit, but it's optional. There's also `AssetSingletonMut`.
type MySingletonParam<'w> = AssetSingleton<'w, MySingleton>;

fn my_system(my_singleton: MySingletonParam) {
    // Return if the asset doesn't exist
    let Some(my_asset) = my_singleton.get() else {
        return;
    };

    // or panic
    let my_asset = my_singleton.unwrap();

    // `MySingletonMut` has `get_mut` and `unwrap_mut`
}
```

## Usage

Add to your `Cargo.toml`

```toml
# Replace * with your desired version

[dependencies]
seldom_singleton = "*"
```

## Compatibility

| Bevy | `seldom_singleton` |
| ---- | ------------------ |
| 0.13 | 0.1                |

## License

`seldom_singleton` is dual-licensed under MIT and Apache 2.0 at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
