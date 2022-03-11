
pub fn set_local_storage(key: &str, value: &str) {
    match web_sys::window().unwrap().local_storage().unwrap() {
        Some(local_storage) => {
            local_storage.set_item(key, value).unwrap();
        },
        _ => {}
    }
}
