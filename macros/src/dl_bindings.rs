use proc_macro2::{Ident, TokenStream};
use syn::punctuated::Punctuated;
use syn::{Token, Attribute, AttrStyle, PathArguments, Field, Visibility, Type, TypeBareFn, Abi, LitStr, BareFnArg};
use syn::token::{SelfValue, And, Brace, Paren, Dot, Unsafe, Pound, Bracket, Colon2, Colon, Extern};
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
                    => functions.push({
                        let mut fn_item = fn_item.clone();
                        if fn_item.sig.abi.is_none() {
                            fn_item.sig.abi = Some(externs.abi.clone());
                        }
                        Function::from(fn_item)
                    }),
                _ => continue,
            }
        }

        Ok(Self { functions })
    }
}

#[derive(Clone)]
struct Function {
    base: ForeignItemFn,
    fnptr_name: Ident,
    abi: String,
}

impl From<ForeignItemFn> for Function {
    fn from(item: ForeignItemFn) -> Self {
        Self {
            fnptr_name: Ident::new(
                format!("fn_{}", item.sig.ident.to_string()).as_str(),
                item.sig.ident.span()),
            abi: item.sig.abi
                .clone()
                .expect("Please specify ABI")
                .name
                .unwrap()
                .value(),
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
                                base: Box::new(Expr::Field(
                                    ExprField {
                                        attrs: vec![],
                                        base: Box::new(ident_expr!(Ident::new("self", span))),
                                        dot_token: Dot { spans: [span] },
                                        member:Member::Named(Ident::new("inner", span)),
                                    })
                                ),
                                dot_token: Dot { spans: [span] },
                                member: Member::Named(self.fnptr_name.clone()),
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

        let mut attr = self.base.attrs.clone();
        
        attr.push(Attribute {
            pound_token: Pound { spans: [span] },
            style: AttrStyle::Outer,
            bracket_token: Bracket { span },
            path: Path { leading_colon: None, segments: {
                let mut segments = Punctuated::<PathSegment, Colon2>::new();
                segments.push(PathSegment {
                    ident: Ident::new("inline", span),
                    arguments: PathArguments::None,
                });
                segments
            }},
            tokens: TokenStream::new(),
        });

        syn::ItemFn {
            attrs: attr,
            vis: self.base.vis.clone(),
            sig,
            block
        }
    }

    pub fn fn_pointer(&self) -> Field {
        let span = self.fnptr_name.span();

        Field {
            attrs: vec![],
            vis: Visibility::Inherited,
            ident: Some(self.fnptr_name.clone()),
            colon_token: Some(Colon{ spans: [span] }),
            ty: Type::BareFn(TypeBareFn {
                lifetimes: None,
                unsafety: None,
                abi: Some(Abi{
                    extern_token: Extern { span },
                    name: Some(LitStr::new(&self.abi, span)),
                }),
                fn_token: syn::token::Fn { span },
                paren_token: Paren { span },
                inputs: self.base.sig.inputs.iter().filter_map(|arg| {
                    if let FnArg::Typed(arg) = arg {
                        return Some(
                            BareFnArg {
                                attrs: arg.attrs.clone(),
                                name: None,
                                ty: *arg.ty.clone(),
                            }
                        );
                    }
                    None
                }).collect(),
                variadic: self.base.sig.variadic.clone(),
                output: self.base.sig.output.clone(),
            }),
        }
    }
}

impl DlBinding {
    pub fn generate_open_cl(&self) -> proc_macro::TokenStream {
        let callers: ToTokenList<ItemFn> = self.functions.clone()
            .into_iter()
            .map(|item| item.caller()).collect();

        let fn_pointers: Punctuated<Field, Token![,]> = self.functions.clone()
            .into_iter()
            .map(|item| item.fn_pointer()).collect();

        proc_macro::TokenStream::from(quote::quote!(
            struct OpenClInner {
                library: *const (),
                #fn_pointers
            }

            impl Drop for OpenClInner {
                fn drop(&mut self) {
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
                #callers
            }
        ))
    }
}