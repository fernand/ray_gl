mod gl_shader;

use gl::types::{GLint, GLuint, GLvoid, GLsizeiptr};
use gl_shader::{shader_from_source, ShaderKind, ShaderProgram, ck};
use glfw::Context;
use std::mem;
use std::ptr;

fn print_workgroup_info() {
    let mut x_cnt = 0; let mut y_cnt = 0; let mut z_cnt = 0;
    unsafe {
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_COUNT, 0, &mut x_cnt);
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_COUNT, 1, &mut y_cnt);
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_COUNT, 2, &mut z_cnt);
    }
    println!("max work group size x:{}, y:{}, z:{}", x_cnt, y_cnt, z_cnt);

    let mut x_size = 0; let mut y_size = 0; let mut z_size = 0;
    unsafe {
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_SIZE, 0, &mut x_size);
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_SIZE, 1, &mut y_size);
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_SIZE, 2, &mut z_size);
    }
    println!("max local (in one shader) work group size x:{}, y:{}, z:{}", x_size, y_size, z_size);

    let mut work_grp_inv = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_COMPUTE_WORK_GROUP_INVOCATIONS, &mut work_grp_inv);
    }
    println!("max local work group invocations {}", work_grp_inv);
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

    print_workgroup_info();

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
            let mut pixels: Vec<f32> = vec![0.; (nx*ny*3) as usize];
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                gl::RGB,
                gl::FLOAT,
                pixels.as_mut_ptr() as *mut GLvoid,
            ); ck();
            println!("{}", pixels[0]);
        }
        window.set_should_close(true);
    }
}
