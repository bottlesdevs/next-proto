# next-proto

Generated protobuf/gRPC bindings shared across the Bottles Next workspace.

Currently defines the `winebridge` protocol (see `proto/winebridge.proto`),
used by [`next-core`](../next-core) to communicate with the WineBridge agent
running inside a Wine prefix. Types are generated at build time via
`tonic-prost-build` (see `build.rs`) and re-exported under
`next_proto::winebridge`.
