mod gl_shader;

use gl::types::{GLint, GLuint, GLvoid, GLsizeiptr};
use gl_shader::{shader_from_source, ShaderKind, ShaderProgram};
use glfw::Context;
use std::mem;
use std::ptr;

fn create_quad_vao() -> GLuint {
    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    let vertices: Vec<f32> = vec![-1., -1., 0., 0., -1., 1., 0., 1., 1., -1., 1., 0., 1., 1., 1., 1.];
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::EnableVertexAttribArray(0);
        let stride = (4 * mem::size_of::<f32>()) as GLint;
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride,
            (2 * mem::size_of::<f32>()) as *const GLvoid);
    }
    vao
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

    let mut tex_output: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut tex_output);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex_output);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as GLint, nx, ny, 0, gl::RGBA, gl::FLOAT, ptr::null(),
        );
        gl::BindImageTexture(0, tex_output, 0, gl::FALSE, 0, gl::WRITE_ONLY, gl::RGBA32F);

        let mut x_cnt = 0; let mut y_cnt = 0; let mut z_cnt = 0;
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_COUNT, 0, &mut x_cnt);
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_COUNT, 1, &mut y_cnt);
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_COUNT, 2, &mut z_cnt);
        println!("max work group size x:{}, y:{}, z:{}", x_cnt, y_cnt, z_cnt);

        let mut x_size = 0; let mut y_size = 0; let mut z_size = 0;
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_SIZE, 0, &mut x_size);
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_SIZE, 1, &mut y_size);
        gl::GetIntegeri_v(gl::MAX_COMPUTE_WORK_GROUP_SIZE, 2, &mut z_size);
        println!("max local (in one shader) work group size x:{}, y:{}, z:{}", x_size, y_size, z_size);
    }
        let mut work_grp_inv = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_COMPUTE_WORK_GROUP_INVOCATIONS, &mut work_grp_inv);
        }
        println!("max local work group invocations {}", work_grp_inv);

        let compute_shader = shader_from_source("compute.glsl",
            &std::fs::read("shaders/compute.glsl").unwrap(),
            ShaderKind::Compute
        );
        let compute_shader_program = ShaderProgram::from_shaders(&[compute_shader]);

        let quad_vao = create_quad_vao();
        let quad_vert = shader_from_source("quad.vert",
            &std::fs::read("shaders/quad.vert").unwrap(),
            ShaderKind::Vertex
        );
        let quad_frag = shader_from_source("quad.frag",
            &std::fs::read("shaders/quad.frag").unwrap(),
            ShaderKind::Fragment
        );
        let quad_program = ShaderProgram::from_shaders(&[quad_vert, quad_frag]);

        
        while !window.should_close() {
            compute_shader_program.set_used();
            unsafe {
                gl::DispatchCompute(nx as GLuint, ny as GLuint, 1);
                gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                quad_program.set_used();
                gl::BindVertexArray(quad_vao);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, tex_output);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
            window.swap_buffers();
            glfw.poll_events();
        }
}
