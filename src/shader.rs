use std::ffi::CString;

use nalgebra_glm::Mat4;

pub struct Shader {
    vert: u32,
    frag: u32,
    prog: u32,
    proj_loc: i32,
}

impl Shader {
    pub fn new() -> Self {
        let (vert, frag);
        let prog;
        let proj_loc;

        unsafe {
            const VERT_SHADER: &str = include_str!("shaders/vert.glsl");
            vert = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(0, vert);
            gl::ShaderSource(
                vert,
                1,
                &(VERT_SHADER.as_bytes().as_ptr().cast()),
                &(VERT_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(vert);
            let mut success = 0;
            gl::GetShaderiv(vert, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(vert, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len as usize);
                println!("Vertex Shader Error: {}", String::from_utf8_lossy(&v));
            }

            const FRAG_SHADER: &str = include_str!("shaders/frag.glsl");
            frag = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(0, frag);
            gl::ShaderSource(
                frag,
                1,
                &(FRAG_SHADER.as_bytes().as_ptr().cast()),
                &(VERT_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(frag);
            gl::GetShaderiv(frag, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(frag, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len as usize);
                println!("Frag Shader Error: {}", String::from_utf8_lossy(&v));
            }

            prog = gl::CreateProgram();
            gl::AttachShader(prog, vert);
            gl::AttachShader(prog, frag);
            gl::LinkProgram(prog);
            gl::GetProgramiv(prog, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(prog, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len as usize);
                println!("Prog Link Error: {}", String::from_utf8_lossy(v.as_slice()));
            }
            gl::ValidateProgram(prog);
            gl::GetProgramiv(prog, gl::VALIDATE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(prog, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len as usize);
                println!("Prog Validate Error: {}", String::from_utf8_lossy(&v));
            }

            let name = CString::new("proj").unwrap();
            proj_loc = gl::GetUniformLocation(prog, name.as_ptr());
        }

        Self {
            vert,
            frag,
            prog,
            proj_loc,
        }
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.prog) }
    }

    pub fn set_proj(&self, proj: &Mat4) {
        unsafe { gl::UniformMatrix4fv(self.proj_loc, 1, gl::FALSE, proj.as_ptr()) }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.vert);
            gl::DeleteShader(self.frag);
            gl::DeleteProgram(self.prog);
        }
    }
}
