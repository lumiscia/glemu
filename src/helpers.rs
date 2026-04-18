use crate::types::{GLchar, GLenum, GLfloat, GLint, GLsizei};
use std::os::raw::c_char;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as Gl;

pub fn nul_terminated_bytes(mut value: String) -> Vec<u8> {
    value.push('\0');
    value.into_bytes()
}

pub fn supported_extension_bytes(gl: &Gl) -> Vec<Vec<u8>> {
    gl.get_supported_extensions()
        .map(|values| values.to_vec())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|value| value.as_string())
        .map(nul_terminated_bytes)
        .collect()
}

pub fn gl_parameter_string(gl: &Gl, pname: GLenum, fallback: &str) -> Vec<u8> {
    gl.get_parameter(pname)
        .ok()
        .and_then(|value| value.as_string())
        .map(nul_terminated_bytes)
        .unwrap_or_else(|| nul_terminated_bytes(fallback.to_owned()))
}

pub fn js_value_to_glint(value: &JsValue) -> GLint {
    value
        .as_f64()
        .map(|number| number as GLint)
        .or_else(|| value.as_bool().map(|flag| flag as GLint))
        .unwrap_or_default()
}

pub fn js_value_to_glfloat(value: &JsValue) -> GLfloat {
    value
        .as_f64()
        .map(|number| number as GLfloat)
        .unwrap_or_default()
}

pub fn get_parameter_int(gl: &Gl, pname: GLenum) -> Option<GLint> {
    gl.get_parameter(pname)
        .ok()
        .map(|value| js_value_to_glint(&value))
}

pub fn write_glint_params_from_js(pname: GLenum, params: *mut GLint, value: &JsValue) {
    write_scalar_params(pname, params, value, js_value_to_glint);
}

pub fn write_glfloat_params_from_js(pname: GLenum, params: *mut GLfloat, value: &JsValue) {
    write_scalar_params(pname, params, value, js_value_to_glfloat);
}

fn write_scalar_params<T>(
    pname: GLenum,
    params: *mut T,
    value: &JsValue,
    convert: impl Fn(&JsValue) -> T,
) where
    T: Copy + Default,
{
    let count = expected_gl_param_count(pname);
    if params.is_null() {
        return;
    }

    for index in 0..count {
        // SAFETY: `params` is checked for null above and the caller promises that
        // it points to storage for at least `expected_gl_param_count(pname)` items.
        unsafe { *params.add(index) = T::default() };
    }

    if let Some(number) = value.as_f64() {
        // SAFETY: `params` is non-null and points to at least one output item.
        unsafe { *params = convert(&JsValue::from_f64(number)) };
        return;
    }

    if let Some(flag) = value.as_bool() {
        // SAFETY: `params` is non-null and points to at least one output item.
        unsafe { *params = convert(&JsValue::from_bool(flag)) };
        return;
    }

    if let Some(length) = js_array_len(value) {
        let length = length.min(count as u32);
        for index in 0..length {
            if let Some(item) = js_array_item(value, index) {
                // SAFETY: `index < count`, so this write stays within the caller's buffer.
                unsafe { *params.add(index as usize) = convert(&item) };
            }
        }
    }
}

pub fn expected_gl_param_count(pname: GLenum) -> usize {
    match pname {
        0x0BA2 | 0x0C10 => 4,
        0x0D3A | 0x846D | 0x846E => 2,
        _ => 1,
    }
}

fn js_array_len(value: &JsValue) -> Option<u32> {
    js_sys::Reflect::get(value, &JsValue::from_str("length"))
        .ok()
        .and_then(|entry| entry.as_f64())
        .map(|length| length as u32)
}

fn js_array_item(value: &JsValue, index: u32) -> Option<JsValue> {
    js_sys::Reflect::get_u32(value, index).ok()
}

pub fn copy_info_log(bytes: &[u8], max_length: GLsizei, length: *mut GLsizei, buf: *mut GLchar) {
    if max_length <= 0 || buf.is_null() {
        return;
    }

    let copy_len = bytes.len().min(max_length as usize - 1);

    // SAFETY: `buf` is non-null and the caller promises it points to writable
    // storage for `max_length` bytes. `copy_len` is clamped so the trailing NUL
    // stays within bounds. `length` is optional and only written when non-null.
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf.cast::<u8>(), copy_len);
        *buf.add(copy_len) = 0 as c_char;
        if !length.is_null() {
            *length = copy_len as GLsizei;
        }
    }
}

pub fn pixel_byte_size(width: i32, height: i32, format: u32, type_: u32) -> usize {
    let components = match format {
        0x1902 | 0x1909 | 0x1903 | 0x1906 => 1,
        0x190A | 0x8227 => 2,
        0x1907 | 0x8C41 => 3,
        _ => 4,
    };
    let bytes_per_component = match type_ {
        0x1400 | 0x1401 => 1,
        0x140B | 0x8D61 | 0x1402 | 0x1403 | 0x8363 | 0x8033 | 0x8034 => 2,
        0x1404..=0x1406 => 4,
        _ => 1,
    };

    (width.max(0) as usize) * (height.max(0) as usize) * components * bytes_per_component
}

