use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/*
 * Description: Called when a window event occurs, handles events like escape key press, and window close action.
 * Arguments:   WindowEvent - an event from the window (resize, close, etc)
 *              ControlFlow - indicates the desired behavior of the event loop
 */
fn handle_window_event(event: &WindowEvent, control_flow: &mut ControlFlow) {
    match event {
        // Close the window when the close button is clicked or Escape is pressed
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput {
            input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            },
            ..
        } => *control_flow = ControlFlow::Exit,
        // Ignore other events
        _ => {}
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run_window() {

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    // Create a new event loop
    let event_loop = EventLoop::new();

    // Create a new window
    let window = WindowBuilder::new().build(&event_loop)
        .expect("Problem building window");

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    // Run the event loop
    event_loop.run(move |system_event, _, control_flow| {
        // Check if the system event is a window event
        if let Event::WindowEvent { ref event, window_id } = system_event {
            // Check if the window even is related to this window
            if window_id == window.id() {
                handle_window_event(event, control_flow);
            }
        }
    });
}