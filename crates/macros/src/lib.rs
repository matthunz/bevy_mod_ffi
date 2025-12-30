use proc_macro::TokenStream;
use quote::quote;

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
