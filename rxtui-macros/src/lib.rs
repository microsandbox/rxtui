use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{
    DeriveInput, Expr, FnArg, Ident, ItemFn, LitStr, Pat, PatType, Token, Type, parse_macro_input,
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
        }
    };

    TokenStream::from(expanded)
}

/// Attribute macro that simplifies the update method implementation
///
/// This macro automatically handles message downcasting, state fetching,
/// and topic routing, significantly reducing boilerplate in component update methods.
///
/// # Simple Usage (no topics, no state)
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
/// # With State
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
/// # With Topics (New Syntax)
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
/// # Transformation (ASCII Diagram)
///
/// ```text
/// ┌─────────────────────────────────────────────────────────────────┐
/// │                        USER WRITES THIS:                        │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ #[update(msg = CounterMsg, topics = [self.topic => ResetMsg])]  │
/// │ fn update(&self, ctx: &Context, msg: Messages,                  │
/// │           mut state: CounterState) -> Action {                  │
/// │     match msg {                                                 │
/// │         Messages::CounterMsg(m) => { ... }                      │
/// │         Messages::ResetMsg(m) => { ... }                        │
/// │     }                                                           │
/// │ }                                                               │
/// └─────────────────────────────────────────────────────────────────┘
///                                 │
///                                 ▼
/// ┌─────────────────────────────────────────────────────────────────┐
/// │                    MACRO GENERATES THIS:                        │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ fn update(&self, ctx: &Context,                                 │
/// │           msg: Box<dyn Message>,                                │
/// │           topic: Option<&str>) -> Action {                      │
/// │                                                                 │
/// │     enum Messages { /* generated */ }                           │
/// │     let mut state = ctx.get_state::<CounterState>();            │
/// │                                                                 │
/// │     if let Some(topic) = topic {                                │
/// │         if topic == self.topic.as_ref() { /* dynamic check */   │
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
/// # Parameters (by position)
/// - Position 0: `&self` (required)
/// - Position 1: `&Context` (required) - any parameter name allowed
/// - Position 2: Message type (required) - any parameter name allowed
/// - Position 3: State type (optional) - any parameter name allowed
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

/// Attribute macro that simplifies the view method implementation
///
/// This macro automatically handles state fetching, reducing boilerplate
/// in component view methods.
///
/// # Example (with state)
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
/// # Example (without state)
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
/// # Transformation (ASCII Diagram)
///
/// ```text
/// ┌─────────────────────────────────────────────────────────────────┐
/// │                        USER WRITES THIS:                        │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ #[view]                                                         │
/// │ fn view(&self, context: &Context, my_state: CounterState)       │
/// │         -> Node {                                               │
/// │     // User can use any parameter names they want               │
/// │     node! { ... }                                               │
/// │ }                                                               │
/// └─────────────────────────────────────────────────────────────────┘
///                                 │
///                                 ▼
/// ┌─────────────────────────────────────────────────────────────────┐
/// │                    MACRO GENERATES THIS:                        │
/// ├─────────────────────────────────────────────────────────────────┤
/// │ fn view(&self, context: &Context) -> Node {                     │
/// │     let my_state = context.get_state::<CounterState>();         │
/// │     // User's function body with their chosen names             │
/// │     { node! { ... } }                                           │
/// │ }                                                               │
/// └─────────────────────────────────────────────────────────────────┘
/// ```
///
/// # Parameters (by position)
/// - Position 0: `&self` (required)
/// - Position 1: `&Context` (required) - any parameter name allowed
/// - Position 2: State type (optional) - any parameter name allowed
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
