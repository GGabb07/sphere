mod camera;
mod shader;
mod terrgen;
mod vertex_array;

use crate::{camera::Camera, shader::Shader, vertex_array::VertexArray};
use glfw::*;
use nalgebra_glm::Mat4;

fn main() {
    let (mut glfw, mut window, events) = create_window();

    gl::load_with(|fn_name| window.get_proc_address(fn_name));

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0., 0., 0., 1.);
    }
    let vao = VertexArray::new();
    let prog = Shader::new();

    let mut cam = Camera::new(6., 0.005);
    let mut proj = Mat4::identity();

    let mut filled = true;

    let mut start = glfw.get_time();
    let mut end;
    let mut delta = 0.;

    const TARGET_FPS: f64 = 60.;
    const OPTIMAL_TIME: f64 = 1. / TARGET_FPS;

    let mut fps: u8 = 0;
    let mut timer = glfw.get_time();

    let mut old_x = 0.;
    let mut old_y = 0.;

    let mut cursor_locked = true;
    window.set_cursor_mode(CursorMode::Disabled);
    window.show();

    'main_loop: loop {
        end = start;
        start = glfw.get_time();
        delta += start - end;

        if delta >= OPTIMAL_TIME {
            glfw.poll_events();
            for (_, event) in flush_messages(&events) {
                match event {
                    WindowEvent::CursorPos(x_pos, y_pos) => {
                        cam.rotate((x_pos - old_x) as f32, (y_pos - old_y) as f32);
                        old_x = x_pos;
                        old_y = y_pos;
                    }
                    WindowEvent::FramebufferSize(w, h) => unsafe {
                        gl::Viewport(0, 0, w, h);
                    },
                    WindowEvent::Close => break 'main_loop,
                    WindowEvent::Key(key, _, act, _) => {
                        if let Action::Press = act {
                            if let Key::Enter = key {
                                unsafe {
                                    gl::PolygonMode(
                                        gl::FRONT_AND_BACK,
                                        if filled {
                                            filled = false;
                                            gl::LINE
                                        } else {
                                            filled = true;
                                            gl::FILL
                                        },
                                    );
                                }
                            } else if let Key::E = key {
                                window.set_cursor_mode(if cursor_locked {
                                    cursor_locked = false;
                                    CursorMode::Normal
                                } else {
                                    cursor_locked = true;
                                    CursorMode::Disabled
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
            cam.update(delta as f32, &window, &mut proj);

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                prog.use_program();
                prog.set_proj(&proj);

                vao.render();

                gl::UseProgram(0);
                gl::BindVertexArray(0);
            }

            window.swap_buffers();

            fps += 1;
            delta -= OPTIMAL_TIME;
        }

        if glfw.get_time() - timer >= 1. {
            println!("FPS: {fps}");
            fps = 0;
            timer += 1.;
        }
    }
}

fn create_window() -> (Glfw, PWindow, GlfwReceiver<(f64, WindowEvent)>) {
    let mut glfw = init(fail_on_errors).expect("Could not initiazlie GLFW");

    glfw.default_window_hints();
    glfw.window_hint(WindowHint::Resizable(true));
    glfw.window_hint(WindowHint::Visible(false));

    let (mut window, events) = glfw
        .create_window(1280, 720, "Window Creation", WindowMode::Windowed)
        .expect("Could not create the GLFW Window");

    window.set_all_polling(true);
    window.make_current();
    glfw.set_swap_interval(SwapInterval::Sync(1));

    (glfw, window, events)
}
