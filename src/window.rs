use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;

use winit::window::Window as WinitWindow;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{WindowAttributes, WindowId},
};

use crate::core::*;

#[derive(Debug, Clone, Copy)]
pub struct Config<'a> {
    pub title: &'a str,
    pub width: u32,
    pub height: u32,
}

pub struct Buffer<'a> {
    inner: softbuffer::Buffer<'a, Rc<WinitWindow>, Rc<WinitWindow>>,
    size: IVec2,
}

impl<'a> Surface for Buffer<'a> {
    fn get_pixel(&self, pos: IVec2) -> crate::core::Color {
        Color::from_u32(self.inner[Self::index(pos, self.size.x) as usize])
    }

    fn set_pixel(&mut self, pos: IVec2, color: Color) {
        let index = Self::index(pos, self.size.x);
        self.inner[index as usize] = color.as_u32();
    }

    fn size(&self) -> IVec2 {
        self.size
    }
}

impl<'a> Buffer<'a> {
    pub fn present(self) {
        self.inner.present().unwrap()
    }
}

pub struct Window {
    inner: Rc<WinitWindow>,
    surface: softbuffer::Surface<Rc<WinitWindow>, Rc<WinitWindow>>,
    size: IVec2,
}

impl Window {
    fn new(event_loop: &ActiveEventLoop, config: &Config) -> Self {
        assert!(config.width <= i32::MAX as u32 && config.height <= i32::MAX as u32);
        let inner = Rc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title(config.title)
                        .with_inner_size(PhysicalSize::new(config.width, config.height)),
                )
                .unwrap(),
        );
        let context = softbuffer::Context::new(Rc::clone(&inner)).unwrap();
        let surface = softbuffer::Surface::new(&context, Rc::clone(&inner)).unwrap();
        Self {
            inner,
            surface,
            size: ivec2(config.width as i32, config.height as i32),
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        assert!(width < i32::MAX as u32 && height < i32::MAX as u32);
        let Some(width) = NonZeroU32::new(width) else {
            return;
        };
        let Some(height) = NonZeroU32::new(height) else {
            return;
        };
        self.size.x = width.get() as i32;
        self.size.y = height.get() as i32;
        self.surface.resize(width, height).unwrap();
    }

    pub fn buffer(&mut self) -> Buffer {
        Buffer {
            inner: self.surface.buffer_mut().unwrap(),
            size: self.size,
        }
    }

    pub fn size(&self) -> IVec2 {
        self.size
    }
}

pub trait State {
    #[allow(unused)]
    fn resize(&mut self, window: &mut Window, size: IVec2) {}
    fn render(&mut self, window: &mut Window, dt: f32);
}

struct App<'a, S, F> {
    config: Config<'a>,
    window: Option<Window>,
    state: Option<S>,
    state_provider: F,
    last_time: Instant,
    frames: usize,
    frame_time: f32,
}

impl<'a, S, F> App<'a, S, F>
where
    F: Fn(&mut Window) -> S,
    S: State,
{
    fn new(config: Config<'a>, state_provider: F) -> Self {
        Self {
            config,
            window: None,
            state: None,
            state_provider,
            last_time: Instant::now(),
            frames: 0,
            frame_time: 0.0,
        }
    }
}

impl<'a, S, F> ApplicationHandler for App<'a, S, F>
where
    F: Fn(&mut Window) -> S,
    S: State,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        debug_assert!(self.window.is_none());
        let mut window = Window::new(event_loop, &self.config);
        if self.state.is_none() {
            self.state = Some((self.state_provider)(&mut window));
        }
        self.window = Some(window);
        self.last_time = Instant::now();
        self.frames = 0;
        self.frame_time = 0.0;
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        debug_assert!(self.window.is_some());
        self.window = None;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(window) = self.window.as_mut() {
                    window.resize(size.width, size.height);
                    if let Some(state) = self.state.as_mut() {
                        state.resize(window, ivec2(size.width as i32, size.height as i32));
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_mut() {
                    if let Some(state) = self.state.as_mut() {
                        let now = Instant::now();
                        let dt = (now - self.last_time).as_secs_f32();
                        self.last_time = now;
                        state.render(window, dt);

                        self.frames += 1;
                        self.frame_time += dt;
                        while self.frame_time >= 1.0 {
                            self.frame_time -= 1.0;
                            println!("FPS: {}", self.frames);
                            self.frames = 0;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.inner.request_redraw();
        }
    }
}

pub fn run<S, F>(config: Config, state_provider: F)
where
    F: Fn(&mut Window) -> S,
    S: State,
{
    EventLoop::new()
        .unwrap()
        .run_app(&mut App::new(config, state_provider))
        .unwrap();
}
