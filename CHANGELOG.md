## [0.2.0] - 2025-11-16

### 🚀 Features

- [**breaking**] Update deku and address breaking changes

### 🐛 Bug Fixes

- [**breaking**] Bypass constants module
- [**breaking**] Bypass middle modules
- *(README)* Imgshields
- *(README)* Remove --workspace from test
- Imports in docs
- *(.github)* Remove github pages

### ⚙️ Miscellaneous Tasks

- *(README)* Add imgshields
- *(README)* Add feat checklist
- *(README)* Remove workspace flag in doc
## [0.1.0] - 2025-11-09

### 🚀 Features

- *(twamp-control)* Add twamp-control crate
- *(twamp-test)* Add twamp-test crate
- *(session-sender)* Add crate
- *(session-reflector)* Add crate
- *(control-client)* Add crate
- *(server)* Add crate
- *(controller)* Add crate
- *(responder)* Add crate
- Add local deps
- *(twamp-control)* Add constants file
- *(twamp-control)* Add twamp-control commands
- *(control-client)* Connection logic
- *(server)* Connection logic
- *(responder)* Initial impl
- *(controller)* Initial impl
- Add support for Request-TW-Session
- Add a bit of docs
- Use enums in struct
- Serde accept-session properly + tryinto
- Use deku instead of serde & bincode
- Add tests for greeting + struct visibility
- Add tests for Set-Up-Response
- Add timestamp + move accept enum to own mod
- *(twamp-control)* Count getter for greeting
- *(twamp-control)* Server-start timestamp integ.
- Request-tw-session tests + misc
- *(twamp-control)* Start/start-ack/stop session
- *(twamp-control)* Use deku assert_eq for mbz
- *(twamp-control)* Use deku for command number
- Add start/stop session
- [**breaking**] WIP push machine change
- *(twamp-test)* Sender packet impl + tests
- Make twamp-test work a bit
- [**breaking**] Controller refactor ultra instinct
- Refactor into tasks + some configurable params
- More work
- *(controller)* Jitter + owd
- *(.github)* [**breaking**] Add jobs for doc build and deploy

### 🐛 Bug Fixes

- *(twamp-control)* Failing tests
- *(control-client)* Send request-tw-session
- *(twamp-control)* Failing doctests
- *(server)* Fix start-ack log statement
- *(twamp-control)* Failing test
- *(.github)* [**breaking**] Fix syntax error in workflow yml
- *(.github)* [**breaking**] Make index.html
- [**breaking**] Restructure to non-workspace
- Imports in test modules

### 🚜 Refactor

- Move timestamp to own crate

### ⚙️ Miscellaneous Tasks

- Cargo new
- Update Cargo.lock
- *(twamp-test)* Add tracing dep
- *(control-client)* Add deps
- *(server)* Add deps
- *(session-sender)* Add tracing dep
- *(session-reflector)* Add tracing dep
- *(twamp-control)* Add deps
- *(controller, responder)* Add deps
- *(session-reflector)* Dummy impl
- *(README)* Add smol README
- *(README)* Add run commands
- *(README)* Add initial roadmap
- *(controller, control-client)* Move logic
- Use write_all instead of write
- Add more docs
- *(twamp-control)* Move enums to own folder + tests
- Use num_enum for c style enums
- *(README)* Fix example commands
- *(README)* Add doc cmd
- *(README)* Add cmd for tests
- *(README)* Update done stuff
- *(README)* Add unauth mode only note
- *(twamp-test)* Add deku dependency
- *(timestamp)* Impl basic Display trait
- *(twamp-test)* Break down to modules
- *(session-sender)* Add twamp-test dependency
- *(webV2)* Add twamp-test well known port
- Deps in cargo tomls
- *(README)* Add more to roadmap
- Address concerns of clippy ustaad
- *(README)* Remove trailing slash of multi-line cmd
- Scripts for client/server testing
- *(README)* Update script example
- *(README)* Add example run logs
- *(README)* Mark metrics as done
- *(README)* Add cmds for increase udp buffer
- *(server)* Code cleanup
- *(README)* Remove completed items
- Cleanup examples and src
- Update README/scripts with new paths
- Bump rust to 2024 edition
- Update all crates to rust 2024 edition
- Add some docs for hyperlink fix
- *(.github)* [**breaking**] Verbose output of cargo doc
- *(.github)* [**breaking**] Output rust version in CI
- *(src)* Remove comments to test doc
- *(.github)* Cleanups
- *(.github)* [**breaking**] Try inlining re-exports
- Add some text for main doc page
