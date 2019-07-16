use gl::types::{GLchar, GLint, GLuint};
use std::ptr;

#[derive(Debug)]
pub enum ShaderKind {
    Vertex,
    Fragment,
    Compute,
}

pub struct Shader {
    pub id: GLuint,
}

pub struct ShaderProgram {
    pub id: GLuint,
}

impl ShaderProgram {
    pub fn from_shaders(shaders: &[Shader]) -> ShaderProgram {
        let program_id;
        unsafe {
            program_id = gl::CreateProgram(); ck();
            for shader in shaders {
                gl::AttachShader(program_id, shader.id); ck();
            }
            gl::LinkProgram(program_id); ck();
            let mut program_status = 0;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut program_status); ck();
            if program_status != 1 {
                let mut info_log_length = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut info_log_length); ck();
                let mut info_log = vec![0; info_log_length as usize];
                gl::GetProgramInfoLog(
                    program_id,
                    info_log_length,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                ); ck();
                println!("Shader info log\n{}", String::from_utf8_lossy(&info_log));
                panic!("program '{}' link failed", program_id);
            }
            for shader in shaders {
                gl::DetachShader(program_id, shader.id); ck();
            }
        }
        ShaderProgram { id: program_id }
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
            ck();
        }
    }
}

pub fn shader_from_source(name: &str, source: &[u8], kind: ShaderKind) -> Shader {
    let gl_shader_kind = match kind {
        ShaderKind::Vertex => gl::VERTEX_SHADER,
        ShaderKind::Fragment => gl::FRAGMENT_SHADER,
        ShaderKind::Compute => gl::COMPUTE_SHADER,
    };

    unsafe {
        let shader_id = gl::CreateShader(gl_shader_kind); ck();
        gl::ShaderSource(
            shader_id,
            1,
            [source.as_ptr() as *const GLchar].as_ptr(),
            [source.len() as GLint].as_ptr(),
        ); ck();
        gl::CompileShader(shader_id); ck();

        let mut compile_status = 0;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compile_status); ck();
        if compile_status != gl::TRUE as GLint {
            let mut info_log_length = 0;
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length); ck();
            let mut info_log = vec![0; info_log_length as usize];
            gl::GetShaderInfoLog(
                shader_id,
                info_log.len() as GLint,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            ); ck();
            println!("Shader info log\n{}", String::from_utf8_lossy(&info_log));
            panic!("{:?} shader '{}' compilation failed", kind, name);
        }
        Shader { id: shader_id }
    }
}

fn ck() {
    unsafe {
        // Note that ideally we should be calling gl::GetError() in a loop until it
        // returns gl::NO_ERROR, but for now we'll just report the first one we find.
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            panic!(
                "GL error: 0x{:x} ({})",
                err,
                match err {
                    gl::INVALID_ENUM => "INVALID_ENUM",
                    gl::INVALID_VALUE => "INVALID_VALUE",
                    gl::INVALID_OPERATION => "INVALID_OPERATION",
                    gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                    gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                    gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                    gl::STACK_OVERFLOW => "STACK_OVERFLOW",
                    _ => "Unknown",
                }
            );
        }
    }
}
