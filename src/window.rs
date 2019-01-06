
use glutin::GlContext;

pub struct GameWindow {
    inner: glutin::GlWindow,
}

impl GameWindow {
    pub fn new(
        window_attribs: glutin::WindowAttributes,
        context_attribs: glutin::GlAttributes<&glutin::Context>,
    ) -> (GameWindow, glutin::EventsLoop) {
        let events_loop = GameWindow::new_events_loop();
        let gl_window = glutin::GlWindow::new(
            GameWindow::new_window_builder(window_attribs),
            GameWindow::new_context_builder(context_attribs),
            &events_loop,
        )
        .expect("Failed to create GlWindow");

        //VERY IMPORTANT SEQUENCE - if load_gl_ptr before make_current it throws an error - cannot load fn ptr
        unsafe {
            gl_window.make_current().expect("Could not make the window current!");
        }
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        (GameWindow { inner: gl_window }, events_loop)
    }
    fn new_events_loop() -> glutin::EventsLoop {
        glutin::EventsLoop::new()
    }
    fn new_context_builder(
        gl_attribs: glutin::GlAttributes<&glutin::Context>,
    ) -> glutin::ContextBuilder {
        let mut context_builder = glutin::ContextBuilder::new();
        context_builder.gl_attr = gl_attribs;
        context_builder
    }
    fn new_window_builder(window_attribs: glutin::WindowAttributes) -> glutin::WindowBuilder {
        let mut window_builder = glutin::WindowBuilder::new();
        window_builder.window = window_attribs;
        window_builder
    }
    pub fn resize(&self, logical_size: glutin::dpi::LogicalSize) {
        let physical_size =
            glutin::dpi::PhysicalSize::from_logical(logical_size, self.inner.get_hidpi_factor());
        unsafe {
            gl::Viewport(
                0,
                0,
                physical_size.width as i32,
                physical_size.height as i32,
            );
        }
    }
    #[warn(dead_code)]
    pub fn make_current(&self) {
        unsafe {
            self.inner.make_current().unwrap();
        }
    }
    pub fn swap_buffers(&self) -> Result<(), glutin::ContextError> {
        self.inner.swap_buffers()
    }
}
