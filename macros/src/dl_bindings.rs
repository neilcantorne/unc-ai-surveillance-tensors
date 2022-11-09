use proc_macro2::Ident;
use syn::ForeignItemFn;

#[derive(Clone)]
pub struct DlBinding {
    functions: Vec<Function>,
}

impl syn::parse::Parse for DlBinding {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let externs = input.parse::<syn::ItemForeignMod>()?;

        let mut functions = Vec::new();

        for item in &externs.items {
            match item {
                syn::ForeignItem::Fn(fn_item) 
                    => functions.push(Function::from(fn_item.clone())),
                _ => continue,
            }
        }

        Ok(Self { functions })
    }
}

#[derive(Clone)]
struct Function {
    base: ForeignItemFn,
    fnptr: Ident,
}

impl From<ForeignItemFn> for Function {
    fn from(item: ForeignItemFn) -> Self {
        Self {
            fnptr: Ident::new(
                format!("fn_{}", item.sig.ident.to_string()).as_str(),
                item.sig.ident.span()),
            base: item
        }
    }
}