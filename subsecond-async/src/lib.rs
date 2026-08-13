//! # subsecond-async
//!
//! Compatibility layer for [`subsecond`] to use with async runtimes like Tokio. This merely
//! provides async-aware helpers.
//!
//! ## Usage
//!
//! To call sync code, use the regular [`subsecond::call`] function. To call async code, use the
//! provided [`call`] function, passing it a future.
//!
//! ```rust
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! for x in 0..5 {
//!     subsecond_async::call(async || {
//!         println!("Tick {x}!");
//!     });
//! }
//! # }
//! ```
//!
//! To actually load patches into your applications, follow the regular [`subsecond`] usage guide.

use std::{pin::Pin, task::{Context, Poll}};

/// A wrapper around a future that support [`subsecond`] hotpatching.
///
/// Wrap any future using [`SubsecondFuture::new`] to have it hotpatchable using [`subsecond`].
#[pin_project::pin_project]
pub struct SubsecondFuture<F>(#[pin] F);

impl<F: Future> SubsecondFuture<F> {
    /// Create a new future that can be hotpatched, wrapping another future.
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F: Future> Future for SubsecondFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = Some(self.project().0);
        // Unwrap safety: subsecond::call takes an FnMut purely for typesystem and move semantics.
        // It needs to retry calling because subsecond itself may fail, but it will only ever call
        // the function once. However, since the closure is moved, they can't take FnOnce.
        // This is a no-op outside of debug mode anyways.
        subsecond::call(|| inner.take().expect("Subsecond called us twice.").poll(cx))
    }
}

/// Call a given async function with hot-reloading enabled. If the future's code changes, `call`
/// will use the newer version of the function.
///
/// Refer to [`subsecond::call`] for more details.
pub fn call<Fut: Future, F: FnOnce() -> Fut>(f: F) -> SubsecondFuture<Fut> {
    let mut op = Some(f);
    subsecond::call(|| SubsecondFuture(op.take().expect("Subsecond called twice")()))
}

/// Same as [`subsecond::call`], without the `FnMut` requirement.
pub fn call_sync<O>(f: impl FnOnce() -> O) -> O {
    let mut op = Some(f);
    subsecond::call(move || op.take().expect("Subsecond called twice.")())
}
