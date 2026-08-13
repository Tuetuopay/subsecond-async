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
