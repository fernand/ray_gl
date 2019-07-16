#![allow(dead_code)]
#![allow(unused_imports)]
mod gl_shader;

use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use gl_shader::{shader_from_source, Shader, ShaderKind, ShaderProgram};
use glfw::Context;
use std::mem;
use std::ptr;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut window, _) = glfw
        .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();
    gl::load_with(|s| glfw.get_proc_address_raw(s));
}
