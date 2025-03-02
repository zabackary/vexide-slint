#![no_std]
#![no_main]

use vexide::prelude::{Compete, CompeteExt as _, Peripherals};
use vexide_slint::initialize_slint_platform;

// The slint macro syntax doesn't actually work for the software renderer that
// this crate uses. You'll have to use a build script. Check the readme for more
// information.
slint::slint!(
import { AboutSlint, VerticalBox } from "std-widgets.slint";

export component MainWindow inherits Window {
    VerticalBox {
        Text {
            text: "Hello World!";
        }
        AboutSlint {
            preferred-height: 150px;
        }
    }
}
);

struct Robot {
    // ...
}

impl Compete for Robot {
    // ...
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {
        // ...
    };

    // Since running the Slint UI is a blocking operation, we need to spawn the
    // competition task as a separate task that will run concurrently.
    // The Slint runtime internally polls all spawned futures.
    vexide::task::spawn(robot.compete()).detach();

    // Initialize the Slint platform with the V5 display-backed implementation.
    initialize_slint_platform(peripherals.display);
    // Create and run the application. For more information on this, see the
    // Slint documentation.
    MainWindow::new()
        .expect("Failed to create window")
        .run()
        .expect("Failed to run application");
    // Since MyApplication::run() could return if the application is closed
    // programmatically, we need to convince the compiler that the return type
    // is `!` (never).
    vexide::program::exit();
}
