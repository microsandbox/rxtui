use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{
    DeriveInput, Expr, FnArg, Ident, ImplItem, ItemFn, ItemImpl, LitStr, Pat, PatType, Token, Type,
    parse_macro_input,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents a topic mapping like "timer" => TimerMsg or self.topic => TimerMsg
enum TopicKey {
    Static(LitStr),
    Dynamic(Expr),
}

struct TopicMapping {
    key: TopicKey,
    _arrow: Token![=>],
    msg_type: Type,
}

/// Parse the update attribute arguments with new syntax
struct UpdateArgs {
    msg_type: Option<Type>,
    topics: Vec<TopicMapping>,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Parse for TopicMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Try to parse as a string literal first
        let key = if input.peek(LitStr) {
            TopicKey::Static(input.parse()?)
        } else {
            // Otherwise parse as an expression (e.g., self.topic_name)
            TopicKey::Dynamic(input.parse()?)
        };

        Ok(TopicMapping {
            key,
            _arrow: input.parse()?,
            msg_type: input.parse()?,
        })
    }
}

impl Parse for UpdateArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut msg_type = None;
        let mut topics = Vec::new();

        while !input.is_empty() {
            // Parse identifier (msg or topics)
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if ident == "msg" {
                msg_type = Some(input.parse()?);
            } else if ident == "topics" {
                // Parse array of topic mappings
                let content;
                syn::bracketed!(content in input);

                while !content.is_empty() {
                    topics.push(content.parse::<TopicMapping>()?);

                    if !content.is_empty() {
                        content.parse::<Token![,]>()?;
                    }
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(UpdateArgs { msg_type, topics })
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Extract parameter name and type from a function argument
fn extract_param_info(arg: &FnArg) -> Option<(Ident, Type)> {
    if let FnArg::Typed(PatType { pat, ty, .. }) = arg
        && let Pat::Ident(pat_ident) = &**pat
    {
        let name = pat_ident.ident.clone();
        let ty = (**ty).clone();
        return Some((name, ty));
    }
    None
}

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

            // Forward effects method when feature is enabled
            #[cfg(feature = "effects")]
            fn effects(&self, ctx: &rxtui::Context) -> Vec<rxtui::effect::Effect> {
                <#name>::effects(self, ctx)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Simplifies component update methods by automatically handling message downcasting,
/// state fetching, and topic routing.
///
/// # Basic usage
///
/// The simplest form just handles a single message type:
///
/// ```ignore
/// #[update]
/// fn update(&self, ctx: &Context, msg: CounterMsg) -> Action {
///     match msg {
///         CounterMsg::Exit => Action::Exit,
///         _ => Action::None,
///     }
/// }
/// ```
///
/// # With state management
///
/// Add a state parameter and it will be automatically fetched and passed in:
///
/// ```ignore
/// #[update]
/// fn update(&self, ctx: &Context, msg: CounterMsg, mut state: CounterState) -> Action {
///     match msg {
///         CounterMsg::Increment => {
///             state.count += 1;
///             Action::Update(Box::new(state))
///         }
///         CounterMsg::Exit => Action::Exit,
///     }
/// }
/// ```
///
/// # With topic-based messaging
///
/// Components can also listen to topic messages. Topics can be static strings or
/// dynamic expressions from self:
///
/// ```ignore
/// #[update(msg = AppMsg, topics = ["timer" => TimerMsg, self.topic_name => UpdateMsg])]
/// fn update(&self, ctx: &Context, messages: Messages, mut state: AppState) -> Action {
///     match messages {
///         Messages::AppMsg(msg) => { /* handle regular message */ }
///         Messages::TimerMsg(msg) => { /* handle timer topic */ }
///         Messages::UpdateMsg(msg) => { /* handle dynamic topic */ }
///     }
/// }
/// ```
///
/// # How it works
///
/// The macro transforms your simplified function into the full Component trait implementation:
///
/// ```text
/// ┌─────────────────────────────────────────────────────────────────┐
/// │ #[update(msg = CounterMsg, topics = [self.topic => ResetMsg])]  │
/// │ fn update(&self, ctx: &Context, msg: Messages,                  │
/// │           mut state: CounterState) -> Action {                  │
/// │     match msg {                                                 │
/// │         Messages::CounterMsg(m) => { ... }                      │
/// │         Messages::ResetMsg(m) => { ... }                        │
/// │     }                                                           │
/// │ }                                                               │
/// └─────────────────────────────────────────────────────────────────┘
///                                 ↓
/// ┌─────────────────────────────────────────────────────────────────┐
/// │ fn update(&self, ctx: &Context,                                 │
/// │           msg: Box<dyn Message>,                                │
/// │           topic: Option<&str>) -> Action {                      │
/// │                                                                 │
/// │     enum Messages { /* generated */ }                           │
/// │     let mut state = ctx.get_state::<CounterState>();            │
/// │                                                                 │
/// │     if let Some(topic) = topic {                                │
/// │         if topic == &*(self.topic) {                            │
/// │             if let Some(m) = msg.downcast::<ResetMsg>() {       │
/// │                 let msg = Messages::ResetMsg(m.clone());        │
/// │                 return { /* user's match block */ };            │
/// │             }                                                   │
/// │         }                                                       │
/// │         return Action::None;                                    │
/// │     }                                                           │
/// │                                                                 │
/// │     if let Some(m) = msg.downcast::<CounterMsg>() {             │
/// │         let msg = Messages::CounterMsg(m.clone());              │
/// │         return { /* user's match block */ };                    │
/// │     }                                                           │
/// │                                                                 │
/// │     Action::None                                                │
/// │ }                                                               │
/// └─────────────────────────────────────────────────────────────────┘
/// ```
///
/// # Parameters
///
/// The function parameters are detected by position:
/// - `&self` (required)
/// - `&Context` (required) - any name allowed
/// - Message type (required) - any name allowed
/// - State type (optional) - any name allowed
#[proc_macro_attribute]
pub fn update(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;

    // Parse function parameters by position
    let mut params = input_fn.sig.inputs.iter();

    // Position 0: &self (skip it)
    params
        .next()
        .expect("#[update] function must have &self as first parameter");

    // Position 1: &Context
    let ctx_param = params
        .next()
        .expect("#[update] function must have &Context as second parameter");
    let (ctx_name, _ctx_type) =
        extract_param_info(ctx_param).expect("Failed to extract context parameter info");

    // Position 2: Message type
    let msg_param = params
        .next()
        .expect("#[update] function must have message type as third parameter");
    let (msg_name, msg_type) =
        extract_param_info(msg_param).expect("Failed to extract message parameter info");

    // Position 3: State type (optional)
    let state_info = params.next().and_then(extract_param_info);

    // Check if we have topic arguments
    if args.is_empty() {
        // Simple case: no topics specified
        // Generate state fetching code if state parameter exists
        let state_setup = if let Some((state_name, state_type)) = &state_info {
            quote! { let mut #state_name = #ctx_name.get_state::<#state_type>(); }
        } else {
            quote! {}
        };

        let expanded = quote! {
            #fn_vis fn #fn_name(&self, #ctx_name: &rxtui::Context, msg: Box<dyn rxtui::Message>, _topic: Option<&str>) -> rxtui::Action {
                if let Some(#msg_name) = msg.downcast::<#msg_type>() {
                    #state_setup
                    let #msg_name = #msg_name.clone();
                    return #fn_block;
                }

                rxtui::Action::None
            }
        };

        TokenStream::from(expanded)
    } else {
        // Complex case: with topics
        let args = parse_macro_input!(args as UpdateArgs);

        // Use provided msg type or fall back to first positional arg
        let regular_type = args.msg_type.unwrap_or(msg_type.clone());

        // Generate enum name from the message parameter type
        let enum_name = &msg_type;

        // Generate enum variants
        let mut enum_variants = vec![];
        let regular_variant =
            format_ident!("{}", quote!(#regular_type).to_string().replace("::", "_"));
        enum_variants.push(quote! { #regular_variant(#regular_type) });

        // Generate topic handling code
        let mut topic_matches = vec![];
        for topic in &args.topics {
            let topic_type = &topic.msg_type;
            let variant_name =
                format_ident!("{}", quote!(#topic_type).to_string().replace("::", "_"));

            enum_variants.push(quote! { #variant_name(#topic_type) });

            let topic_check = match &topic.key {
                TopicKey::Static(lit_str) => {
                    quote! { topic == #lit_str }
                }
                TopicKey::Dynamic(expr) => {
                    // Use &* to convert String to &str
                    quote! { topic == &*(#expr) }
                }
            };

            topic_matches.push(quote! {
                if #topic_check {
                    if let Some(msg) = msg.downcast::<#topic_type>() {
                        let #msg_name = #enum_name::#variant_name(msg.clone());
                        return #fn_block;
                    }
                }
            });
        }

        // Generate state setup
        let state_setup = if let Some((state_name, state_type)) = &state_info {
            quote! { let mut #state_name = #ctx_name.get_state::<#state_type>(); }
        } else {
            quote! {}
        };

        // Generate the complete function
        let expanded = quote! {
            #fn_vis fn #fn_name(&self, #ctx_name: &rxtui::Context, msg: Box<dyn rxtui::Message>, topic: Option<&str>) -> rxtui::Action {
                // Generate the enum for message types
                #[allow(non_camel_case_types)]
                enum #enum_name {
                    #(#enum_variants),*
                }

                #state_setup

                // Handle topic messages first
                if let Some(topic) = topic {
                    #(#topic_matches)*
                    return rxtui::Action::None;
                }

                // Handle regular message
                if let Some(msg) = msg.downcast::<#regular_type>() {
                    let #msg_name = #enum_name::#regular_variant(msg.clone());
                    return #fn_block;
                }

                rxtui::Action::None
            }
        };

        TokenStream::from(expanded)
    }
}

/// Simplifies component view methods by automatically fetching state from the context.
///
/// # With state
///
/// If you include a state parameter, it will be automatically fetched:
///
/// ```ignore
/// #[view]
/// fn view(&self, ctx: &Context, state: CounterState) -> Node {
///     node! {
///         div [
///             text(format!("Count: {}", state.count))
///         ]
///     }
/// }
/// ```
///
/// # Without state
///
/// For stateless components, just omit the state parameter:
///
/// ```ignore
/// #[view]
/// fn view(&self, ctx: &Context) -> Node {
///     node! {
///         div [
///             text("Static content")
///         ]
///     }
/// }
/// ```
///
/// The macro automatically detects whether a state parameter is present and generates
/// the appropriate code to fetch it from the context.
///
/// # Parameters
///
/// The function parameters are detected by position:
/// - `&self` (required)
/// - `&Context` (required) - any name allowed
/// - State type (optional) - any name allowed
#[proc_macro_attribute]
pub fn view(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;

    // Parse function parameters by position
    let mut params = input_fn.sig.inputs.iter();

    // Position 0: &self (skip it)
    params
        .next()
        .expect("#[view] function must have &self as first parameter");

    // Position 1: &Context
    let ctx_param = params
        .next()
        .expect("#[view] function must have &Context as second parameter");
    let (ctx_name, _ctx_type) =
        extract_param_info(ctx_param).expect("Failed to extract context parameter info");

    // Position 2: State type (optional)
    if let Some(state_param) = params.next() {
        let (state_name, state_type) =
            extract_param_info(state_param).expect("Failed to extract state parameter info");

        // Generate with state fetching
        let expanded = quote! {
            #fn_vis fn #fn_name(&self, #ctx_name: &rxtui::Context) -> rxtui::Node {
                let #state_name = #ctx_name.get_state::<#state_type>();
                #fn_block
            }
        };

        TokenStream::from(expanded)
    } else {
        // No state parameter - just forward as-is
        let expanded = quote! {
            #fn_vis fn #fn_name(&self, #ctx_name: &rxtui::Context) -> rxtui::Node {
                #fn_block
            }
        };

        TokenStream::from(expanded)
    }
}

/// Marks an async method as a single effect that runs in the background.
///
/// # Basic usage
///
/// Define an async effect that runs in the background:
///
/// ```ignore
/// #[effect]
/// async fn timer_effect(&self, ctx: &Context) {
///     loop {
///         tokio::time::sleep(Duration::from_secs(1)).await;
///         ctx.send(Msg::Tick);
///     }
/// }
/// ```
///
/// # With state
///
/// Effects can access component state:
///
/// ```ignore
/// #[effect]
/// async fn fetch_data(&self, ctx: &Context, state: MyState) {
///     let url = &state.api_url;
///     let data = fetch(url).await;
///     ctx.send(Msg::DataLoaded(data));
/// }
/// ```
///
/// # Multiple effects
///
/// You can define multiple effects on a component - they will all be collected
/// into a single `effects()` method:
///
/// ```ignore
/// impl MyComponent {
///     #[effect]
///     async fn timer(&self, ctx: &Context) {
///         // Timer logic
///     }
///
///     #[effect]
///     async fn websocket(&self, ctx: &Context) {
///         // WebSocket logic
///     }
/// }
/// ```
///
/// # Parameters
///
/// The function parameters are detected by position:
/// - `&self` (required)
/// - `&Context` (required) - any name allowed
/// - State type (optional) - any name allowed
///
/// Note: Use the #[component] macro on the impl block to automatically collect
/// all methods marked with #[effect] into the effects() method.
#[proc_macro_attribute]
pub fn effect(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;

    // Parse function parameters by position
    let mut params = input_fn.sig.inputs.iter();

    // Position 0: &self (skip it)
    params
        .next()
        .expect("#[effects] function must have &self as first parameter");

    // Position 1: &Context
    let ctx_param = params
        .next()
        .expect("#[effects] function must have &Context as second parameter");
    let (ctx_name, _ctx_type) =
        extract_param_info(ctx_param).expect("Failed to extract context parameter info");

    // Position 2: State type (optional)
    let state_setup = if let Some(state_param) = params.next() {
        let (state_name, state_type) =
            extract_param_info(state_param).expect("Failed to extract state parameter info");
        quote! { let #state_name = #ctx_name.get_state::<#state_type>(); }
    } else {
        quote! {}
    };

    // Generate a helper method that creates the effect
    let helper_name = format_ident!("__{}_effect", fn_name);

    let expanded = quote! {
        #[allow(dead_code)]
        #fn_vis fn #helper_name(&self, #ctx_name: &rxtui::Context) -> rxtui::effect::Effect {
            Box::pin({
                let #ctx_name = #ctx_name.clone();
                #state_setup
                async move #fn_block
            })
        }

        // Keep the original async function for reference/testing if needed
        #[allow(dead_code)]
        #fn_vis async fn #fn_name(&self, #ctx_name: &rxtui::Context) #fn_block
    };

    TokenStream::from(expanded)
}

/// Impl-level macro that automatically handles Component trait boilerplate.
///
/// This macro processes an impl block and:
/// 1. Collects all methods marked with `#[effect]`
/// 2. Generates helper methods for each effect
/// 3. Automatically creates the `effects()` method
///
/// # Example
///
/// ```ignore
/// #[component]
/// impl MyComponent {
///     #[update]
///     fn update(&self, ctx: &Context, msg: Msg, mut state: State) -> Action {
///         // update logic
///     }
///
///     #[view]
///     fn view(&self, ctx: &Context, state: State) -> Node {
///         // view logic
///     }
///
///     #[effect]
///     async fn timer(&self, ctx: &Context) {
///         // async effect logic
///     }
/// }
/// ```
///
/// The macro will automatically generate the `effects()` method that collects
/// all methods marked with `#[effect]`.
#[proc_macro_attribute]
pub fn component(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(input as ItemImpl);

    // Find all methods marked with #[effect]
    let mut effect_methods = Vec::new();
    let mut processed_items = Vec::new();

    for item in impl_block.items.drain(..) {
        if let ImplItem::Fn(mut method) = item {
            // Check if this method has the #[effect] attribute
            let has_effect_attr = method
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("effect"));

            if has_effect_attr {
                // Remove the #[effect] attribute
                method.attrs.retain(|attr| !attr.path().is_ident("effect"));

                let method_name = &method.sig.ident;
                let helper_name = format_ident!("__{}_effect", method_name);

                // Parse parameters
                let mut params = method.sig.inputs.iter();

                // Skip &self
                params.next();

                // Get context parameter
                let ctx_param = params.next();
                let ctx_name = if let Some(FnArg::Typed(PatType { pat, .. })) = ctx_param {
                    if let Pat::Ident(pat_ident) = &**pat {
                        &pat_ident.ident
                    } else {
                        panic!("Expected context parameter");
                    }
                } else {
                    panic!("Expected context parameter");
                };

                // Check for state parameter
                let state_setup = if let Some(FnArg::Typed(PatType { pat, ty, .. })) = params.next()
                {
                    if let Pat::Ident(pat_ident) = &**pat {
                        let state_name = &pat_ident.ident;
                        let state_type = &**ty;
                        quote! { let #state_name = #ctx_name.get_state::<#state_type>(); }
                    } else {
                        quote! {}
                    }
                } else {
                    quote! {}
                };

                let method_block = &method.block;

                // Generate helper method
                let helper_method = quote! {
                    #[allow(dead_code)]
                    fn #helper_name(&self, #ctx_name: &rxtui::Context) -> rxtui::effect::Effect {
                        Box::pin({
                            let #ctx_name = #ctx_name.clone();
                            #state_setup
                            async move #method_block
                        })
                    }
                };

                // Store effect method info for later
                effect_methods.push((helper_name, ctx_name.clone()));

                // Add both the helper and original method
                let helper_item: ImplItem = syn::parse2(helper_method).unwrap();
                processed_items.push(helper_item);

                // Add #[allow(dead_code)] to the original async method
                method.attrs.push(syn::parse_quote! { #[allow(dead_code)] });
                processed_items.push(ImplItem::Fn(method));
            } else {
                processed_items.push(ImplItem::Fn(method));
            }
        } else {
            processed_items.push(item);
        }
    }

    // Add all processed items back
    impl_block.items = processed_items;

    // Generate effects() method if we found any #[effect] methods
    if !effect_methods.is_empty() {
        let effect_calls = effect_methods
            .iter()
            .map(|(helper_name, _)| {
                quote! { self.#helper_name(ctx) }
            })
            .collect::<Vec<_>>();

        let effects_method = quote! {
            #[cfg(feature = "effects")]
            fn effects(&self, ctx: &rxtui::Context) -> Vec<rxtui::effect::Effect> {
                vec![#(#effect_calls),*]
            }
        };

        let effects_item: ImplItem = syn::parse2(effects_method).unwrap();
        impl_block.items.push(effects_item);
    }

    TokenStream::from(quote! { #impl_block })
}
