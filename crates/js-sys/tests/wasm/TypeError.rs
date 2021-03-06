use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use js_sys::*;

#[wasm_bindgen_test]
fn type_error() {
    let error = TypeError::new("msg");
    assert!(error.is_instance_of::<TypeError>());
    assert!(error.is_instance_of::<Error>());

    let base: &Error = error.as_ref();
    assert_eq!(JsValue::from(base.message()), "msg");
}
