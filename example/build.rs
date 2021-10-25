use gen::MyProcBindgen;

fn main() {
    MyProcBindgen::new()
        .input("src/main.rs")
        .output("src/binding.hpp");

    println!("cargo:rerun-if-changed=src/main.rs");
}
