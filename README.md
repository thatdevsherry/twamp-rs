# twamp-rs

WIP TWAMP [RFC 5357](https://datatracker.ietf.org/doc/rfc5357/) implementation
in rust.

Currently building for unauthenticated mode only.

```bash
# Run server first.
> cargo run -p responder -- -p 4000 # defaults to 862 which needs permissions

# Run client
> cargo run -p controller -- \
--responder-addr 127.0.0.1 \
--responder-port 4000 \
--controller-addr 127.0.0.1 \
--responder-reflect-port 4001 \
--number-of-test-packets 1

# Tests
> cargo test --workspace

# Open docs in browser
> cargo doc --workspace --no-deps --open
```

## Roadmap/Features

### Controller

- [x] establish TCP connection to server
- [x] read server greeting
- [x] send set-up-response
- [x] read server-start
- [x] send request-tw-session
- [x] read accept-session
- [x] send start-sessions
- [x] read start-ack
- [x] twamp-test
- [x] send stop-sessions
- [ ] configurable twamp-test
- [ ] configurable twamp-control
- [ ] output for use in metrics (jitter, RTT etc)

### Responder

- [x] handle TCP connection from controller
- [x] send server greeting
- [x] read set-up-response
- [x] send server-start
- [x] read request-tw-session
- [x] send accept-session
- [x] read start-session
- [x] send start-ack
- [x] twamp-test
- [x] read stop-sessions
- [ ] configurable twamp-test
- [ ] configurable twamp-control
