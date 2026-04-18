use core::hash::Hash;
use core::num::NonZeroU32;
use std::os::raw::c_char;

pub type GLboolean = u8;
pub type GLbitfield = u32;
pub type GLchar = c_char;
pub type GLclampf = f32;
pub type GLenum = u32;
pub type GLfloat = f32;
pub type GLint = i32;
pub type GLintptr = i32;
pub type GLsizei = i32;
pub type GLsizeiptr = i32;
pub type GLuint = u32;

pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE: GLboolean = 1;
pub const GL_EXTENSIONS: GLenum = 0x1F03;
pub const GL_NUM_EXTENSIONS: GLenum = 0x821D;

pub trait ResourceHandle: Copy + Eq + Hash {
    fn new(raw: GLuint) -> Self;
    fn get(self) -> GLuint;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ContextId(NonZeroU32);

impl ContextId {
    pub fn new(raw: u32) -> Option<Self> {
        NonZeroU32::new(raw).map(Self)
    }

    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

impl From<ContextId> for u32 {
    fn from(value: ContextId) -> Self {
        value.get()
    }
}

macro_rules! define_handle {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[repr(transparent)]
        pub struct $name(pub GLuint);

        impl $name {
            pub const fn null() -> Self {
                Self(0)
            }

            pub const fn get(self) -> GLuint {
                self.0
            }

            pub const fn is_null(self) -> bool {
                self.0 == 0
            }

            pub fn into_option(self) -> Option<Self> {
                (!self.is_null()).then_some(self)
            }
        }

        impl ResourceHandle for $name {
            fn new(raw: GLuint) -> Self {
                Self(raw)
            }

            fn get(self) -> GLuint {
                self.0
            }
        }

        impl From<GLuint> for $name {
            fn from(value: GLuint) -> Self {
                Self(value)
            }
        }

        impl From<$name> for GLuint {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

define_handle!(Texture);
define_handle!(Buffer);
define_handle!(Program);
define_handle!(Shader);
define_handle!(Framebuffer);
define_handle!(Renderbuffer);
define_handle!(VertexArray);
define_handle!(Query);
define_handle!(Sampler);
define_handle!(SyncHandle);
define_handle!(UniformLocation);
define_handle!(TransformFeedback);

#[cfg(test)]
mod tests {
    use super::{ContextId, ResourceHandle, Texture};

    #[test]
    fn context_ids_are_non_zero() {
        assert!(ContextId::new(0).is_none());
        assert_eq!(ContextId::new(7).unwrap().get(), 7);
    }

    #[test]
    fn resource_handles_track_nullability() {
        let null = Texture::null();
        let value = Texture::new(9);
        assert!(null.is_null());
        assert!(null.into_option().is_none());
        assert_eq!(value.get(), 9);
        assert_eq!(value.into_option().unwrap().get(), 9);
    }
}
