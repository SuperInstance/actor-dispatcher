# Actor Dispatcher

**Actor Dispatcher** is a Rust library implementing a bounded message queue with back-pressure semantics for dispatching messages to actors, preventing unbounded memory growth when message production outpaces consumption.

## Why It Matters

In the actor model, messages are delivered to per-actor mailboxes. Without back-pressure, a fast producer can exhaust memory by filling a slow consumer's mailbox. The Dispatcher solves this with a bounded capacity queue: when full, `try_push` returns `Err(msg)`, signaling the sender to slow down, batch, or drop. This is critical for real-time systems where predictable memory usage matters more than unbounded throughput. Without back-pressure mechanisms, distributed systems enter cascading failure modes — one slow service backs up queues everywhere upstream, eventually causing OOM crashes.

## How It Works

The dispatcher wraps a `VecDeque<T>` with a fixed `capacity`. The algorithm is straightforward:

```
try_push(msg):
  if queue.len() >= capacity:
    return Err(msg)     // O(1) rejection
  queue.push_back(msg)  // amortized O(1)
  return Ok(())

pop():
  queue.pop_front()     // O(1)
```

The back-pressure strategy is **synchronous rejection**: the producer receives immediate feedback (the message is returned as an error), forcing it to adapt. This contrasts with:

- **Drop strategies**: silently discard (fastest, lossy)
- **Buffer strategies**: grow unboundedly (lossless until OOM)
- **Rate-limiting strategies**: throttle producer timing (requires coordination)

Synchronous rejection is optimal when the producer can meaningfully handle rejection (retry, route elsewhere, or aggregate). The capacity should be sized to the consumer's burst-processing window: `capacity ≈ rate_consumer × latency_tolerance`.

## Quick Start

```rust
fn main() {
    let mut d: Dispatcher<i32> = Dispatcher::new(2);
    assert!(d.try_push(1).is_ok());
    assert!(d.try_push(2).is_ok());
    assert_eq!(d.try_push(3), Err(3));  // full → back-pressure
    assert_eq!(d.pop(), Some(1));        // free a slot
    assert!(d.try_push(3).is_ok());      // now succeeds
}
```

## API

| Method | Signature | Complexity |
|--------|-----------|------------|
| `new` | `(capacity: usize) → Self` | O(capacity) |
| `try_push` | `(T) → Result<(), T>` | O(1) amortized |
| `pop` | `() → Option<T>` | O(1) |
| `len` | `() → usize` | O(1) |
| `is_empty` | `() → bool` | O(1) |
| `capacity` | `() → usize` | O(1) |

## Architecture Notes

The Dispatcher implements the **flow-control layer** in the SuperInstance actor system. Within γ + η = C, it enforces conservation of message flow: when the γ (action) layer produces events faster than the η (intelligence) layer can process them, the dispatcher's bounded queue prevents unbounded buffering, maintaining system stability.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

**Little's Law connection:** The optimal capacity can be derived from Little's Law: L = λ × W, where L = average queue length, λ = arrival rate, W = mean processing time. Setting capacity ≈ 2L provides a safety buffer for burst arrivals while bounding worst-case latency to 2W. For example, with λ = 1000 msg/s and W = 5ms, the optimal capacity is 2 × 1000 × 0.005 = 10 messages — enough to absorb bursts without excessive latency.

**Comparison with reactive streams:** The Dispatcher's synchronous rejection model is equivalent to the `Strategy.REJECT` operator in Project Reactor and the `OverflowStrategy.dropHead` in Akka Streams. The key advantage over asynchronous backpressure signaling (like reactive-streams `request(n)`) is simplicity: no coordination channel is needed, and the producer receives feedback in the same call stack.

## References

1. Hewitt, C. (1973). "A Universal Modular Actor Formalism for Artificial Intelligence." *IJCAI*.
2. Nygard, M. (2018). *Release It!* 2nd ed. Pragmatic Bookshelf. Chapter 5: Stability Patterns.
3. Little, J.D.C. (1961). "A Proof for the Queuing Formula L = λW." *Operations Research*, 9(3), 383–387.

## License

MIT
