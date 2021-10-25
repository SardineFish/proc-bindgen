use std::{collections::HashMap, io::Write};

use syn::{Attribute, Item, ItemEnum, ItemFn, ItemStatic, ItemStruct, ItemType, ItemUnion};

#[derive(Default)]
pub struct BindgenBuilder {
    bindgen: ProcBindgen,
}

impl BindgenBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gen_struct<F: for<'a> Fn(&'a ItemStruct) -> String + 'static>(
        &mut self,
        attr: &str,
        gen_func: F,
    ) -> &mut Self {
        self.bindgen
            .struct_gens
            .insert(attr.to_owned(), Box::new(gen_func));
        self
    }

    pub fn gen_enum<F: for<'a> Fn(&'a ItemEnum) -> String + 'static>(
        &mut self,
        attr: &str,
        gen_func: F,
    ) -> &mut Self {
        self.bindgen
            .enum_gens
            .insert(attr.to_owned(), Box::new(gen_func));
        self
    }

    pub fn gen_fn<F: for<'a> Fn(&'a ItemFn) -> String + 'static>(
        &mut self,
        attr: &str,
        gen_func: F,
    ) -> &mut Self {
        self.bindgen
            .fn_gens
            .insert(attr.to_owned(), Box::new(gen_func));
        self
    }

    pub fn gen_static<F: for<'a> Fn(&'a ItemStatic) -> String + 'static>(
        &mut self,
        attr: &str,
        gen_func: F,
    ) -> &mut Self {
        self.bindgen
            .static_gens
            .insert(attr.to_owned(), Box::new(gen_func));
        self
    }

    pub fn gen_type<F: for<'a> Fn(&'a ItemType) -> String + 'static>(
        &mut self,
        attr: &str,
        gen_func: F,
    ) -> &mut Self {
        self.bindgen
            .type_gens
            .insert(attr.to_owned(), Box::new(gen_func));
        self
    }

    pub fn gen_union<F: for<'a> Fn(&'a ItemUnion) -> String + 'static>(
        &mut self,
        attr: &str,
        gen_func: F,
    ) -> &mut Self {
        self.bindgen
            .union_gens
            .insert(attr.to_owned(), Box::new(gen_func));
        self
    }
}

pub type StructGen = Box<dyn for<'a> Fn(&'a ItemStruct) -> String>;
pub type EnumGen = Box<dyn Fn(&ItemEnum) -> String>;
pub type FnGen = Box<dyn Fn(&ItemFn) -> String>;
pub type StaticGen = Box<dyn Fn(&ItemStatic) -> String>;
pub type TypeGen = Box<dyn Fn(&ItemType) -> String>;
pub type UnionGen = Box<dyn Fn(&ItemUnion) -> String>;

pub trait GeneratorEntry {
    fn config(generator: &mut BindgenBuilder);

    #[allow(clippy::new_ret_no_self)]
    fn new() -> ProcBindgen {
        let mut builder = BindgenBuilder::default();
        Self::config(&mut builder);
        builder.bindgen
    }
}

#[derive(Default)]
pub struct ProcBindgen {
    struct_gens: HashMap<String, StructGen>,
    enum_gens: HashMap<String, EnumGen>,
    fn_gens: HashMap<String, FnGen>,
    static_gens: HashMap<String, StaticGen>,
    type_gens: HashMap<String, TypeGen>,
    union_gens: HashMap<String, UnionGen>,
    inputs: Vec<String>,
}

impl ProcBindgen {
    pub fn input<S: Into<String>>(mut self, filename: S) -> Self {
        self.inputs.push(filename.into());
        self
    }
    pub fn output<S: Into<String>>(self, filename: S) {
        let mut output = std::fs::File::open(filename.into()).unwrap();
        for input in self.inputs {
            let file = syn::parse_file(&input).unwrap();
            for item in file.items {
                match &item {
                    Item::Struct(item) => {
                        call_proc_gen(&self.struct_gens, item, &item.attrs, &mut output)
                    }
                    Item::Enum(item) => {
                        call_proc_gen(&self.enum_gens, item, &item.attrs, &mut output)
                    }
                    Item::Fn(item) => call_proc_gen(&self.fn_gens, item, &item.attrs, &mut output),
                    Item::Static(item) => {
                        call_proc_gen(&self.static_gens, item, &item.attrs, &mut output)
                    }
                    Item::Type(item) => {
                        call_proc_gen(&self.type_gens, item, &item.attrs, &mut output)
                    }
                    Item::Union(item) => {
                        call_proc_gen(&self.union_gens, item, &item.attrs, &mut output)
                    }
                    _ => (),
                }
            }
        }
    }
}

fn call_proc_gen<T, S: Write>(
    map: &HashMap<String, Box<dyn Fn(&T) -> String>>,
    item: &T,
    attrs: &[Attribute],
    output_stream: &mut S,
) {
    if let Some(attr) = attrs.iter().find(|attr| attr.path.is_ident("procbind")) {
        let ident = attr.parse_args::<syn::Ident>().unwrap();
        let ident = ident.to_string();
        let generator = map
            .get(&ident)
            .unwrap_or_else(|| panic!("Procedrual generator of '{}' is not found.", ident));
        let proc_output = generator(item);
        output_stream.write_all(proc_output.as_bytes()).unwrap();
        output_stream.flush().unwrap();
    }
}
