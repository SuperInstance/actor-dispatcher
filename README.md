# Actor Dispatcher Pattern

An **actor dispatcher** routes messages to actor instances based on a dispatching strategy (round-robin, least-loaded, hash-based, or content-based).

## Why It Matters

In the actor model, dispatchers decouple message producers from consumers. They enable load balancing, affinity routing, and bounded mailbox depths — essential for building responsive, resilient concurrent systems.

## How It Works

Implements the actor dispatch protocol: each actor has a mailbox, messages are dispatched by strategy, backpressure is applied when mailboxes fill. Supports both push (eager) and pull (lazy) dispatch modes.

## Usage

```toml
[dependencies]
actor-dispatcher = "0.1.0"
```

```rust
use actor_dispatcher;

// See examples/ directory for detailed usage
```

## API

- `Dispatcher` (lib.rs)

## Architecture

This crate is part of the **[SuperInstance](https://github.com/SuperInstance)** ecosystem — a conservation-law-based framework for fleet coordination, ternary computation, and distributed agent systems.

### Related Crates

- [`superinstance-core`](https://github.com/SuperInstance/superinstance-core) — Core conservation law (γ + η = C)
- [`superinstance-harness`](https://github.com/SuperInstance/superinstance-harness) — Build harness and self-improving loop
- [`fleet-coordinator`](https://github.com/SuperInstance/fleet-coordinator) — Fleet-level coordination

## References

- [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)
- [Conservation Law Paper](https://github.com/SuperInstance/SuperInstance/blob/main/docs/conservation-law.md)

## License

MIT
