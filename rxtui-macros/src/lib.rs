use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derive macro that implements the Component trait
///
/// This macro automatically implements all the boilerplate methods
/// required by the Component trait.
///
/// # Example
///
/// ```ignore
/// #[derive(Component, Clone)]
/// struct MyComponent {
///     // any fields you need
/// }
///
/// impl MyComponent {
///     fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
///         // your implementation
///     }
///
///     fn view(&self, ctx: &Context) -> Node {
///         // your implementation
///     }
/// }
/// ```
#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Generate the implementation - no ID field needed
    let expanded = quote! {
        impl rxtui::Component for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn clone_box(&self) -> Box<dyn rxtui::Component> {
                Box::new(self.clone())
            }

            // Forward to the user's implementations
            fn update(&self, ctx: &rxtui::Context, msg: Box<dyn rxtui::Message>, topic: Option<&str>) -> rxtui::Action {
                <#name>::update(self, ctx, msg, topic)
            }

            fn view(&self, ctx: &rxtui::Context) -> rxtui::Node {
                <#name>::view(self, ctx)
            }
        }
    };

    TokenStream::from(expanded)
}
