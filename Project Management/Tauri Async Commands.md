# Module [tauri](https://docs.rs/tauri/1.6.1/tauri/index.html)::[async_runtime](https://docs.rs/tauri/1.6.1/tauri/async_runtime/index.html#)![Copy item path](https://docs.rs/-/rustdoc.static/clipboard-7571035ce49a181d.svg)

[source](https://docs.rs/tauri/1.6.1/src/tauri/async_runtime.rs.html#5-348) · [−]

The singleton async runtime used by Tauri and exposed to users.

Tauri uses [`tokio`](https://docs.rs/tokio/1.36.0/x86_64-unknown-linux-gnu/tokio/index.html "mod tokio") Runtime to initialize code, such as [`Plugin::initialize`](https://docs.rs/tauri/1.6.1/tauri/plugin/trait.Plugin.html#method.initialize) and [`crate::Builder::setup`](https://docs.rs/tauri/1.6.1/tauri/struct.Builder.html#method.setup "method tauri::Builder::setup") hooks. This module also re-export some common items most developers need from [`tokio`](https://docs.rs/tokio/1.36.0/x86_64-unknown-linux-gnu/tokio/index.html "mod tokio"). If there’s one you need isn’t here, you could use types in [`tokio`](https://docs.rs/tokio/1.36.0/x86_64-unknown-linux-gnu/tokio/index.html "mod tokio") directly. For custom command handlers, it’s recommended to use a plain `async fn` command.

## Structs

- [Mutex](https://docs.rs/tauri/1.6.1/tauri/async_runtime/struct.Mutex.html "struct tauri::async_runtime::Mutex")
    
    An asynchronous `Mutex`-like type.
    
- [Receiver](https://docs.rs/tauri/1.6.1/tauri/async_runtime/struct.Receiver.html "struct tauri::async_runtime::Receiver")
    
    Receives values from the associated `Sender`.
    
- [RwLock](https://docs.rs/tauri/1.6.1/tauri/async_runtime/struct.RwLock.html "struct tauri::async_runtime::RwLock")
    
    An asynchronous reader-writer lock.
    
- [Sender](https://docs.rs/tauri/1.6.1/tauri/async_runtime/struct.Sender.html "struct tauri::async_runtime::Sender")
    
    Sends values to the associated `Receiver`.
    
- [TokioHandle](https://docs.rs/tauri/1.6.1/tauri/async_runtime/struct.TokioHandle.html "struct tauri::async_runtime::TokioHandle")
    
    Handle to the runtime.
    
- [TokioJoinHandle](https://docs.rs/tauri/1.6.1/tauri/async_runtime/struct.TokioJoinHandle.html "struct tauri::async_runtime::TokioJoinHandle")
    
    An owned permission to join on a task (await its termination).
    
- [TokioRuntime](https://docs.rs/tauri/1.6.1/tauri/async_runtime/struct.TokioRuntime.html "struct tauri::async_runtime::TokioRuntime")
    
    The Tokio runtime.
    

## Enums

- [JoinHandle](https://docs.rs/tauri/1.6.1/tauri/async_runtime/enum.JoinHandle.html "enum tauri::async_runtime::JoinHandle")
    
    An owned permission to join on a task (await its termination).
    
- [Runtime](https://docs.rs/tauri/1.6.1/tauri/async_runtime/enum.Runtime.html "enum tauri::async_runtime::Runtime")
    
    A runtime used to execute asynchronous tasks.
    
- [RuntimeHandle](https://docs.rs/tauri/1.6.1/tauri/async_runtime/enum.RuntimeHandle.html "enum tauri::async_runtime::RuntimeHandle")
    
    A handle to the async runtime
    

## Functions

- [block_on](https://docs.rs/tauri/1.6.1/tauri/async_runtime/fn.block_on.html "fn tauri::async_runtime::block_on")
    
    Runs a future to completion on runtime.
    
- [channel](https://docs.rs/tauri/1.6.1/tauri/async_runtime/fn.channel.html "fn tauri::async_runtime::channel")
    
    Creates a bounded mpsc channel for communicating between asynchronous tasks with backpressure.
    
- [handle](https://docs.rs/tauri/1.6.1/tauri/async_runtime/fn.handle.html "fn tauri::async_runtime::handle")
    
    Returns a handle of the async runtime.
    
- [set](https://docs.rs/tauri/1.6.1/tauri/async_runtime/fn.set.html "fn tauri::async_runtime::set")
    
    Sets the runtime to use to execute asynchronous tasks. For convenience, this method takes a [`TokioHandle`](https://docs.rs/tauri/1.6.1/tauri/async_runtime/struct.TokioHandle.html "struct tauri::async_runtime::TokioHandle"). Note that you cannot drop the underlying [`TokioRuntime`](https://docs.rs/tauri/1.6.1/tauri/async_runtime/struct.TokioRuntime.html "struct tauri::async_runtime::TokioRuntime").
    
- [spawn](https://docs.rs/tauri/1.6.1/tauri/async_runtime/fn.spawn.html "fn tauri::async_runtime::spawn")
    
    Spawns a future onto the runtime.
    
- [spawn_blocking](https://docs.rs/tauri/1.6.1/tauri/async_runtime/fn.spawn_blocking.html "fn tauri::async_runtime::spawn_blocking")
    
    Runs the provided function on an executor dedicated to blocking operations.