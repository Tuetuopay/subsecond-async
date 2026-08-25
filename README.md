# subsecond-async

Compatibility layer for [`subsecond`] to use with async runtimes like Tokio.
This merely provides async-aware helpers.

## Usage

To call sync code, use the regular [`subsecond::call`] function. To call async
code, use the provided [`call`] function, passing it a future.

```rust
# #[tokio::main(flavor = "current_thread")]
# async fn main() {
for x in 0..5 {
    subsecond_async::call(async || {
        println!("Tick {x}!");
    });
}
# }
```

To actually load patches into your applications, follow the regular
[`subsecond`] usage guide.

A `subsecond` macro is available with the `macros` feature (enabled by default)
that enables a function to easily be marked as subsecond-compatible.

```rust
# use subsecond_async::subsecond;
# #[tokio::main(flavor = "current_thread")]
# async fn main() {
for i in 0.. {
    tick(i).await;
}
# }

#[subsecond]
async fn tick(i: i32) {
    for j in 0..5 {
        println!("[{j}] Hello, tick {i}!");
    }
}
```

## Warnings

Here be dragons. Due to how subsecond is built, this library has *less*
guarantees than regular subsecond. Hotpatching is already difficult as-is, and
with async it becomes harder. In particular, expect worse support to reload a
long-lived function. This should happen rarely for async code, though it's
better to use subsecond on async functions that will be called from scratch
after patching (e.g. request handlers, tick functions, etc).

Thus, expect *more* crashes and issues than with regular subsecond for sync
code.

Since this is a development tool that gains a lot of time even when it works
90% of the time, I feel like it is already useful and worth it as-is.
