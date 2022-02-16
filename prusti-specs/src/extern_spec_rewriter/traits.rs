//! Encoding of external specs for traits
use super::common::*;
use crate::{parse_quote_spanned, specifications::common::generate_struct_name_for_trait};
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use std::collections::HashMap;
use syn::{parse_quote, spanned::Spanned};

type AssocTypesToGenericsMap<'a> = HashMap<&'a syn::Ident, syn::TypeParam>;

/// Generates a struct for a `syn::ItemTrait` which is used for checking
/// compilation of external specs on traits.
///
/// Given an extern spec for traits
/// ```rust
/// #[extern_spec]
/// trait SomeTrait {
///     type ArgTy;
///     type RetTy;
///
///     fn foo(&self, arg: Self::ArgTy) -> Self::RetTy;
/// }
/// ```
/// it produces a struct
/// ```rust
/// struct Aux<TSelf, TArgTy, TRetTy> {
///     // phantom data for TSelf, TArgTy, TRetTy
/// }
/// where TSelf: SomeTrait<ArgTy = TArgTy, RetTy = TRetTy>
/// ```
/// and a corresponding impl block with methods of `SomeTrait`.
///
pub fn rewrite_extern_spec(item_trait: &syn::ItemTrait) -> syn::Result<TokenStream> {
    let generated_struct = generate_new_struct(item_trait)?;

    let trait_impl = generated_struct.generate_impl()?;
    let new_struct = generated_struct.generated_struct;
    Ok(quote_spanned! {item_trait.span()=>
            #new_struct
            #trait_impl
    })
}

