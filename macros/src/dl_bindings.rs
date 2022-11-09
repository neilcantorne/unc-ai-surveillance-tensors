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