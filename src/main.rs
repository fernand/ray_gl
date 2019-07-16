#![allow(dead_code)]
#![allow(unused_imports)]
mod gl_shader;

use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use gl_shader::{shader_from_source, Shader, ShaderKind, ShaderProgram};
use std::mem;
use std::ptr;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
}
