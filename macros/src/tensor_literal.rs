use quote::TokenStreamExt;
use syn::parse::ParseStream;
use syn::{Lit, Token};
use proc_macro2::{Span, Ident, Group, Punct, Literal, Delimiter};

pub struct TensorLiteral {
    elements: Vec<Vec<Literal>>,
    element_type: String,
}

impl quote::ToTokens for TensorLiteral {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let row = self.elements.len();
        let column = self.elements.first().unwrap().len();

        // Tensor type identifier
        tokens.append(Ident::new_raw("Tensor", Span::call_site()));

        // Turbofish operator
        tokens.append(Punct::new(':', proc_macro2::Spacing::Joint));
        tokens.append(Punct::new(':', proc_macro2::Spacing::Joint));
        tokens.append(Punct::new('<', proc_macro2::Spacing::Alone));

        // Generic parameters
        tokens.append(Ident::new_raw(&self.element_type, Span::call_site()));
        tokens.append(Punct::new(',', proc_macro2::Spacing::Alone));
        tokens.append(Literal::usize_suffixed(row));
        tokens.append(Punct::new(',', proc_macro2::Spacing::Alone));
        tokens.append(Literal::usize_suffixed(column));
        tokens.append(Punct::new('>', proc_macro2::Spacing::Alone));

        // Scope operator
        tokens.append(Punct::new(':', proc_macro2::Spacing::Joint));
        tokens.append(Punct::new(':', proc_macro2::Spacing::Joint));

        // Call from array
        tokens.append(Ident::new_raw("from", Span::call_site()));
        
        // 2-D array contruction
        tokens.append(Group::new(Delimiter::Parenthesis, 
        Group::new(Delimiter::Bracket, 
            {
                let mut stream = quote::__private::TokenStream::new();
                for vector in &self.elements {
                    stream.append(Group::new(Delimiter::Bracket, {
                        let mut stream = quote::__private::TokenStream::new();

                        for element in vector {
                            stream.append(element.to_owned());
                            stream.append(Punct::new(',', proc_macro2::Spacing::Alone));
                        }

                        stream
                    }));

                    stream.append(Punct::new(',', proc_macro2::Spacing::Alone));
                }

                stream
            }
        ).to_token_stream()));
    }
}

impl syn::parse::Parse for TensorLiteral {
    fn parse(input: ParseStream) -> syn::Result<Self>  {
        let mut elements = Vec::<Vec<Literal>>::new();
        let mut element_type = None;

        elements.push(Vec::new());

        while !input.is_empty() {
            let literal = {
                let lit = input.parse::<Lit>()?;
                match lit {
                    Lit::Int(int_lit) => {
                        if element_type.is_none()
                            && !int_lit.suffix().is_empty() {
                            element_type = Some(int_lit.suffix().to_string());
                        }

                        if let Some(el_type) = element_type.as_ref() {
                            
                            if int_lit.suffix().is_empty() &&
                                el_type.chars().next().unwrap() == 'f' {
                                return Err(syn::Error::new_spanned(int_lit, 
                                        "Expects float literal"));
                            }
                            else if int_lit.suffix() != el_type {
                                return Err(syn::Error::new_spanned(int_lit, 
                                    format!("Expects {} literal", el_type)));
                            }
                        }

                        int_lit.token()
                    },
                    Lit::Float(float_lit) => {
                        if element_type.is_none()
                            && !float_lit.suffix().is_empty() {
                            element_type = Some(float_lit.suffix().to_string());
                        }

                        if let Some(el_type) = element_type.as_ref() {
                            
                            if float_lit.suffix().is_empty() &&
                                el_type.chars().next().unwrap() != 'f' {
                                return Err(syn::Error::new_spanned(float_lit, 
                                        "Expects int literal"));
                            }
                            else if float_lit.suffix() != el_type {
                                return Err(syn::Error::new_spanned(float_lit, 
                                    format!("Expects {} literal", el_type)));
                            }
                        }

                        float_lit.token()
                    },
                    _ => return Err(syn::Error::new_spanned(lit, 
                        "Expect float or integer types")),
                }
            };

            elements.last_mut().unwrap().push(literal);

            if input.parse::<Token!(,)>().is_ok() {}
            else if input.parse::<Token!(|)>().is_ok() {
                elements.push(Vec::new());
            }
        }

        Ok(Self { elements, element_type: element_type.unwrap() })
    }
}