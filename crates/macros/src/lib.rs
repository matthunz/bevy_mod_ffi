use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, Token, parse_macro_input};

#[proc_macro_attribute]
pub fn main(input: TokenStream, attrs: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let attrs = proc_macro2::TokenStream::from(attrs);

    quote! {
        #attrs
        #input

        #[unsafe(no_mangle)]
        extern "C" fn bevy_main(world_ptr: *mut bevy_mod_ffi::bevy_mod_ffi_core::world) {
            let mut world = unsafe { bevy_mod_ffi::world::World::from_ptr(world_ptr) };
            main(&mut world)
        }
    }
    .into()
}

#[proc_macro_derive(SharedComponent, attributes(component))]
pub fn derive_shared_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut storage_type = quote! { bevy_mod_ffi::component::StorageType::Table };
    let mut mutability = quote! { bevy_mod_ffi::component::Mutable };
    let mut on_add = quote! { None };
    let mut on_insert = quote! { None };
    let mut on_replace = quote! { None };
    let mut on_remove = quote! { None };
    let mut on_despawn = quote! { None };

    for attr in &input.attrs {
        if !attr.path().is_ident("component") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("storage") {
                let _: Token![=] = meta.input.parse()?;
                let value: syn::LitStr = meta.input.parse()?;
                match value.value().as_str() {
                    "SparseSet" => {
                        storage_type = quote! { bevy_mod_ffi::component::StorageType::SparseSet };
                    }
                    "Table" => {
                        storage_type = quote! { bevy_mod_ffi::component::StorageType::Table };
                    }
                    _ => {}
                }
            } else if meta.path.is_ident("immutable") {
                mutability = quote! { bevy_mod_ffi::component::Immutable };
            } else if meta.path.is_ident("on_add") {
                let _: Token![=] = meta.input.parse()?;
                let func: Expr = meta.input.parse()?;
                on_add = quote! { Some(#func) };
            } else if meta.path.is_ident("on_insert") {
                let _: Token![=] = meta.input.parse()?;
                let func: Expr = meta.input.parse()?;
                on_insert = quote! { Some(#func) };
            } else if meta.path.is_ident("on_replace") {
                let _: Token![=] = meta.input.parse()?;
                let func: Expr = meta.input.parse()?;
                on_replace = quote! { Some(#func) };
            } else if meta.path.is_ident("on_remove") {
                let _: Token![=] = meta.input.parse()?;
                let func: Expr = meta.input.parse()?;
                on_remove = quote! { Some(#func) };
            } else if meta.path.is_ident("on_despawn") {
                let _: Token![=] = meta.input.parse()?;
                let func: Expr = meta.input.parse()?;
                on_despawn = quote! { Some(#func) };
            }
            Ok(())
        });
    }

    let expanded = quote! {
        impl bevy_mod_ffi::component::SharedComponent for #name {
            type Mutability = #mutability;

            const STORAGE_TYPE: bevy_mod_ffi::component::StorageType = #storage_type;

            fn on_add() -> Option<for<'w> fn(bevy_mod_ffi::world::DeferredWorld<'w>, bevy_mod_ffi::component::HookContext)> {
                #on_add
            }

            fn on_insert() -> Option<for<'w> fn(bevy_mod_ffi::world::DeferredWorld<'w>, bevy_mod_ffi::component::HookContext)> {
                #on_insert
            }

            fn on_replace() -> Option<for<'w> fn(bevy_mod_ffi::world::DeferredWorld<'w>, bevy_mod_ffi::component::HookContext)> {
                #on_replace
            }

            fn on_remove() -> Option<for<'w> fn(bevy_mod_ffi::world::DeferredWorld<'w>, bevy_mod_ffi::component::HookContext)> {
                #on_remove
            }

            fn on_despawn() -> Option<for<'w> fn(bevy_mod_ffi::world::DeferredWorld<'w>, bevy_mod_ffi::component::HookContext)> {
                #on_despawn
            }
        }
    };

    TokenStream::from(expanded)
}
