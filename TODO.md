# TODO

## In Progress

cf. https://codeberg.org/uhppoted/uhppoted/issues/12

- [x] udp::broadcast send-to-self
- [x] decode trait
- [x] decode validate macro
- [x] config builder
- [x] codegen - just use Go
- [x] copy _remaining_ lambda to udp.rs
- [x] copy _loop_ from _tcp.rs_
- [x] CHANGELOG
- [ ] NB: io::ErrorKind::ConnectionReset for TCP/UDP sendto
- [ ] README API summary
- [ ] API.md
- [ ] docstrings

- [ ] CLI
    - [ ] clean up/restructure
    - [ ] --debug: set log level
    - (?) only log if debug

- [ ] integration tests
    - [x] broadcast
    - [x] connected UDP
    - [x] TCP
    - [ ] listen
    - [x] timeouts
    - [x] invalid SOM
    - [ ] invalid opcode
    - [ ] set TIMEOUT to 500

- [ ] codegen
    - replace args.comma with 'join'

- [ ] event types enum
- [ ] event reasons enum
- [ ] event directions enum
- [ ] door mode enum

- [x] find-controllers
- [x] get-controller
- [x] set-IPv4
- [x] get-time
- [x] set-time
- [x] get-listener
- [x] set-listener
- [x] get-door
- [x] set-door
- [x] set-door-passcodes
- [x] open-door
- [x] get-status
- [x] get-cards
- [x] get-card
- [ ] get-card-at-index
- [ ] put-card
- [ ] delete-card
- [ ] delete-all-cards
- [ ] get-event
- [ ] get-event-index
- [ ] set-event-index
- [ ] record-special-events
- [ ] get-time-profile
- [ ] set-time-profile
- [ ] clear-time-profiles
- [ ] add-task
- [ ] refresh-tasklist
- [ ] clear-tasklist
- [ ] set-pc-control
- [ ] set-interlock
- [ ] activate-keypads
- [ ] get-antipassback
- [ ] set-antipassback
- [ ] set-firstcard
- [ ] restore-default-parameters


## To Be Done