/// Responsible for generating a struct
fn generate_new_struct(item_trait: &syn::ItemTrait) -> syn::Result<GeneratedStruct> {
    let trait_ident = &item_trait.ident;

    let struct_name = generate_struct_name_for_trait(item_trait);
    let struct_ident = syn::Ident::new(&struct_name, item_trait.span());

    let mut new_struct: syn::ItemStruct = parse_quote_spanned! {item_trait.span()=>
        #[allow(non_camel_case_types)] struct #struct_ident {}
    };

    let parsed_generics = parse_trait_type_params(item_trait)?;
    // Generic type parameters are added as generics to the struct
    for parsed_generic in parsed_generics.iter() {
        if let ProvidedTypeParam::GenericType(type_param) = parsed_generic {
            new_struct
                .generics
                .params
                .push(syn::GenericParam::Type(type_param.clone()));
        }
    }

    // Find associated types in trait
    let mut assoc_types_to_generics_map = HashMap::new();
    let associated_type_decls = get_associated_types(item_trait);

    for &decl in associated_type_decls.iter() {
        if decl.default.is_some() {
            return Err(syn::Error::new(
                decl.span(),
                "Defaults for associated types in external trait specs are invalid",
            ));
        }
    }

    for associated_type_decl in associated_type_decls {
        let associated_type_ident = &associated_type_decl.ident;
        let generic_ident = syn::Ident::new(
            format!("Prusti_T_{}", associated_type_ident).as_str(),
            associated_type_ident.span(),
        );
        let type_param: syn::TypeParam =
            parse_quote_spanned! {associated_type_ident.span()=> #generic_ident };
        assoc_types_to_generics_map.insert(associated_type_ident, type_param);
    }

    // Add them as generics
    assoc_types_to_generics_map
        .values()
        .map(|param| syn::GenericParam::Type(param.clone()))
        .for_each(|generic_param| new_struct.generics.params.push(generic_param));

    // Add a new type parameter to struct which represents an implementation of the trait
    let self_type_param_ident = syn::Ident::new("Prusti_T_Self", item_trait.span());
    new_struct.generics.params.push(syn::GenericParam::Type(
        parse_quote!(#self_type_param_ident),
    ));

    // Add a where clause which restricts this self type parameter to the trait
    if item_trait.generics.where_clause.as_ref().is_some() {
        let span = item_trait.generics.where_clause.as_ref().unwrap().span();
        return Err(syn::Error::new(
            span,
            "Where clauses for extern traits specs are not supported",
        ));
    }
    let trait_assoc_type_idents = assoc_types_to_generics_map.keys();
    let trait_assoc_type_generics = assoc_types_to_generics_map.values();
    let self_where_clause: syn::WhereClause = parse_quote! {
        where #self_type_param_ident: #trait_ident <#(#parsed_generics),* #(#trait_assoc_type_idents = #trait_assoc_type_generics),*>
    };
    new_struct.generics.where_clause = Some(self_where_clause);

    add_phantom_data_for_generic_params(&mut new_struct);

    Ok(GeneratedStruct {
        generated_struct: new_struct,
        item_trait,
        assoc_types_to_generics_map,
        self_type_param_ident,
        parsed_generics,
    })
}

fn parse_trait_type_params(item_trait: &syn::ItemTrait) -> syn::Result<Vec<ProvidedTypeParam>> {
    let mut result = Vec::new();
    for generic_param in item_trait.generics.params.iter() {
        if let syn::GenericParam::Type(type_param) = generic_param {
            let parameter = ProvidedTypeParam::try_parse(type_param);
            if parameter.is_none() {
                return Err(syn::Error::new(
                    type_param.span(),
                    "Type parameters in external trait specs must be annotated with exactly one of #[generic] or #[concrete]"
                ));
            }
            result.push(parameter.unwrap());
        }
    }

    Ok(result)
}

struct GeneratedStruct<'a> {
    item_trait: &'a syn::ItemTrait,
    assoc_types_to_generics_map: AssocTypesToGenericsMap<'a>,
    self_type_param_ident: syn::Ident,
    generated_struct: syn::ItemStruct,
    parsed_generics: Vec<ProvidedTypeParam>,
}

impl<'a> GeneratedStruct<'a> {
    fn generate_impl(&self) -> syn::Result<syn::ItemImpl> {
        // Generate impl block
        let struct_ident = &self.generated_struct.ident;
        let generic_params = self
            .generated_struct
            .generics
            .params
            .clone()
            .into_token_stream();
        let where_clause = self
            .generated_struct
            .generics
            .where_clause
            .clone()
            .into_token_stream();

        let mut struct_impl: syn::ItemImpl = parse_quote_spanned! {self.item_trait.span()=>
            #[allow(non_camel_case_types)]
            impl< #generic_params > #struct_ident < #generic_params > #where_clause {}
        };

        // Add items to impl block
        for trait_item in self.item_trait.items.iter() {
            match trait_item {
                syn::TraitItem::Type(_) => {
                    // Ignore associated types, they are encoded as generics in the struct
                }
                syn::TraitItem::Method(trait_method) => {
                    if trait_method.default.is_some() {
                        return Err(syn::Error::new(
                            trait_method.default.as_ref().unwrap().span(),
                            "Default methods in external trait specs are invalid",
                        ));
                    }

                    let method = self.generate_method_stub(trait_method);
                    struct_impl.items.push(syn::ImplItem::Method(method));
                }
                _ => unimplemented!("Unimplemented trait item for extern spec"),
            };
        }

        Ok(struct_impl)
    }

    /// Generates a "stub" implementation for a trait method
    fn generate_method_stub(&self, trait_method: &syn::TraitItemMethod) -> syn::ImplItemMethod {
        let mut trait_method_sig = trait_method.sig.clone();

        // Rewrite occurrences of associated types in signature to defined generics
        syn::visit_mut::visit_signature_mut(
            &mut AssociatedTypeRewriter::new(&self.assoc_types_to_generics_map),
            &mut trait_method_sig,
        );
        let trait_method_ident = &trait_method_sig.ident;

        // Rewrite "self" to "_self" in method attributes and method inputs
        let mut trait_method_attrs = trait_method.attrs.clone();
        trait_method_attrs
            .iter_mut()
            .for_each(|attr| attr.tokens = rewrite_self(attr.tokens.clone()));
        let trait_method_inputs =
            rewrite_method_inputs(&self.self_type_param_ident, &mut trait_method_sig.inputs);

        // Create the method signature
        let trait_ident = &self.item_trait.ident;
        let parsed_generics = &self.parsed_generics;
        let self_param_ident = &self.self_type_param_ident;
        let method_path: syn::ExprPath = parse_quote_spanned! {trait_method_ident.span()=>
            <#self_param_ident as #trait_ident :: <#(#parsed_generics),*> > :: #trait_method_ident
        };

        // Create method
        return parse_quote_spanned! {trait_method.span()=>
            #[trusted]
            #[prusti::extern_spec]
            #(#trait_method_attrs)*
            #[allow(unused)]
            #trait_method_sig {
                #method_path ( #trait_method_inputs );
                unimplemented!();
            }
        };
    }
}

