use proc_bindgen::{proc_code, BindgenBuilder, ProcBindgen};
use syn::{ItemStruct, __private::quote::format_ident};

pub struct MyProcBindgen;

impl MyProcBindgen {
    pub fn create() -> ProcBindgen {
        let mut builder = BindgenBuilder::new();

        builder.gen_struct("my_bindgen", gen_struct);

        builder.build()
    }
}

pub fn gen_struct(item: &ItemStruct) -> String {
    let name = item.ident.clone();
    let extern_fn_name = format_ident!("do_with_{}", name);

    proc_code!(

        void #extern_fn_name(App* app, #name obj);

        template<>
        void App::do_with<#name>(#name obj) {
            #extern_fn_name(this, event);
        }
    )
}
