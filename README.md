# twamp-rs

WIP TWAMP [RFC 5357](https://datatracker.ietf.org/doc/rfc5357/) implementation
in rust.

```bash
# Run server first.
> RUST_LOG="debug" cargo watch -x "run"

# Run client
> RUST_LOG="debug" cargo r -- --server 127.0.0.1
```

## Roadmap/Features

### Controller

- [x] establish TCP connection to server
- [x] read server greeting
- [x] send set-up-response
- [x] read server-start
- [ ] send request-tw-session
... will add more once these work

### Responder

- [x] handle TCP connection from controller
- [x] send server greeting
- [x] read set-up-response
- [x] send server-start
- [ ] read request-tw-session
... will add more once these work

### TWAMP-Control (unauthenticated only)

- [x] `ServerGreeting` as per RFC
- [x] `SetUpResponse` as per RFC
- [ ] `ServerStart` as per RFC
... add more once these work
