
pub struct DlBinding {
    pub abi: syn::LitStr,
    pub functions: Vec<syn::ForeignItemFn>,
}

impl syn::parse::Parse for DlBinding {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let externs = input.parse::<syn::ItemForeignMod>()?;

        let abi = externs.clone()
            .abi
            .name
            .ok_or(syn::Error::new_spanned(externs.abi, "Please specify an ABI"))?;

        let mut functions = Vec::new();

        for item in &externs.items {
            match item {
                syn::ForeignItem::Fn(fn_item) => functions.push(fn_item.clone()),
                _ => continue,
            }
        }

        Ok(Self { abi, functions })
    }
}