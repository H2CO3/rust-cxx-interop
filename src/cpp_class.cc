class RustStruct;

class CppClass {
public:
    RustStruct *arg;
    void (*fn)(RustStruct *);

    CppClass(RustStruct *arg_, void (*fn_)(RustStruct *)) :
        arg(arg_),
        fn(fn_)
    {}
};

extern "C" {
      CppClass *cpp_class_new() {
          return new CppClass(nullptr, nullptr);
      }

      void cpp_class_delete(CppClass *obj) {
          delete obj;
      }

      void cpp_class_set_callback_fn(CppClass *obj, void (*cb)(RustStruct *)) {
          obj->fn = cb;
      }

      void cpp_class_set_callback_arg(CppClass *obj, RustStruct *arg) {
          obj->arg = arg;
      }

      void cpp_class_fire_callback(CppClass *obj) {
          (obj->fn)(obj->arg);
      }
}
