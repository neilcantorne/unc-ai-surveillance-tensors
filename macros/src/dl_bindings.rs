use std::str::FromStr;

use proc_macro2::{ Ident, TokenStream };
use syn::punctuated::Punctuated;
use syn::token::{
    SelfValue,
    And,
    Brace,
    Paren,
    Dot,
    Unsafe,
    Pound,
    Bracket,
    Colon2,
    Colon,
    Extern,
    Let,
    Semi,
    As,
    Const,
    Star,
    If,
    Bang
};
use syn::{
    Token,
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
    Attribute,
    AttrStyle,
    PathArguments,
    Field,
    Visibility,
    Type,
    TypeBareFn,
    Abi,
    LitStr,
    BareFnArg,
    FieldValue,
    ExprUnsafe,
    ExprLet,
    PatIdent,
    ExprLit,
    Lit,
    LitByteStr,
    ExprCast,
    TypePtr,
    TypePath,
    ExprIf,
    ExprMacro,
    MacroDelimiter
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
    symbol: String,
    symbol_bytes: Vec<u8>
}

impl From<ForeignItemFn> for Function {
    fn from(mut item: ForeignItemFn) -> Self {
        let mut symbol = item.sig.ident.to_string();
        let mut remove_index = None;

        for (index, attribute) in item.attrs.iter().enumerate() {
            if let Some(ident) = attribute.path.get_ident() {
                if ident.to_string() == "symbol" {
                    symbol = attribute
                        .tokens
                        .to_string();

                    if symbol.is_empty() {
                        panic!("Invalid symbol")
                    }

                    symbol.pop();
                    symbol.remove(0);

                    remove_index = Some(index)
                }
            }
        }

        if let Some(index) = remove_index {
            item.attrs.remove(index);
        }
        
        Self {
            symbol: symbol.clone(),
            symbol_bytes: {
                let mut symbol = symbol.clone();
                symbol.push('\0');

                symbol.as_bytes().to_vec()
            },
            fnptr_name: Ident::new(
                format!("fn_{}", item.sig.ident.to_string()).as_str(),
                item.sig.ident.span()),
            abi: item.sig.abi
                .clone()
                .expect("Please specify ABI")
                .name
                .unwrap()
                .value(),
            base: item,
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

macro_rules! path_expr {
    ($span:expr, $($segment:ident)::*) => {
        Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: Path {
                leading_colon: None,
                segments: {
                    let mut segments = Punctuated::<PathSegment, Token![::]>::new();
        
                    $(
                        segments.push(PathSegment {
                            ident: Ident::new(stringify!($segment), {$span}),
                            arguments: syn::PathArguments::None,
                        });
                    )*

                    segments
                }
            }
        })
    };
}

macro_rules! define_path {
    ($span:expr, $($segment:ident)::*) => {
        Path {
            leading_colon: None,
            segments: {
                let mut segments = Punctuated::<PathSegment, Token![::]>::new();
    
                $(
                    segments.push(PathSegment {
                        ident: Ident::new(stringify!($segment), {$span}),
                        arguments: syn::PathArguments::None,
                    });
                )*

                segments
            }
        }
    }
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

    pub fn initializer(&self) -> FieldValue {
        let span = self.fnptr_name.span();

        FieldValue {
            attrs: vec![],
            member: Member::Named(self.fnptr_name.clone()),
            colon_token: Some(Colon { spans: [self.fnptr_name.span()]}),
            expr: Expr::Unsafe(ExprUnsafe {
                attrs: vec![],
                unsafe_token: Unsafe { span },
                block: Block {
                    brace_token: Brace { span },
                    stmts: vec![Stmt::Semi({
                        Expr::Let(ExprLet{
                            attrs: vec![],
                            let_token: Let { span },
                            pat: Pat::Ident(PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: self.fnptr_name.clone(),
                                subpat: None,
                            }),
                            eq_token: syn::token::Eq { spans: [span] },
                            expr: Box::new(
                                Expr::Call(ExprCall {
                                    attrs: vec![],
                                    func: Box::new(ident_expr!(Ident::new("dlsym", span))),
                                    paren_token: Paren { span },
                                    args: {
                                        let mut args = Punctuated::<Expr, Token![,]>::new();
                                        
                                        args.push(ident_expr!(Ident::new("library", span)));
                                        args.push(Expr::Cast(ExprCast{
                                            attrs: vec![],
                                            expr: Box::new(Expr::Lit(ExprLit {
                                                attrs: vec![],
                                                lit: Lit::ByteStr(LitByteStr::new(&self.symbol_bytes, span)),
                                            })),
                                            as_token: As { span },
                                            ty: Box::new(Type::Ptr(TypePtr{
                                                star_token: Star { spans: [span] },
                                                const_token: Some(Const { span }),
                                                mutability: None,
                                                elem: Box::new(Type::Path(TypePath {
                                                    qself: None,
                                                    path: Path {
                                                        leading_colon: None,
                                                        segments: {
                                                            let mut segments = Punctuated::<PathSegment, Colon2>::new();
                                                            segments.push(PathSegment {
                                                                ident: Ident::new("u8", span),
                                                                arguments: PathArguments::None,
                                                            });
                                                            segments
                                                        },
                                                    },
                                                })),
                                            })),
                                        }));
                                        args
                                    },
                                })
                            ),
                        })
                    }, Semi { spans: [span] }),
                    Stmt::Expr(Expr::If(ExprIf {
                        attrs: vec![],
                        if_token: If { span },
                        cond: Box::new(Expr::Call(ExprCall {
                            attrs: vec![],
                            func: Box::new(Expr::Field(ExprField {
                                attrs: vec![],
                                base: Box::new(ident_expr!(self.fnptr_name.clone())),
                                dot_token: Dot { spans: [span] },
                                member: Member::Named(Ident::new("is_null", span)),
                            })),
                            paren_token: Paren { span },
                            args: Punctuated::<Expr, Token![,]>::new(),
                        })),
                        then_branch: Block {
                            brace_token: Brace { span },
                            stmts: vec![
                                Stmt::Semi(Expr::Macro(ExprMacro {
                                    attrs: vec![],
                                    mac: syn::Macro {
                                        path: define_path!(span, failed_load),
                                        bang_token: Bang { spans: [span] },
                                        delimiter: MacroDelimiter::Paren(Paren { span }),
                                        tokens: TokenStream::from_str(&self.symbol).unwrap(),
                                    },
                                }), Semi { spans: [span] })
                            ]
                        },
                        else_branch: None,
                    })),
                    Stmt::Expr(Expr::Call(ExprCall {
                        attrs: vec![],
                        func: Box::new(path_expr!(span, std::mem::transmute)),
                        paren_token: Paren { span },
                        args: {
                            let mut args = Punctuated::<Expr, Token![,]>::new();

                            args.push(ident_expr!(self.fnptr_name.clone()));

                            args
                        },
                    }))]
                },
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

        let intializers: Punctuated<FieldValue, Token![,]> = self.functions.clone()
            .into_iter()
            .map(|item| item.initializer()).collect();

        proc_macro::TokenStream::from(quote::quote!(
            struct OpenClInner {
                library: usize,
                #fn_pointers
            }

            impl Drop for OpenClInner {
                fn drop(&mut self) {
                    unsafe { dlclose(self.library) }
                }
            }

            pub struct OpenClFactory {
                inner: std::sync::Arc<OpenClInner>
            }

            impl OpenClFactory {
                fn resolve() -> Result<usize, BackendError> {
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
                                
                                if !library == 0usize {
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
            
                    Ok(Self {
                        inner: std::sync::Arc::new(OpenClInner {
                            library,
                            #intializers
                        })
                    })
                }

                pub fn create(&mut self) -> OpenCl {
                    OpenCl {
                        inner: self.inner.clone()
                    }
                }
            }

            #[derive(Clone)]
            pub struct OpenCl {
                inner: std::sync::Arc<OpenClInner>
            }

            impl OpenCl {
                #callers
            }
        ))
    }
}