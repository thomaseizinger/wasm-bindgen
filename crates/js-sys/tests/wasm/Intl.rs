use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen_test]
fn get_canonical_locales() {
    let locales = Array::new();
    locales.push(&"EN-US".into());
    locales.push(&"Fr".into());
    let locales = JsValue::from(locales);
    let canonical_locales = Intl::get_canonical_locales(&locales);
    assert_eq!(canonical_locales.length(), 2);
    canonical_locales.for_each(&mut |l, i, _| {
        if i == 0 {
            assert_eq!(l, "en-US");
        } else {
            assert_eq!(l, "fr");
        }
    });
    let canonical_locales = Intl::get_canonical_locales(&"EN-US".into());
    assert_eq!(canonical_locales.length(), 1);
    canonical_locales.for_each(&mut |l, _, _| {
        assert_eq!(l, "en-US");
    });
}

#[wasm_bindgen_test]
fn collator() {
    let locales = Array::of1(&JsValue::from("en-US"));
    let opts = Object::new();

    let c = Intl::Collator::new(&locales, &opts);
    assert!(c.compare().is_instance_of::<Function>());
    assert!(c.resolved_options().is_instance_of::<Object>());

    let a = Intl::Collator::supported_locales_of(&locales, &opts);
    assert!(a.is_instance_of::<Array>());
}
