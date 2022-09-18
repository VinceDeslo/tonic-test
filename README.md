# Messing around with gRPC

This repo is a playground to mess around with gRPC, clap and commands in Rust.

### Objective
Build out a gRPC service to retrieve greetings based on a provided name.
If the name of the author is sent, an insult is received instead.

### Requirements
- Rust
- Tonic (gRPC)
- Tokio (HTTP)
- Logging

### Binaries
- `server`: a Tonic gRPC server that implements a GreeterService.
- `client`: a Tonic gRPC client that pings the service with a hard coded payload.
- `cli`: a tool using clap and commands to run the other two binaries.

### Top level Make commands:
- `make run-server` : Executes the server binary directly with cargo (serving on `localhost:8080`).
- `make run-client` : Executes the client binary and hits the server with a request.
- `make curl` : Runs a `grpcurl` script to hit the the server with a request.
- `make cli-client`: Runs the CLI tool with a command to run the client.
- `make cli-server`: Runs the CLi tool with a command to run the server.

### Structure
```
tonic-test
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── README.md
├── build.rs
├── proto
│   └── ** Protocol buffer definitions loaded by tonic **
├── scripts
│   └── ** grpcurl scripts **
├── src
│   ├── cli
│   │   ├── ** CLI code using Clap and Commands **
│   ├── client
│   │   └── ** gRPC client code
│   └── server
│       └── ** gRPC server code **
└── target
    ├── ** Binary outputs and debug info **
```
