#![windows_subsystem = "windows"]

mod state_renderer;
mod game_state;

use glium::glutin;
use winit::event::{WindowEvent, Event, DeviceEvent, VirtualKeyCode, ElementState};
use winit::event_loop::ControlFlow;
use std::time::Instant;
use glium::Surface;
use crate::game_state::GameState;
use crate::state_renderer::render_state;
use std::fs::File;
use std::borrow::Borrow;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("2048");
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context, &event_loop).unwrap();

    let mut frame_counter: u32 = 0;
    let mut frame_start = Instant::now();
    let mut frame_duration = Instant::now().duration_since(frame_start).as_millis() as u64;

    let mut game_state = GameState::init();
    let font = glium_text_nxt::FontTexture::new(&display, File::open("ClearSans-Medium.ttf").unwrap(), 24).unwrap();
    let mut in_focus = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        if frame_duration >= 16 {
            frame_counter += 1;

            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
            render_state(&game_state, &display, &mut target, font.borrow());
            target.finish().unwrap();
            frame_start = Instant::now();
        }

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::Resized(size) => display.gl_window().resize(size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Focused(focused) => in_focus = focused,
                    _ => (),
                }
            },
            Event::DeviceEvent {event, ..} => {
                match event {
                    DeviceEvent::Key(input) => {
                        if in_focus && input.state == ElementState::Pressed {
                            match input.virtual_keycode.unwrap() {
                                VirtualKeyCode::Left => {
                                    game_state.shift_left();
                                    if game_state.changed_this_turn {
                                        game_state.add_tile();
                                        game_state.clear_changed_flag();
                                    }

                                    if game_state.no_more_turns() {
                                        game_state = game_state.restart();
                                    }
                                },
                                VirtualKeyCode::Right => {
                                    game_state.shift_right();
                                    if game_state.changed_this_turn {
                                        game_state.add_tile();
                                        game_state.clear_changed_flag();
                                    }

                                    if game_state.no_more_turns() {
                                        game_state = game_state.restart();
                                    }
                                },
                                VirtualKeyCode::Up => {
                                    game_state.shift_up();
                                    if game_state.changed_this_turn {
                                        game_state.add_tile();
                                        game_state.clear_changed_flag();
                                    }

                                    if game_state.no_more_turns() {
                                        game_state = game_state.restart();
                                    }
                                },
                                VirtualKeyCode::Down => {
                                    game_state.shift_down();
                                    if game_state.changed_this_turn {
                                        game_state.add_tile();
                                        game_state.clear_changed_flag();
                                    }

                                    if game_state.no_more_turns() {
                                        game_state = game_state.restart();
                                    }
                                },
                                VirtualKeyCode::R => game_state = game_state.restart(),
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            },
            Event::RedrawRequested(_) => {
                display.swap_buffers();
            },
            _ => (),
        }

        frame_duration = Instant::now().duration_since(frame_start).as_millis() as u64;
    });
}