fn get_associated_types(item_trait: &syn::ItemTrait) -> Vec<&syn::TraitItemType> {
    let mut result = Vec::new();
    for trait_item in item_trait.items.iter() {
        if let syn::TraitItem::Type(assoc_type) = trait_item {
            result.push(assoc_type);
        }
    }
    result
}

/// Given a map from associated types to generic type params, this struct
/// rewrites associated type paths to these generic params.
///
/// # Example
/// Given a mapping `AssocType1 -> T_AssocType1, AssocType2 -> T_AssocType2`,
/// visiting a function
/// ```
/// fn foo(arg: Self::AssocType1) -> Self::AssocType2 { }
/// ```
/// results in
/// ```
/// fn foo(arg: T_AssocType1) -> T_AssocType2 { }
/// ```
///
struct AssociatedTypeRewriter<'a> {
    repl: &'a AssocTypesToGenericsMap<'a>,
}

impl<'a> AssociatedTypeRewriter<'a> {
    pub fn new(repl: &'a AssocTypesToGenericsMap<'a>) -> Self {
        AssociatedTypeRewriter { repl }
    }
}

impl<'a> syn::visit_mut::VisitMut for AssociatedTypeRewriter<'a> {
    fn visit_type_path_mut(&mut self, ty_path: &mut syn::TypePath) {
        let path = &ty_path.path;
        if path.segments.len() == 2
            && path.segments[0].ident == "Self"
            && self.repl.contains_key(&path.segments[1].ident)
        {
            let replacement = self.repl.get(&path.segments[1].ident).unwrap();
            ty_path.path = parse_quote!(#replacement);
        }

        syn::visit_mut::visit_type_path_mut(self, ty_path);
    }
}

#[derive(Debug)]
enum ProvidedTypeParam {
    /// Something non-concrete, i.e. `T`
    ConcreteType(syn::TypeParam),
    /// Something concrete, i.e. `i32`
    GenericType(syn::TypeParam),
}

impl ProvidedTypeParam {
    fn try_parse(from: &syn::TypeParam) -> Option<Self> {
        if from.attrs.len() != 1 {
            return None;
        }

        let path = &from.attrs[0].path;
        if path.segments.len() != 1 {
            return None;
        }

        // Closure for cloning and removing the attrs
        let clone_without_attrs = || {
            let mut cloned = from.clone();
            cloned.attrs.clear();
            cloned
        };

        match path.segments[0].ident.to_string().as_str() {
            "generic" => Some(ProvidedTypeParam::GenericType(clone_without_attrs())),
            "concrete" => Some(ProvidedTypeParam::ConcreteType(clone_without_attrs())),
            _ => None,
        }
    }
}

impl ToTokens for ProvidedTypeParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            ProvidedTypeParam::ConcreteType(ty_param)
            | ProvidedTypeParam::GenericType(ty_param) => ty_param.to_tokens(tokens),
        }
    }
}
