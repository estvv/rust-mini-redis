# Multi-threaded Key-Value Store

A multi-threaded key-value store server implementation in Rust with persistence, TTL support, and pub/sub messaging.

## Overview

This project implements a TCP-based key-value store server with thread-safe concurrent access and publish/subscribe messaging. Built in Rust, it demonstrates concurrent programming concepts including shared state management with `Arc<Mutex<T>>`, non-blocking I/O, broadcast channels, and graceful shutdown handling.

## Features

- **Multi-threaded Architecture**: Each client connection is handled in a separate thread, allowing concurrent client access
- **Thread-safe State**: Uses `Arc<Mutex<Dispatcher>>` pattern to safely share the key-value store across threads
- **Publish/Subscribe**: Real-time messaging with channel-based pub/sub system
- **Client Tracking**: Each client has a unique ID for subscription management
- **TTL Support**: Keys can be set with optional expiration times (in milliseconds)
- **Persistence**: Save and load the key-value store state to/from JSON files
- **Graceful Shutdown**: Handles Ctrl+C (SIGINT) for clean server termination

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `GET <key>` | Retrieve a value by key | `GET mykey` |
| `SET <key> <value> [EXP <ms>]` | Set a key-value pair with optional expiration | `SET mykey hello` or `SET mykey hello EXP 5000` |
| `DEL <key>` | Delete a key | `DEL mykey` |
| `SAVE <filename.json>` | Save current state to `./data/<filename.json>` | `SAVE dump.json` |
| `LOAD <filename.json>` | Load state from `./data/<filename.json>` | `LOAD dump.json` |
| `DROP` | Clear all keys from the store | `DROP` |
| `PUB <channel> <message>` | Publish a message to a channel | `PUB news Hello World` |
| `SUB <channel>` | Subscribe to a channel | `SUB news` |
| `UNSUB <channel>` | Unsubscribe from a channel | `UNSUB news` |

Expirations are specified in milliseconds. Keys with expired TTL are automatically removed when accessed via `GET`.

**Pub/Sub Notes:**
- Each client can only be subscribed to one channel at a time
- Subscribing to a new channel replaces the previous subscription
- Messages are broadcast to all subscribers on the channel
- Unsubscribing from a non-existent or non-subscribed channel returns an error

## Project Structure

```
src/
├── main.rs            # Server entry point, connection handling, request processing
├── dispatcher.rs      # Request routing, command execution, client subscription tracking
├── request.rs         # Request parsing (GET, SET, DEL, SAVE, LOAD, DROP, PUB, SUB, UNSUB)
├── channel_manager.rs # Pub/sub channel management with broadcast channels
├── stock.rs           # Key-value storage with expiration support
└── returns.rs         # Return types (Ok, Err, NotFound, Subscribe, Unsubscribe)
```

## Implementation Details

### Architecture

- **Server**: Non-blocking TCP listener on `127.0.0.1:6379` with read timeouts
- **Concurrency**: Uses `Arc<Mutex<Dispatcher>>` for thread-safe state management, `tokio::select!` for handling subscriptions
- **Client IDs**: Each client is assigned a unique ID using `AtomicU64` counter for subscription tracking
- **State Management**: `Dispatcher` wraps `Stock` (the data store) and `ChannelManager` in `Arc<Mutex<...>>` for safe concurrent access
- **Client Handler**: Each connection spawns a Tokio task that reads line-by-line and dispatches commands
- **Pub/Sub**: Uses `tokio::sync::broadcast` channels for message distribution to multiple subscribers

### Data Storage

```rust
struct Data {
    value: String,
    expiration: Option<u64>,  // Unix timestamp in milliseconds
}

struct Stock {
    map: HashMap<String, Data>,
}
```

Data is persisted as JSON using `serde_json`.

### Pub/Sub System

```rust
pub struct Dispatcher {
    stock: Stock,
    channel_manager: ChannelManager,
    client_subscriptions: HashMap<u64, String>,  // client_id -> channel_name
}

pub struct ChannelManager {
    channels: HashMap<String, broadcast::Sender<String>>,  // channel_name -> sender
}
```

**Client Tracking**: The `Dispatcher` maintains a mapping of client IDs to their subscribed channels, enabling proper validation and cleanup.

**Broadcast Channels**: Each channel uses a `tokio::sync::broadcast` channel with capacity 16. Multiple receivers can subscribe to a single sender.

**Cleanup**: When clients disconnect, their subscription is automatically removed from the tracking map.

### Execution Flow

1. Server binds to port 6379 and listens for connections
2. On new connection, assigns a unique client ID and spawns a Tokio task
3. Client task reads commands line-by-line
4. Commands are parsed into `Request` enum variants
5. `Dispatcher` routes to appropriate handler with client ID
6. `Stock` or `ChannelManager` performs the operation (with mutex lock held)
7. Response is written back to client
8. If subscribed, client uses `tokio::select!` to handle both incoming commands and channel messages

## Dependencies

- `tokio` - Async runtime with networking and sync primitives
- `serde` & `serde_json` - JSON serialization for persistence

## Running

```bash
# Build and run
cargo run

# The server starts on 127.0.0.1:6379
```

In another terminal, connect with:

```bash
nc localhost 6379
# or
telnet localhost 6379
```

Example session:
```
SET username alice
OK
GET username
alice
SET temp data EXP 3000
OK
GET temp
data
# After 3 seconds:
GET temp
Key 'temp' not found
SAVE mydata.json
OK
```

### Pub/Sub Example

Terminal 1 (Subscriber):
```
SUB news
Subscribed
[Waiting for messages...]
MESSAGE news Breaking: Rust 2.0 released!
MESSAGE news Update: Performance improvements
UNSUB news
Unsubscribed
```

Terminal 2 (Publisher):
```
PUB news Breaking: Rust 2.0 released!
Published to 1 subscriber(s)
PUB news Update: Performance improvements
Published to 1 subscriber(s)
```

### Error Handling Examples

```
UNSUB news
Error: Not subscribed to any channel

SUB news
Subscribed
UNSUB sports
Error: Not subscribed to channel 'sports'

UNSUB nonexistent
Error: Channel 'nonexistent' does not exist
```

## Graceful Shutdown

Press `Ctrl+C` to gracefully shutdown the server. The server will:
1. Stop accepting new connections
2. Signal all client threads to shut down
3. Exit cleanly

## Project History

This project was developed incrementally:

1. Basic client-server with `GET`, `SET`, `DEL` commands
2. Multi-threading with mutex for concurrent client access
3. `SAVE`, `LOAD`, and `DROP` commands for persistence
4. Graceful shutdown and dynamic reading with non-blocking I/O
5. **Pub/Sub system**: Added `PUB`, `SUB`, `UNSUB` commands with broadcast channels
6. **Client tracking**: Implemented unique client IDs and subscription validation to prevent disconnection issues