extern crate cc;

fn main() {
  cc::Build::new()
            .cpp(true)
            .include("cc")
            .file("cc/integer_rust.cc")
            .cpp_link_stdlib("stdc++") // TODO
            .warnings_into_errors(true)
            .compile("integer");
}
