# FEATURES

## Core Logic & Data Operations
- [x] Basic key-value store operations (GET, SET, DEL).
- [x] Key expiration support (SET with EXP parameter).
- [x] DROP command to clear all data.
- [x] Multi-threaded async server with Tokio runtime.
- [ ] Add INCR and DECR for thread-safe counting.
- [ ] Add support for transactions (MULTI/EXEC).
- [ ] Implement a basic eviction policy for memory management (Eviction Policy, LRU, LFU, etc.).

## Persistence & Disaster Recovery
- [x] SAVE/LOAD commands for JSON-based persistence.
- [x] Signal handling for graceful shutdown (Ctrl+C).
- [ ] Implement AOF (Append Only File) for real-time data safety.
- [ ] Save a backup every x time to prevent data loss (scheduled backups).
- [ ] Handle OS signals to save data before exiting (SIGTERM, etc.).

## Protocol & Networking
- [x] Pub/Sub messaging system (PUB/SUB/UNSUB commands).
- [x] Client connection tracking and cleanup.
- [ ] Support the official Redis Serialization Protocol (RESP).
- [ ] Implement a simple replication mechanism for data redundancy.
- [ ] Add TELL command for server-to-server communication in a cluster setup.

## Security & Configuration
- [ ] Implement an AUTH command for password protection.
- [ ] Add a CONFIG command for runtime configuration changes.
- [ ] Support config files or ENV vars for port, passwords, and limits.

## Observability & Performance
- [ ] Add an INFO command for server stats and memory usage.
- [ ] Implement structured logging (using `tracing` or `log` crate).
- [ ] Add a simple monitoring endpoint (e.g., /metrics) for Prometheus integration.
