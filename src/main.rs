use anyhow::Result;
use std::time::{Duration, Instant};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

// Define the Renderer trait
pub trait Renderer {
    fn initialize(&self) -> Result<()>;
    fn draw_frame(&self) -> Result<()>;
    fn cleanup(&self);
}

// Placeholder struct for VulkanRenderer (implement other backends similarly)
pub struct VulkanRenderer;

impl VulkanRenderer {
    pub fn new() -> Self {
        VulkanRenderer
    }
}

impl Renderer for VulkanRenderer {
    fn initialize(&self) -> Result<()> {
        println!("Initializing Vulkan Renderer...");
        Ok(())
    }

    fn draw_frame(&self) -> Result<()> {
        println!("Drawing frame with Vulkan Renderer...");
        Ok(())
    }

    fn cleanup(&self) {
        println!("Cleaning up Vulkan Renderer...");
    }
}

// Enum to select which backend to use
enum GraphicsBackend {
    Vulkan,
    // Add other backends here as needed: OpenGL, Metal, DirectX
}

// Function to create a renderer based on selected backend
pub fn create_renderer(backend: GraphicsBackend) -> Box<dyn Renderer> {
    match backend {
        GraphicsBackend::Vulkan => Box::new(VulkanRenderer::new()),
        // Add other match arms for additional backends
    }
}

// Function to create a window using winit
pub fn create_window() -> Result<winit::window::Window> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Game Engine")
        .build(&event_loop)?;

    Ok(window)
}

// Game loop setup
pub fn run_game() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Game Engine")
        .build(&event_loop)?;

    let renderer = create_renderer(GraphicsBackend::Vulkan);

    // Initialize the renderer
    renderer.initialize()?;

    // Desired frame rate (e.g., 60 Hz refresh rate)
    let target_frame_duration = Duration::from_secs_f32(1.0 / 60.0);

    let mut last_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                // Time elapsed since the last frame
                let current_time = Instant::now();
                let elapsed = current_time - last_frame_time;

                if elapsed >= target_frame_duration {
                    // Update the last frame time
                    last_frame_time = current_time;

                    // Render the frame
                    if let Err(e) = renderer.draw_frame() {
                        eprintln!("Error rendering frame: {}", e);
                        *control_flow = ControlFlow::Exit;
                    }

                    // Redraw the window
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                // Draw the frame again
                if let Err(e) = renderer.draw_frame() {
                    eprintln!("Error rendering frame: {}", e);
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    renderer.cleanup();
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            _ => (),
        }
    });
}

fn main() {
    if let Err(e) = run_game() {
        eprintln!("Error running game engine: {}", e);
    }
}
