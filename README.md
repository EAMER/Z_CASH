# CTS-Gate: Zcash Backend Conformance Testing

A command-line tool for testing Zcash blockchain backend implementations (lightwalletd, etc.) using gRPC.

## MVP Overview

CTS-Gate validates that a Zcash backend correctly implements the expected gRPC API contract. It:

- ✅ Connects to a blockchain backend via gRPC
- ✅ Loads structured test cases (JSON/YAML)
- ✅ Executes tests and validates responses
- ✅ Generates machine-readable (JSON) and human-readable (Markdown) reports
- ✅ Returns proper exit codes for CI/CD integration

## Project Structure

```
cts-gate/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── client.rs         # gRPC client implementation
│   ├── config.rs         # Profile & test case loading
│   ├── runner.rs         # Test execution logic
│   ├── comparator.rs     # Response validation
│   └── report.rs         # Report generation
├── proto/
│   └── service.proto     # Protocol Buffer definitions
├── cases/
│   └── mvp/
│       ├── 01_get_latest_block.json
│       ├── 02_get_block_range.json
│       ├── 03_get_latest_tree_state.json
│       ├── 04_get_block_range_invalid.json
│       └── 05_get_latest_block_structure.json
├── profiles/
│   └── lightwalletd.json # Backend configuration
├── reports/              # Generated test reports
└── build.rs             # Protocol Buffer compilation
```

## Building

Prerequisites: Rust 1.70+, Protocol Buffers compiler

```bash
cargo build --release
```

## Running Tests

```bash
cargo run -- run --profile lightwalletd --suite mvp --output reports
```

Or in release mode:

```bash
./target/release/cts-gate run --profile lightwalletd --suite mvp --output reports
```

## MVP Supported Methods

1. **GetLatestBlock** - Fetch the latest block header
2. **GetBlockRange** - Stream blocks in a range
3. **GetLatestTreeState** - Get the latest state tree
4. **Error Cases** - Validate error handling

## Test Cases

Each test case is a JSON/YAML file containing:

```json
{
  "name": "Test Name",
  "method": "GetLatestBlock",
  "request": { /* request parameters */ },
  "expected": { /* expected response structure */ },
  "validation": {
    "required_fields": ["field1", "field2"],
    "field_types": { "field1": "string", "field2": "number" },
    "exact_match": false,
    "allow_error": false
  }
}
```

## Output Reports

### JSON Report (`report.json`)

```json
{
  "summary": {
    "total": 5,
    "passed": 4,
    "failed": 1,
    "success_rate": 80
  },
  "results": [ /* detailed results */ ],
  "timestamp": "2024-04-16T10:30:00+00:00"
}
```

### Markdown Report (`report.md`)

Human-readable report with:
- Summary statistics
- Per-test details
- Expected vs Actual values
- Error messages

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## Technology Stack

- **Language**: Rust
- **gRPC**: `tonic`
- **Protocol Buffers**: `prost`
- **CLI**: `clap`
- **Serialization**: `serde_json`, `serde_yaml`

## Development

### Adding a New Backend Profile

Create a new file in `profiles/` named `{backend_name}.json`:

```json
{
  "profile": "my-backend",
  "backend": "my-backend",
  "endpoint": "http://localhost:9067"
}
```

### Adding a New Test Case

Create a new file in `cases/mvp/` (or new suite directory) with the test case JSON structure.

### Extending Supported Methods

1. Add the gRPC method to `proto/service.proto`
2. Implement the client method in `src/client.rs`
3. Create test cases in `cases/mvp/`

## CI/CD Integration

The tool integrates with GitHub Actions for automated testing. See `.github/workflows/test.yml`.

## License

TBD
"# Z_CASH" 
