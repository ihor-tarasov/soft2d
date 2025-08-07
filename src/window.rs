use std::collections::HashSet;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::{Duration, Instant};

use winit::keyboard::PhysicalKey;
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
    pub target_fps: Option<u32>,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            title: "Soft2D Window",
            width: 640,
            height: 480,
            target_fps: Some(60),
        }
    }
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

    fn clear(&mut self, color: Color) {
        self.inner.fill(color.as_u32());
    }
}

impl<'a> Buffer<'a> {
    pub fn present(self) {
        self.inner.present().unwrap()
    }
}

pub use winit::keyboard::KeyCode;

pub struct Window {
    inner: Rc<WinitWindow>,
    surface: softbuffer::Surface<Rc<WinitWindow>, Rc<WinitWindow>>,
    size: IVec2,
    key_pressed: HashSet<KeyCode>,
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
            key_pressed: HashSet::new(),
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

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.key_pressed.contains(&key)
    }
}

pub trait State {
    #[allow(unused)]
    fn resize(&mut self, window: &mut Window, size: IVec2) {}
    fn render(&mut self, window: &mut Window, dt: f32);
}

struct App<'a, S> {
    config: Config<'a>,
    window: Option<Window>,
    state: S,
    last_time: Instant,
    frames: usize,
    spend_time: f32,
    target_frame_time: Option<f32>,
}

impl<'a, S> App<'a, S>
where
    S: State,
{
    fn new(config: Config<'a>, state: S) -> Self {
        Self {
            config,
            window: None,
            state,
            last_time: Instant::now(),
            frames: 0,
            spend_time: 0.0,
            target_frame_time: config.target_fps.map(|target_fps| 1.0 / target_fps as f32),
        }
    }
}

impl<'a, S> ApplicationHandler for App<'a, S>
where
    S: State,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        debug_assert!(self.window.is_none());
        self.window = Some(Window::new(event_loop, &self.config));
        self.last_time = Instant::now();
        self.frames = 0;
        self.spend_time = 0.0;
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
                    self.state
                        .resize(window, ivec2(size.width as i32, size.height as i32));
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(window) = self.window.as_mut() {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        if !event.repeat {
                            if event.state.is_pressed() {
                                window.key_pressed.insert(code);
                            } else {
                                window.key_pressed.remove(&code);
                            }
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_mut() {
                    let start = Instant::now();
                    let dt = (start - self.last_time).as_secs_f32();
                    self.last_time = start;

                    self.state.render(window, dt);

                    self.frames += 1;
                    self.spend_time += dt;
                    while self.spend_time >= 1.0 {
                        self.spend_time -= 1.0;
                        println!("FPS: {}", self.frames);
                        self.frames = 0;
                    }

                    if let Some(target_frame_time) = self.target_frame_time {
                        let delta = (Instant::now() - start).as_secs_f32();
                        if delta < target_frame_time {
                            let sleep_time = target_frame_time - delta;
                            std::thread::sleep(Duration::from_secs_f32(sleep_time));
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

pub fn run<S>(config: Config, state: S)
where
    S: State,
{
    EventLoop::new()
        .unwrap()
        .run_app(&mut App::new(config, state))
        .unwrap();
}
