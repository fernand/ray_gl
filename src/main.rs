#![allow(dead_code)]
#![allow(unused_imports)]
mod gl_shader;

use gl::types::{GLint, GLuint, GLvoid, GLsizeiptr};
use gl_shader::{shader_from_source, ShaderKind, ShaderProgram, ck};
use glfw::Context;
use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;
use std::mem;
use std::ptr;

fn write_png(nx: i32, ny: i32, floats: Vec<f32>) -> std::io::Result<()> {
    let file = File::create("image.png")?;
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut pixels: Vec<u8> = Vec::new();
    for col in floats.iter() {
        pixels.push((255.99 * col.sqrt()) as u8);
    }
    writer.write_image_data(&pixels)?;
    Ok(())
}

fn get_image_buffer(nx: i32, ny: i32) -> GLuint {
    let mut texture: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as GLint, nx, ny, 0, gl::RGBA, gl::FLOAT, ptr::null());
        gl::BindImageTexture(0, texture, 0, gl::FALSE, 0, gl::WRITE_ONLY, gl::RGBA32F);
    }
    texture
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let nx: i32 = 2000;
    let ny: i32 = 1000;

    let (mut window, _) = glfw
        .create_window(nx as u32, ny as u32, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    //print_workgroup_info();

    let texture = get_image_buffer(nx, ny);

    let compute_shader = shader_from_source("compute.glsl",
        &std::fs::read("shaders/compute.glsl").unwrap(),
        ShaderKind::Compute
    );
    let compute_shader_program = ShaderProgram::from_shaders(&[compute_shader]);

    while !window.should_close() {
        compute_shader_program.set_used();
        unsafe {
            gl::DispatchCompute(nx as GLuint, ny as GLuint, 1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
            let mut img_data: Vec<f32> = vec![0.; (nx*ny*3) as usize];
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                gl::RGB,
                gl::FLOAT,
                img_data.as_mut_ptr() as *mut GLvoid,
            ); ck();
            write_png(nx, ny, img_data).unwrap();
        }
        window.set_should_close(true);
    }
}
