use my_gen::MyProcBindgen;

fn main() {
    MyProcBindgen::create()
        .input("src/main.rs")
        .output("src/binding.hpp");

    println!("cargo:rerun-if-changed=src/main.rs");
}
