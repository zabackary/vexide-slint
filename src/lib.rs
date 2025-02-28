#![no_std]

//! Slint platform implementation for the V5 Brain screen.

extern crate alloc;
use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use slint::{
    platform::{
        software_renderer::{MinimalSoftwareWindow, RepaintBufferType},
        Platform, PointerEventButton, WindowEvent,
    },
    LogicalPosition, PhysicalPosition, PhysicalSize, Rgb8Pixel,
};
use vexide::devices::display::{Display, Rect, TouchState};
use vexide::time::Instant;

/// A Slint platform implementation for the V5 Brain screen.
///
/// This struct is a wrapper around a [`Display`] and a [`MinimalSoftwareWindow`]
/// and will handle updates to the screen through the [`Platform`] trait.
pub struct V5Platform {
    start: Instant,
    window: Rc<MinimalSoftwareWindow>,
    display: RefCell<Display>,
    display_pressed: RefCell<bool>,

    buffer: RefCell<
        [Rgb8Pixel;
            Display::HORIZONTAL_RESOLUTION as usize * Display::VERTICAL_RESOLUTION as usize],
    >,
}
impl V5Platform {
    /// Create a new [`V5Platform`] from a [`Display`].
    ///
    /// This is used internally by [`initialize_slint_platform`] to create the
    /// platform.
    #[must_use]
    pub fn new(display: Display) -> Self {
        let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
        window.set_size(PhysicalSize::new(
            Display::HORIZONTAL_RESOLUTION as _,
            Display::VERTICAL_RESOLUTION as _,
        ));
        Self {
            start: Instant::now(),
            window,
            display: RefCell::new(display),
            display_pressed: RefCell::new(false),
            #[allow(clippy::large_stack_arrays)] // we got plenty
            buffer: RefCell::new(
                [Rgb8Pixel::new(0, 0, 0);
                    Display::HORIZONTAL_RESOLUTION as usize * Display::VERTICAL_RESOLUTION as usize],
            ),
        }
    }

    fn get_touch_event(&self) -> WindowEvent {
        let event = self.display.borrow().touch_status();
        let physical_pos = PhysicalPosition::new(event.x.into(), event.y.into());
        let position = LogicalPosition::from_physical(physical_pos, 1.0);
        match event.state {
            TouchState::Released => {
                *self.display_pressed.borrow_mut() = false;
                WindowEvent::PointerReleased {
                    position,
                    button: PointerEventButton::Left,
                }
            }
            TouchState::Pressed => {
                if self.display_pressed.replace(true) {
                    WindowEvent::PointerMoved { position }
                } else {
                    WindowEvent::PointerPressed {
                        position,
                        button: PointerEventButton::Left,
                    }
                }
            }
            TouchState::Held => WindowEvent::PointerMoved { position },
        }
    }
}

impl Platform for V5Platform {
    fn create_window_adapter(
        &self,
    ) -> Result<alloc::rc::Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }
    fn duration_since_start(&self) -> core::time::Duration {
        self.start.elapsed()
    }
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        loop {
            slint::platform::update_timers_and_animations();

            self.window.draw_if_needed(|renderer| {
                let mut buf = *self.buffer.borrow_mut();
                renderer.render(&mut buf, Display::HORIZONTAL_RESOLUTION as _);

                self.display.borrow_mut().draw_buffer(
                    Rect::from_dimensions(
                        [0, 0],
                        Display::HORIZONTAL_RESOLUTION as _,
                        Display::VERTICAL_RESOLUTION as _,
                    ),
                    buf,
                    Display::HORIZONTAL_RESOLUTION.into(),
                );
            });

            self.window.dispatch_event(self.get_touch_event());

            // Hand the CPU back to the scheduler so that user code can run
            // This used to not run if there were any animations running, but
            // it seems to be necessary to run it regardless
            vexide::runtime::block_on(vexide::time::sleep(Display::REFRESH_INTERVAL));
        }
    }
}

/// Sets the Slint platform to [`V5Platform`].
///
/// This function should be called before any other Slint functions are called
/// and lets Slint know that it should use the V5 Brain screen as the platform.
///
/// # Panics
///
/// Panics if the Slint platform is already set (i.e., this function has already
/// been called).
pub fn initialize_slint_platform(display: Display) {
    slint::platform::set_platform(Box::new(V5Platform::new(display)))
        .expect("Slint platform already set!");
}
