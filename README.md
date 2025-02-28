# Slint for Vexide

This crate exists to allow for the use of Slint-based UIs in vexide. It provides
an implementation of the Slint `Platform` trait that uses the V5 brain to render
the UI.

## Usage

To use this crate, add it to your `Cargo.toml`:

```toml
[dependencies]
slint-vexide = "0.1"
```

Then, in your code, you can use it like so:

```rust
use slint_vexide::initialize_slint_platform;

#[vexide::main]
async fn main() {
    let robot = Robot {
        // ...
    };
    vexide::runtime::spawn(robot.compete());

    initialize_slint_platform();
    slint::run();
}
```
