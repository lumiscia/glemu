use super::*;

pub(crate) unsafe extern "C" fn gl_delete_queries(n: GLsizei, ids: *const GLuint) {
    with_gl(|s| {
        for i in 0..n as usize {
            let id = unsafe { *ids.add(i) };
            if let Some(query) = s.queries.remove(&crate::types::Query::from(id)) {
                s.gl.delete_query(Some(&query));
            }
        }
    });
}

gen_objects!(gl_gen_queries, create_query, queries, crate::types::Query);

pub(crate) unsafe extern "C" fn gl_begin_query(target: GLenum, id: GLuint) {
    with_gl(|s| {
        if let Some(query) = s.queries.get(&crate::types::Query::from(id)) {
            s.gl.begin_query(target, query);
        }
    });
}

pub(crate) unsafe extern "C" fn gl_end_query(target: GLenum) {
    with_gl(|s| s.gl.end_query(target));
}

pub(crate) unsafe extern "C" fn gl_get_query_object_uiv(
    id: GLuint,
    pname: GLenum,
    params: *mut GLuint,
) {
    with_gl(|s| {
        if let Some(query) = s.queries.get(&crate::types::Query::from(id)) {
            let value = s.gl.get_query_parameter(query, pname);
            let value = if let Some(number) = value.as_f64() {
                number as GLuint
            } else if let Some(flag) = value.as_bool() {
                flag as GLuint
            } else {
                return;
            };
            unsafe { *params = value };
        }
    });
}
