use pyo3::prelude::*;

#[pyclass]
struct MyClass {}

#[pymethods]
impl MyClass {
    #[classattr]
    fn class_attr_with_args(foo: i32) {}
}

#[pymethods]
impl MyClass {
    #[classattr(foobar)]
    const CLASS_ATTR_WITH_ATTRIBUTE_ARG: i32 = 3;
}

#[pymethods]
impl MyClass {
    fn staticmethod_without_attribute() {}
}

#[pymethods]
impl MyClass {
    #[staticmethod]
    fn staticmethod_with_receiver(&self) {}
}

// FIXME: This currently doesn't fail
// #[pymethods]
// impl MyClass {
//     #[classmethod]
//     fn classmethod_with_receiver(&self) {}
// }

#[pymethods]
impl MyClass {
    #[getter(x)]
    fn getter_without_receiver() {}
}

#[pymethods]
impl MyClass {
    #[setter(x)]
    fn setter_without_receiver() {}
}

#[pymethods]
impl MyClass {
    #[new]
    #[text_signature = "()"]
    fn text_signature_on_new() {}
}

#[pymethods]
impl MyClass {
    #[call]
    #[text_signature = "()"]
    fn text_signature_on_call(&self) {}
}

#[pymethods]
impl MyClass {
    #[getter(x)]
    #[text_signature = "()"]
    fn text_signature_on_getter(&self) {}
}

#[pymethods]
impl MyClass {
    #[setter(x)]
    #[text_signature = "()"]
    fn text_signature_on_setter(&self) {}
}

#[pymethods]
impl MyClass {
    #[classattr]
    #[text_signature = "()"]
    fn text_signature_on_classattr() {}
}

#[pymethods]
impl MyClass {
    #[classattr]
    #[staticmethod]
    fn multiple_method_types() {}
}

#[pymethods]
impl MyClass {
    fn generic_method<T>(value: T) {}
}


#[pymethods]
impl MyClass {
    fn impl_trait_method_first_arg(impl_trait: impl AsRef<PyAny>) {}
}

#[pymethods]
impl MyClass {
    fn impl_trait_method_second_arg(&self, impl_trait: impl AsRef<PyAny>) {}
}

#[pymethods]
impl MyClass {
    async fn async_method(&self) {}
}

fn main() {}
