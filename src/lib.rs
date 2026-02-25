use pyo3::prelude::*;
mod lib_str;

#[pymodule]
mod pyrs_str {
    use crate::lib_str;
    use pyo3::pyfunction;
    use std::borrow::Cow;

    #[pyfunction]
    fn str_simplify(s: &str) -> Cow<'_, str> {
        lib_str::str_simplify(s)
    }
}
