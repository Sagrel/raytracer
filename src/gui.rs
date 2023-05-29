#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::{
    sync::{Arc, Mutex},
    thread,
};

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{CursorGrabMode, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    bvh::Bvh, camera::Camera, config::Config, raytrace::raytrace_in_place, scene::Scene, Real,
    Vector,
};

struct UiState {
    pub pixels: Pixels,
    // Keep the dimensions in a enum to indicate that it has been modified?
    pub size: PhysicalSize<u32>,
    pub pitch: Real,
    pub pos: Vector,
    pub fov: Real,
    pub yaw: Real, // TODO Add more fields to customize the experience
    pub reload: bool,
}

fn worker_thread(state: Arc<Mutex<UiState>>, mut config: Config) {
    let mut samples = 0;
    let mut image = Vec::new();
    let scene = Scene::read_scene(&config.scene);
    let bvh = Bvh::new(&scene.shapes);

    loop {
        let camera = {
            let mut state = state.lock().unwrap();
            // Check if the state has changed
            // TODO this is not the best way of doing it probably...
            if state.reload {
                // TODO Modifying the config feels kind of dirty tbh
                config.width = state.size.width as usize;
                config.height = state.size.height as usize;
                let num_pixels = (state.size.width * state.size.height) as usize;
                let size = state.size;
                state.pixels.resize_buffer(size.width, size.height).unwrap();
                state
                    .pixels
                    .resize_surface(size.width, size.height)
                    .unwrap();
                if image.len() != num_pixels {
                    image = vec![Vector::default(); num_pixels];
                } else {
                    for pixel in image.iter_mut() {
                        *pixel = Vector::default()
                    }
                }
                samples = 0;
                state.reload = false;
            } else {
                // Display whatever we have already
                render_to_buffer(state.pixels.frame_mut(), &image, samples);
                state.pixels.render().unwrap();
            }
            Camera::new_angles(
                state.fov,
                state.pitch,
                state.yaw,
                state.pos,
                state.size.width as Real / state.size.height as Real,
            )
        };
        // Raytrace
        raytrace_in_place(&mut image[..], &config, &scene, &camera, &bvh);
        samples += 1;
    }
}

fn render_to_buffer(screen: &mut [u8], image: &[Vector], samples: usize) {
    for (image_pixel, screen_pixel) in image.iter().zip(screen.chunks_exact_mut(4)) {
        let corrected_color = *image_pixel / samples as Real * 255.0;
        screen_pixel.copy_from_slice(&[
            corrected_color.x as u8,
            corrected_color.y as u8,
            corrected_color.z as u8,
            u8::MAX,
        ])
    }
}

pub fn gui_mode(config: Config) -> Result<(), Error> {
    // TODO include EGUI support for debuggin and stuff
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let size = PhysicalSize::new(config.width as u32, config.height as u32);
    let window = {
        WindowBuilder::new()
            .with_title("Simple raytracer")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    // NOTE: This does not work in WSL...
    window.set_cursor_grab(CursorGrabMode::Confined).unwrap();
    window.set_cursor_visible(false);

    let state = {
        let pixels = Pixels::new(
            size.width,
            size.height,
            SurfaceTexture::new(size.width, size.height, &window),
        )?;

        Arc::new(Mutex::new(UiState {
            pixels,
            size,
            pitch: 0.0,
            yaw: 0.0,
            fov: 20.0,
            reload: true,
            pos: Vector::new(13.0, 2.0, 3.0),
        }))
    };

    let state_clone = state.clone();
    let mut mouse_enabled = false;

    thread::spawn(move || worker_thread(state_clone, config));

    event_loop.run(move |event, _, control_flow| {
        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.

        if !mouse_enabled {
            if let winit::event::Event::DeviceEvent {
                device_id: _,
                event:
                    DeviceEvent::MouseMotion {
                        delta: (x_diff, y_diff),
                    },
            } = event
            {
                if x_diff.abs() > f64::EPSILON || y_diff.abs() > f64::EPSILON {
                    let mut state = state.lock().unwrap();

                    state.pitch += y_diff as Real / 5.0;
                    state.yaw += x_diff as Real / 5.0;
                    state.reload = true;
                }
            }
        }

        if input.update(&event) {
            // Close events
            if input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize event
            if let Some(size) = input.window_resized() {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.size = size;
            }

            // Keyboard events
            if input.key_pressed(VirtualKeyCode::Escape) {
                mouse_enabled = !mouse_enabled;
                window.set_cursor_visible(mouse_enabled);
                window
                    .set_cursor_grab(if mouse_enabled {
                        CursorGrabMode::None
                    } else {
                        CursorGrabMode::Confined
                    })
                    .unwrap();
            }

            // TODO handle keyboard movement?
            if input.key_pressed(VirtualKeyCode::Q) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.fov -= 5.0;
            }
            if input.key_pressed(VirtualKeyCode::E) {
                let mut state = state.lock().unwrap();
                state.reload = true;
                state.fov += 5.0;
            }
        }
    });
}
