use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .file("src/cpp_class.cc")
        .flag("-std=c++11")
        .compile("foo");
}
