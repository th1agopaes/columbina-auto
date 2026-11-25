//! The application shell: the window and the event loop around the game.

use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

/// Opens a window with the given title and blocks until it is closed.
///
/// The window closes with its close button or the Esc key.
///
/// # Errors
///
/// Returns an error if the platform cannot start an event loop, for example
/// on a machine with no display (such as a CI runner).
pub fn run(title: &str) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    let mut app = App {
        title: title.to_owned(),
        window: None,
    };
    event_loop.run_app(&mut app)
}

/// State kept between events.
struct App {
    title: String,
    /// `None` until the platform says the app can create windows.
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Some platforms (Android, the web) only allow windows after the first resume.
        if self.window.is_none() {
            let attributes = Window::default_attributes().with_title(&self.title);
            let window = event_loop
                .create_window(attributes)
                .expect("the window should open");
            self.window = Some(window);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => event_loop.exit(),
            _ => {}
        }
    }
}
