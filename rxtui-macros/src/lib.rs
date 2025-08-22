use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, parse_macro_input};

/// Derive macro that implements the Component trait
///
/// This macro automatically implements all the boilerplate methods
/// required by the Component trait.
///
/// By default, it looks for a field named `id`. You can specify a different
/// field using the `#[component_id]` attribute.
///
/// # Example
///
/// ```ignore
/// #[derive(Component, Clone)]
/// struct MyComponent {
///     id: Option<ComponentId>,  // Default field name
///     // other fields...
/// }
///
/// // Or with a custom field name:
/// #[derive(Component, Clone)]
/// struct MyComponent {
///     #[component_id]
///     key: Option<ComponentId>,
///     // other fields...
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
#[proc_macro_derive(Component, attributes(component_id))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Verify it's a struct
    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Component can only be derived for structs"),
    };

    // Find the ID field - either marked with #[component_id] or named "id"
    let id_field =
        match fields {
            Fields::Named(fields) => {
                // First, look for a field with #[component_id] attribute
                let marked_field = fields.named.iter().find(|f| {
                    f.attrs
                        .iter()
                        .any(|attr| attr.path().is_ident("component_id"))
                });

                if let Some(field) = marked_field {
                    field
                        .ident
                        .as_ref()
                        .expect("Named field should have an identifier")
                } else {
                    // Otherwise, look for a field named "id"
                    fields.named.iter()
                    .find(|f| f.ident.as_ref().is_some_and(|ident| ident == "id"))
                    .and_then(|f| f.ident.as_ref())
                    .expect(
                        "Component derive requires either a field named `id: Option<ComponentId>` \
                         or a field marked with `#[component_id]` attribute"
                    )
                }
            }
            _ => panic!("Component can only be derived for structs with named fields"),
        };

    // Generate the implementation
    let expanded = quote! {
        impl rxtui::Component for #name {
            fn get_id(&self) -> Option<rxtui::ComponentId> {
                self.#id_field.clone()
            }

            fn set_id(&mut self, id: rxtui::ComponentId) {
                self.#id_field = Some(id);
            }

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
