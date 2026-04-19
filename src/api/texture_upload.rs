use super::context::Context;
use wasm_bindgen::JsValue;
#[cfg(feature = "image-data")]
use web_sys::ImageData;
#[cfg(feature = "video-frame")]
use web_sys::VideoFrame;
#[cfg(feature = "dom-uploads")]
use web_sys::{HtmlCanvasElement, HtmlImageElement, HtmlVideoElement, ImageBitmap};

macro_rules! impl_tex_image_2d_uploads {
    (
        $(#[$meta:meta])*
        $base_name:ident,
        $sized_name:ident,
        $source_ty:ty,
        $source_arg:ident,
        $upload_method:ident,
        $sized_upload_method:ident
    ) => {
        $(#[$meta])*
        pub fn $base_name(
            &self,
            target: u32,
            level: i32,
            internal_format: i32,
            format: u32,
            ty: u32,
            $source_arg: &$source_ty,
        ) -> Result<(), JsValue> {
            self.with_state(|state| {
                state.gl.$upload_method(
                    target,
                    level,
                    internal_format,
                    format,
                    ty,
                    $source_arg,
                )
            })
        }

        $(#[$meta])*
        pub fn $sized_name(
            &self,
            target: u32,
            level: i32,
            internal_format: i32,
            width: i32,
            height: i32,
            format: u32,
            ty: u32,
            $source_arg: &$source_ty,
        ) -> Result<(), JsValue> {
            self.with_state(|state| {
                state.gl.$sized_upload_method(
                    target,
                    level,
                    internal_format,
                    width,
                    height,
                    0,
                    format,
                    ty,
                    $source_arg,
                )
            })
        }
    };
}

macro_rules! impl_tex_sub_image_2d_uploads {
    (
        $(#[$meta:meta])*
        $base_name:ident,
        $sized_name:ident,
        $source_ty:ty,
        $source_arg:ident,
        $upload_method:ident,
        $sized_upload_method:ident
    ) => {
        $(#[$meta])*
        pub fn $base_name(
            &self,
            target: u32,
            level: i32,
            x_offset: i32,
            y_offset: i32,
            format: u32,
            ty: u32,
            $source_arg: &$source_ty,
        ) -> Result<(), JsValue> {
            self.with_state(|state| {
                state
                    .gl
                    .$upload_method(target, level, x_offset, y_offset, format, ty, $source_arg)
            })
        }

        $(#[$meta])*
        pub fn $sized_name(
            &self,
            target: u32,
            level: i32,
            x_offset: i32,
            y_offset: i32,
            width: i32,
            height: i32,
            format: u32,
            ty: u32,
            $source_arg: &$source_ty,
        ) -> Result<(), JsValue> {
            self.with_state(|state| {
                state.gl.$sized_upload_method(
                    target,
                    level,
                    x_offset,
                    y_offset,
                    width,
                    height,
                    format,
                    ty,
                    $source_arg,
                )
            })
        }
    };
}

impl Context {
    #[cfg(feature = "dom-uploads")]
    impl_tex_image_2d_uploads!(
        tex_image_2d_with_image_bitmap,
        tex_image_2d_with_image_bitmap_and_width_and_height,
        ImageBitmap,
        image_bitmap,
        tex_image_2d_with_u32_and_u32_and_image_bitmap,
        tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_image_bitmap
    );

    #[cfg(feature = "dom-uploads")]
    impl_tex_image_2d_uploads!(
        tex_image_2d_with_html_canvas,
        tex_image_2d_with_html_canvas_and_width_and_height,
        HtmlCanvasElement,
        canvas,
        tex_image_2d_with_u32_and_u32_and_html_canvas_element,
        tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_canvas_element
    );

    #[cfg(feature = "dom-uploads")]
    impl_tex_image_2d_uploads!(
        tex_image_2d_with_html_image,
        tex_image_2d_with_html_image_and_width_and_height,
        HtmlImageElement,
        image,
        tex_image_2d_with_u32_and_u32_and_html_image_element,
        tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_image_element
    );

    #[cfg(feature = "dom-uploads")]
    impl_tex_image_2d_uploads!(
        tex_image_2d_with_html_video,
        tex_image_2d_with_html_video_and_width_and_height,
        HtmlVideoElement,
        video,
        tex_image_2d_with_u32_and_u32_and_html_video_element,
        tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_video_element
    );

    #[cfg(feature = "image-data")]
    impl_tex_image_2d_uploads!(
        tex_image_2d_with_image_data,
        tex_image_2d_with_image_data_and_width_and_height,
        ImageData,
        image_data,
        tex_image_2d_with_u32_and_u32_and_image_data,
        tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_image_data
    );

    impl_tex_image_2d_uploads!(
        #[cfg(feature = "video-frame")]
        tex_image_2d_with_video_frame,
        tex_image_2d_with_video_frame_and_width_and_height,
        VideoFrame,
        video_frame,
        tex_image_2d_with_u32_and_u32_and_video_frame,
        tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_video_frame
    );

    #[cfg(feature = "dom-uploads")]
    impl_tex_sub_image_2d_uploads!(
        tex_sub_image_2d_with_image_bitmap,
        tex_sub_image_2d_with_image_bitmap_and_width_and_height,
        ImageBitmap,
        image_bitmap,
        tex_sub_image_2d_with_u32_and_u32_and_image_bitmap,
        tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_image_bitmap
    );

    #[cfg(feature = "dom-uploads")]
    impl_tex_sub_image_2d_uploads!(
        tex_sub_image_2d_with_html_canvas,
        tex_sub_image_2d_with_html_canvas_and_width_and_height,
        HtmlCanvasElement,
        canvas,
        tex_sub_image_2d_with_u32_and_u32_and_html_canvas_element,
        tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_html_canvas_element
    );

    #[cfg(feature = "dom-uploads")]
    impl_tex_sub_image_2d_uploads!(
        tex_sub_image_2d_with_html_image,
        tex_sub_image_2d_with_html_image_and_width_and_height,
        HtmlImageElement,
        image,
        tex_sub_image_2d_with_u32_and_u32_and_html_image_element,
        tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_html_image_element
    );

    #[cfg(feature = "dom-uploads")]
    impl_tex_sub_image_2d_uploads!(
        tex_sub_image_2d_with_html_video,
        tex_sub_image_2d_with_html_video_and_width_and_height,
        HtmlVideoElement,
        video,
        tex_sub_image_2d_with_u32_and_u32_and_html_video_element,
        tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_html_video_element
    );

    #[cfg(feature = "image-data")]
    impl_tex_sub_image_2d_uploads!(
        tex_sub_image_2d_with_image_data,
        tex_sub_image_2d_with_image_data_and_width_and_height,
        ImageData,
        image_data,
        tex_sub_image_2d_with_u32_and_u32_and_image_data,
        tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_image_data
    );

    impl_tex_sub_image_2d_uploads!(
        #[cfg(feature = "video-frame")]
        tex_sub_image_2d_with_video_frame,
        tex_sub_image_2d_with_video_frame_and_width_and_height,
        VideoFrame,
        video_frame,
        tex_sub_image_2d_with_u32_and_u32_and_video_frame,
        tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_video_frame
    );
}
