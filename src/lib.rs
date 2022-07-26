use gl::types::GLuint;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::{Context, Viewport};

pub struct FrameBuffer {
    fbo_id: GLuint,
    texture: Texture,
}

impl FrameBuffer {
    pub fn new(texture: Texture) -> Self {
        let fbo_id;
        unsafe {
            let mut fbos: [GLuint; 1] = [0];
            gl::GenFramebuffers(1, fbos.as_mut_ptr());
            fbo_id = fbos[0];
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture.get_id(),
                0,
            );
        }
        FrameBuffer { fbo_id, texture }
    }

    pub fn draw<F, U>(&self, viewport: Viewport, gl: &mut GlGraphics, f: F) -> U
    where F: FnOnce(Context, &mut GlGraphics) -> U {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo_id);
        }

        let res = gl.draw(viewport, f);

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        res
    }

    pub fn texture(&self) -> &Texture {
        return &self.texture;
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::DeleteFramebuffers(1, &self.fbo_id as *const GLuint);
        }
    }
}
