use gl::types::*;
use glm::*;
use image::*;

//TODO use references to types, so no need to constantlly update data with uniformBuffer.data = newData...
pub enum UniformType {
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Matrix4(Mat4),
    Image(Image),
}

pub struct UniformBuffer {
    //Automatically assigned when given to entity (it look for the name in shader and returns the shader_binding num)
    pub shader_binding: i32,
    pub ty: UniformType,
}

impl UniformBuffer {
    pub fn new(shader_binding: i32, ty: UniformType) -> UniformBuffer {
        UniformBuffer { shader_binding, ty }
    }
    pub fn data(&mut self, data: UniformType) {
        self.ty = data;
    }
    #[inline]
    pub fn bind(&self) {
        match self.ty {
            UniformType::Float(data) => unsafe {
                gl::Uniform1f(self.shader_binding as GLint, data as GLfloat);
            },
            UniformType::Vec2(data) => unsafe {
                gl::Uniform2fv(
                    self.shader_binding as GLint,
                    2,
                    data.as_slice().as_ptr() as *const GLfloat,
                )
            },
            UniformType::Vec3(data) => unsafe {
                gl::Uniform2fv(
                    self.shader_binding as GLint,
                    3,
                    data.as_slice().as_ptr() as *const GLfloat,
                )
            },
            UniformType::Vec4(data) => unsafe {
                gl::Uniform2fv(
                    self.shader_binding as GLint,
                    4,
                    data.as_slice().as_ptr() as *const GLfloat,
                )
            },
            UniformType::Matrix4(data) => unsafe {
                gl::UniformMatrix4fv(
                    self.shader_binding as GLint,
                    1,
                    gl::FALSE,
                    data.as_slice().as_ptr() as *const GLfloat,
                )
            },
            //TODO: must unbind texture, or else... ERRORS and ARTEFACTS!
            UniformType::Image(ref data) => unsafe {
                gl::BindTexture(gl::TEXTURE_2D, data.id);
            },
        }
    }
    #[inline]
    pub fn unbind(&self) {
        match self.ty {
            UniformType::Image(_) => unsafe {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            },
            _ => ()
        }
    }
}

pub struct Image {
    id: GLuint,
}
impl Image {
    pub fn new(path: &str) -> ImageResult<Image> {
        let image = open(std::path::Path::new(path))?;
        let im_dimen = image.dimensions();
        let data = image.raw_pixels();
        dbg!(image.color());
        let mut id = 0;
        println!("{}", id);
        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            //INFO: using RGBA8, because image library png uses RGBA(8) format
            gl::TextureStorage2D(id, 1, gl::RGBA8, im_dimen.0 as i32, im_dimen.1 as i32);
            gl::TextureSubImage2D(
                id,
                0,
                0, 0,
                im_dimen.0 as i32, im_dimen.1 as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void,
            );
        }
        Ok(Image { id })
    }
}
impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
