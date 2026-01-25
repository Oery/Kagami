*__Work In Progress__: Kagami currently only supports a few packets and doesn't support encryption at all.*

*As it is my study case, a very long time may pass before a release happens.*

# Kagami

Mods are powerful but require to be made specifically for a Minecraft version and for a specific Modloader. With closed source clients, they're sometimes not possible at all.

**Kagami** let you write custom code that runs on every Minecraft (java) version, every modloader, every closed source client, etc... It's all done through the magic of networking.

## Example

This is an example of a basic proxy with a health check command.

```rust
#[async_std::main]
async fn main() -> Result<(), KagamiError> {
    let mut app = Proxy::new();

    // This creates a client-side command
    // This handler will run every time the client sends a chat message
    app.events.on_packet::<Chat>(|ctx| {
        if ctx.payload.message == "/ping" {
            // We cancel the packet it doesn't get sent to the server
            ctx.cancel_packet();
            // We can send a chat message directly to the client
            ctx.client.chat("Pong!");
        }
    });

    app.run().await
}
```

## Goals

- **Multiversion**: The project is currently being developed for Minecraft 1.8.9 as more modern versions of the game are easier to mod and closed source clients are a lot less popular with those. However, it is part of the project to allow multiversion projects.
- **Encryption**: This would allow for Kagami to be compatible with onlines server, aka as 99% of Minecraft servers.
- **Reverse Mode**: Kagami is designed to run on the client side, but there are use cases for running it server side.
- **Clap Integration**: clap is known as one of the best tools to make CLIs, maybe it could be part of some Command API?
