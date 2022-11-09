use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::Token;
use syn::token::{SelfValue, And, Brace, Paren, Dot, Unsafe};
use syn::{
    FnArg,
    Receiver,
    Block,
    Stmt,
    Expr,
    ExprCall,
    ForeignItemFn,
    ExprField,
    Member,
    ItemFn,
    Path,
    PathSegment,
    Pat,
    ExprParen,
};

use crate::to_tokens_list::ToTokenList;

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

macro_rules! ident_expr {
    ($identifier:expr) => {
        {
            let mut segments = Punctuated::<PathSegment, Token![::]>::new();
            
            segments.push(PathSegment {
                ident: {$identifier},
                arguments: syn::PathArguments::None,
            });

            Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: None,
                path: Path { leading_colon: None, segments  }
            })
        }
    };
}

impl Function {
    pub fn caller(&self) -> ItemFn {
        let mut sig = self.base.sig.clone();
        let span = sig.ident.span();

        sig.abi = None;
        sig.unsafety = Some(Unsafe { span });


        let args = Punctuated::<Expr, Token![,]>::from_iter(sig.inputs.iter().filter_map(|param| {
            if let FnArg::Typed(param) = param {
                if let Pat::Ident(ident) = param.pat.as_ref() {
                    return Some(ident_expr!(ident.ident.clone()))
                }
            }
            None
        }));

        sig.inputs.insert(0, FnArg::Receiver(
            Receiver {
                attrs: Vec::new(),
                reference: Some((And { spans: [span] }, None)),
                mutability: None,
                self_token: SelfValue { span },
            }
        ));

        let statement = Stmt::Expr(
            Expr::Call(
            ExprCall {
                attrs: vec![],
                func: Box::new(
                    Expr::Paren(ExprParen {
                        attrs: vec![],
                        paren_token: Paren { span },
                        expr: Box::new(Expr::Field(
                            ExprField {
                                attrs: vec![],
                                base: Box::new(ident_expr!(Ident::new("self", span))),
                                dot_token: Dot { spans: [span] },
                                member: Member::Named(self.fnptr.clone()),
                            }
                        ))
                    }
                )),
                paren_token: Paren { span },
                args,
            }
        ));

        let block =  Box::new(Block {
            brace_token: Brace::default(),
            stmts: vec![ statement ],
        });

        syn::ItemFn {
            attrs: self.base.attrs.clone(),
            vis: self.base.vis.clone(),
            sig,
            block
        }
    }
}

impl DlBinding {
    pub fn generate_open_cl(&self) -> proc_macro::TokenStream {
        let callers: ToTokenList<ItemFn> = self.functions.clone()
            .into_iter()
            .map(|item| item.caller()).collect();

        proc_macro::TokenStream::from(quote::quote!(
            struct OpenClInner {
                library: *const (),
                // TODO: function pointers
            }

            impl Drop for OpenClInner {
                fn drop(&mut self) {
                    println!("Drop1");
                    unsafe { dlclose(self.library) }
                }
            }

            pub struct OpenClLoader {
                inner: std::sync::Arc<OpenClInner>
            }

            impl OpenClLoader {
                fn resolve() -> Result<*const (), BackendError> {
                    use std::{ fs::read_dir, ffi::CString };
            
                    let mut rd = read_dir("/usr/lib/x86_64-linux-gnu/").ok()
                        .ok_or(backend_err!("Failed to resolve OpenCL"))?;
            
                    while let Some(Ok(file)) = rd.next() {
                        match file.file_type() {
                            Ok(file_type) =>
                                if !file_type.is_file() { continue; },
                            _ => continue,
                        }
            
                        match file.file_name().to_str() {
                            Some(value) => if value.contains("OpenCL.so") {
                                let dlname = CString::new(
                                        file.path().to_str()
                                        .ok_or(backend_err!("Failed to resolve OpenCL"))?
                                    ).ok()
                                    .ok_or(backend_err!("Failed to resolve OpenCL"))?;
                                
                                let library = unsafe { dlopen(dlname.into_raw(), 1) };
                                
                                if !library.is_null() {
                                    return Ok(library)
                                }
            
                            },
                            _ => continue,
                        };
                    }
            
                    Err(backend_err!("Failed to resolve OpenCL"))
                }
            
                pub fn new() -> Result<Self, BackendError> {
                    let library = Self::resolve()?;
            
                    unsafe {
                        //TODO: load funcs let fn_cl_get_device_ids = std::mem::transmute(dlsym(library, b"clGetDeviceIDs\0" as *const u8));
                        
                        Ok(Self {
                            inner: std::sync::Arc::new(OpenClInner {
                                library,
                                // TODO: function pointers
                            })
                        })
                    }
                }

                pub fn load(&mut self) -> OpenCl {
                    OpenCl {
                        inner: self.inner.clone()
                    }
                }
            }

            pub struct OpenCl {
                inner: std::sync::Arc<OpenClInner>
            }

            impl OpenCl {
                //#callers
            }
        ))
    }
}