pub fn pixel_byte_size_3d(width: i32, height: i32, depth: i32, format: u32, type_: u32) -> usize {
    pixel_byte_size(width, height, format, type_) * depth.max(0) as usize
}

pub fn is_valid_webgl2_get_parameter(pname: GLenum) -> bool {
    if (0x8825..=0x8834).contains(&pname) {
        return true;
    }

    matches!(
        pname,
        0x84E0
            | 0x846E
            | 0x846D
            | 0x0D55
            | 0x8894
            | 0x0BE2
            | 0x8005
            | 0x80CA
            | 0x80C8
            | 0x8009
            | 0x883D
            | 0x80CB
            | 0x80C9
            | 0x0D54
            | 0x0C22
            | 0x0C23
            | 0x86A3
            | 0x8F36
            | 0x8F37
            | 0x0B44
            | 0x0B45
            | 0x8B8D
            | 0x0D56
            | 0x0B73
            | 0x0B74
            | 0x0B70
            | 0x0B71
            | 0x0B72
            | 0x0BD0
            | 0x8CA6
            | 0x8895
            | 0x8B8B
            | 0x0B46
            | 0x8192
            | 0x0D53
            | 0x8B9B
            | 0x8B9A
            | 0x0B21
            | 0x8073
            | 0x88FF
            | 0x8CDF
            | 0x8A33
            | 0x8B4D
            | 0x8A2E
            | 0x8A31
            | 0x851C
            | 0x8824
            | 0x8D6B
            | 0x80E9
            | 0x80E8
            | 0x9125
            | 0x8A2D
            | 0x8B49
            | 0x8DFD
            | 0x8905
            | 0x84E8
            | 0x8D57
            | 0x9111
            | 0x8872
            | 0x84FD
            | 0x0D33
            | 0x8C8A
            | 0x8C8B
            | 0x8C80
            | 0x8A30
            | 0x8A2F
            | 0x8B4B
            | 0x8DFC
            | 0x8869
            | 0x9122
            | 0x8B4C
            | 0x8A2B
            | 0x8B4A
            | 0x8DFB
            | 0x0D3A
            | 0x821B
            | 0x821C
            | 0x8904
            | 0x86A2
            | 0x821D
            | 0x0D05
            | 0x0D02
            | 0x0D04
            | 0x0D03
            | 0x88ED
            | 0x88EF
            | 0x8038
            | 0x8037
            | 0x2A00
            | 0x8C89
            | 0x0C02
            | 0x8CAA
            | 0x0D52
            | 0x8CA7
            | 0x809E
            | 0x80A8
            | 0x80AB
            | 0x80AA
            | 0x80A9
            | 0x0C10
            | 0x0C11
            | 0x8801
            | 0x8800
            | 0x8802
            | 0x8803
            | 0x8CA3
            | 0x8CA4
            | 0x8CA5
            | 0x0D57
            | 0x0B91
            | 0x0B94
            | 0x0B92
            | 0x0B95
            | 0x0B96
            | 0x0B97
            | 0x0B90
            | 0x0B93
            | 0x0B98
            | 0x0D50
            | 0x8069
            | 0x8C1D
            | 0x806A
            | 0x8514
            | 0x8E24
            | 0x8E25
            | 0x8C8F
            | 0x8E23
            | 0x8A28
            | 0x8A34
            | 0x0CF5
            | 0x9243
            | 0x9240
            | 0x806E
            | 0x9241
            | 0x0CF2
            | 0x806D
            | 0x0CF4
            | 0x0CF3
            | 0x85B5
            | 0x1F00
            | 0x1F02
            | 0x0BA2
    )
}

#[cfg(test)]
mod tests {
    use super::{copy_info_log, is_valid_webgl2_get_parameter, pixel_byte_size};
    use crate::types::GLsizei;
    use std::os::raw::c_char;

    #[test]
    fn pixel_byte_size_handles_common_rgba8_case() {
        assert_eq!(pixel_byte_size(4, 8, 0x1908, 0x1401), 128);
    }

    #[test]
    fn pixel_byte_size_clamps_negative_dimensions() {
        assert_eq!(pixel_byte_size(-1, 8, 0x1908, 0x1401), 0);
    }

    #[test]
    fn copy_info_log_writes_nul_terminated_output() {
        let mut length: GLsizei = -1;
        let mut buffer = vec![0 as c_char; 8];
        copy_info_log(
            b"hello",
            buffer.len() as GLsizei,
            &mut length,
            buffer.as_mut_ptr(),
        );
        let bytes = buffer
            .into_iter()
            .map(|value| value as u8)
            .collect::<Vec<_>>();
        assert_eq!(&bytes[..6], b"hello\0");
        assert_eq!(length, 5);
    }

    #[test]
    fn get_parameter_guard_matches_known_values() {
        assert!(is_valid_webgl2_get_parameter(0x1F02));
        assert!(is_valid_webgl2_get_parameter(0x8825));
        assert!(!is_valid_webgl2_get_parameter(0xDEAD));
    }
}
