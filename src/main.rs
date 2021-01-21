// opaque type
#[repr(C)]
struct CppClass {
    _priv: [usize; 0],
}

extern "C" {
    fn cpp_class_new() -> *mut CppClass;
    fn cpp_class_delete(obj: *mut CppClass);
    fn cpp_class_set_callback_fn(obj: *mut CppClass, cb: extern "C" fn(&mut RustStruct));
    fn cpp_class_set_callback_arg(obj: *mut CppClass, arg: &mut RustStruct);
    fn cpp_class_fire_callback(obj: *mut CppClass);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct RustStruct {
    x: u32
}

impl RustStruct {
    fn callback(&mut self) {
        let prev = self.x;
        self.x += 1;

        println!("Callback fired! {} -> {}", prev, self.x);
    }
}

#[no_mangle]
extern fn rust_struct_callback(this: &mut RustStruct) {
    this.callback();
    // or:
    RustStruct::callback(this);
}

fn main() {
    let mut rust_obj = RustStruct { x: 1337 };

    unsafe {
        let cpp_obj = cpp_class_new();
        cpp_class_set_callback_fn(cpp_obj, rust_struct_callback);
        cpp_class_set_callback_arg(cpp_obj, &mut rust_obj);
        cpp_class_fire_callback(cpp_obj);
        cpp_class_delete(cpp_obj);
    }
}
