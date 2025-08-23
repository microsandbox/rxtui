//! Async effects system for running background tasks in components
//!
//! Effects allow components to spawn async tasks that run outside the main
//! event loop. They can perform I/O, timers, network requests, etc. and
//! communicate back to the UI through messages.
//!
//! # Quick Start
//!
//! The easiest way to use effects is with the `#[component]` and `#[effect]` macros:
//!
//! ```ignore
//! use rxtui::prelude::*;
//! use std::time::Duration;
//!
//! #[derive(Component, Clone)]
//! struct Timer;
//!
//! #[component]  // Automatically collects all #[effect] methods
//! impl Timer {
//!     #[update]
//!     fn update(&self, ctx: &Context, msg: TimerMsg, mut state: TimerState) -> Action {
//!         match msg {
//!             TimerMsg::Tick => {
//!                 state.elapsed += 1;
//!                 Action::update(state)
//!             }
//!         }
//!     }
//!
//!     #[view]
//!     fn view(&self, ctx: &Context, state: TimerState) -> Node {
//!         node! {
//!             div [
//!                 text(format!("Elapsed: {}s", state.elapsed))
//!             ]
//!         }
//!     }
//!
//!     #[effect]
//!     async fn tick(&self, ctx: &Context) {
//!         loop {
//!             tokio::time::sleep(Duration::from_secs(1)).await;
//!             ctx.send(TimerMsg::Tick);
//!         }
//!     }
//! }
//! ```
//!
//! # How Effects Work
//!
//! 1. **Lifecycle**: Effects are spawned when a component mounts and cancelled when it unmounts
//! 2. **Concurrency**: Multiple effects run concurrently in the Tokio runtime
//! 3. **Communication**: Effects communicate with components via `ctx.send()` messages
//! 4. **State Access**: Effects can access component state via optional state parameter
//!
//! # Advanced Usage
//!
//! ## Multiple Effects
//!
//! Components can have multiple effects for different concerns:
//!
//! ```ignore
//! #[component]
//! impl Dashboard {
//!     #[effect]
//!     async fn fetch_data(&self, ctx: &Context) {
//!         let data = fetch_from_api().await;
//!         ctx.send(DashboardMsg::DataLoaded(data));
//!     }
//!
//!     #[effect]
//!     async fn websocket_listener(&self, ctx: &Context) {
//!         let mut ws = connect_websocket().await;
//!         while let Some(msg) = ws.next().await {
//!             ctx.send(DashboardMsg::WebSocketMessage(msg));
//!         }
//!     }
//!
//!     #[effect]
//!     async fn auto_refresh(&self, ctx: &Context) {
//!         loop {
//!             tokio::time::sleep(Duration::from_secs(30)).await;
//!             ctx.send(DashboardMsg::Refresh);
//!         }
//!     }
//! }
//! ```
//!
//! ## Accessing State in Effects
//!
//! Effects can read component state by adding a state parameter:
//!
//! ```ignore
//! #[effect]
//! async fn monitor(&self, ctx: &Context, state: AppState) {
//!     // State is automatically fetched via ctx.get_state()
//!     if state.threshold_exceeded() {
//!         send_alert(&state.alert_config).await;
//!         ctx.send(AppMsg::AlertSent);
//!     }
//! }
//! ```
//!
//! ## Manual Implementation
//!
//! For full control, implement the `effects()` method manually:
//!
//! ```ignore
//! impl MyComponent {
//!     fn effects(&self, ctx: &Context) -> Vec<Effect> {
//!         vec![
//!             // First effect
//!             Box::pin({
//!                 let ctx = ctx.clone();
//!                 async move {
//!                     // Async logic here
//!                 }
//!             }),
//!             // Second effect
//!             Box::pin({
//!                 let ctx = ctx.clone();
//!                 let state = ctx.get_state::<MyState>();
//!                 async move {
//!                     // Async logic with state
//!                 }
//!             }),
//!         ]
//!     }
//! }
//! ```
//!
//! # Common Patterns
//!
//! ## Cancellable Operations
//!
//! Effects are automatically cancelled when components unmount, but you can
//! also handle cancellation explicitly:
//!
//! ```ignore
//! #[effect]
//! async fn cancellable_task(&self, ctx: &Context) {
//!     let handle = spawn_cancellable_task();
//!
//!     // This will be cancelled if component unmounts
//!     tokio::select! {
//!         result = handle => {
//!             ctx.send(MyMsg::TaskComplete(result));
//!         }
//!     }
//! }
//! ```
//!
//! ## Error Handling
//!
//! Always handle errors gracefully in effects:
//!
//! ```ignore
//! #[effect]
//! async fn network_task(&self, ctx: &Context) {
//!     match fetch_data().await {
//!         Ok(data) => ctx.send(MyMsg::Success(data)),
//!         Err(e) => ctx.send(MyMsg::Error(e.to_string())),
//!     }
//! }
//! ```

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

mod runtime;
mod types;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use runtime::EffectRuntime;
pub use types::{Effect, EffectsProvider};
