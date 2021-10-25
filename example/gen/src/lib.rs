use proc_bindgen::{proc_code, BindgenBuilder, GeneratorEntry, ProcBindgen};
use syn::{ItemStruct, __private::quote::format_ident};

pub struct MyProcBindgen;

impl MyProcBindgen {
    pub fn new() -> ProcBindgen {
        let mut builder = BindgenBuilder::new();

        builder.gen_struct("my_bindgen", gen_struct);

        builder.build()
    }
}

pub fn gen_struct(item: &ItemStruct) -> String {
    let name = item.ident.clone();
    let extern_fn_name = format_ident!("__app_push_event_{}", name);

    proc_code!(
        template<>
        void App::push_event<#name>(#name event) {
            #extern_fn_name(this, event);
        }
    )
}
