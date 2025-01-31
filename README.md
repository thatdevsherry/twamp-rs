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
--number-of-test-packets 100 \
--timeout 10

# Test with release 
./run_responder
./run_client

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
- [x] metrics

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

## Example run

Tested on localhost with increased udp buffer.

```bash
sudo sysctl -w net.core.rmem_max=16777216
sudo sysctl -w net.core.rmem_default=16777216
```

```bash
❯ ./run_controller
    Finished `release` profile [optimized] target(s) in 0.03s
2025-01-31T07:29:30.601345Z  INFO controller: Controller initialized
2025-01-31T07:29:30.601512Z  INFO control_client: Reading ServerGreeting
2025-01-31T07:29:30.601572Z  INFO control_client: Done reading ServerGreeting
2025-01-31T07:29:30.601582Z  INFO control_client: Preparing to send Set-Up-Response
2025-01-31T07:29:30.601637Z  INFO control_client: Set-Up-Response sent
2025-01-31T07:29:30.601643Z  INFO control_client: Reading Server-Start
2025-01-31T07:29:30.601679Z  INFO control_client: Done reading Server-Start
2025-01-31T07:29:30.601685Z  INFO control_client: Preparing to send Request-TW-Session
2025-01-31T07:29:30.601714Z  INFO control_client: Request-TW-Session sent
2025-01-31T07:29:30.601719Z  INFO control_client: Reading Accept-Session
2025-01-31T07:29:30.601765Z  INFO control_client: Read Accept-Session
2025-01-31T07:29:30.601770Z  INFO control_client: Preparing to send Start-Sessions
2025-01-31T07:29:30.601787Z  INFO control_client: Start-Sessions sent
2025-01-31T07:29:30.601791Z  INFO control_client: Reading Start-Ack
2025-01-31T07:29:30.601808Z  INFO control_client: Done reading Start-Ack
2025-01-31T07:29:30.601827Z  INFO session_sender: Sending Twamp-Test packets to 127.0.0.1:42572
2025-01-31T07:29:30.851552Z  INFO controller::controller: Sent all test packets
2025-01-31T07:29:30.864403Z  INFO controller::controller: Got back all test packets
2025-01-31T07:29:30.864434Z  INFO control_client: Preparing to send Stop-Sessions
2025-01-31T07:29:30.864460Z  INFO control_client: Stop-Sessions sent
2025-01-31T07:29:30.864477Z  INFO controller::controller: Producing metrics
2025-01-31T07:29:30.864485Z  INFO controller::controller: Packet loss: 0%
2025-01-31T07:29:30.866136Z  INFO controller::controller: RTT (MIN): 0.00ms
2025-01-31T07:29:30.866147Z  INFO controller::controller: RTT (MAX): 3.23ms
2025-01-31T07:29:30.866149Z  INFO controller::controller: RTT (AVG): 1.39ms
2025-01-31T07:29:30.866150Z  INFO controller::controller: OWD (Sender -> Reflector) (AVG): 0.99ms
2025-01-31T07:29:30.866153Z  INFO controller::controller: OWD (Reflector -> Sender) (AVG): 0.40ms
2025-01-31T07:29:30.866345Z  INFO controller::controller: Jitter: 0.00ms
```
