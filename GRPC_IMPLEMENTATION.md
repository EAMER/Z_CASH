# gRPC Implementation Guide

## Overview

The `src/client.rs` file currently has placeholder implementations. This guide shows how to implement real gRPC calls to lightwalletd.

## Step 1: Update Cargo.toml Build Script

The `build.rs` script compiles protobuf definitions. Verify it's correct:

```rust
use tonic_build;

fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &["proto/service.proto"],
            &["proto"],
        )
        .unwrap();
}
```

This generates:
- `zcash.service::CompactBlockServiceClient`
- Request/Response types: `EmptyRequest`, `BlockID`, `BlockRange`, `CompactBlock`, `TreeState`

## Step 2: Update client.rs

Replace placeholder methods with real gRPC calls:

```rust
use tonic::transport::Channel;
use tonic::transport::Endpoint;

// Include generated code (output of build.rs)
pub mod zcash_service {
    tonic::include_proto!("zcash.service");
}

use zcash_service::compact_block_service_client::CompactBlockServiceClient;
use zcash_service::{EmptyRequest, BlockRange};

pub struct GrpcClient {
    client: CompactBlockServiceClient<Channel>,
}

impl GrpcClient {
    pub async fn connect(endpoint: &str) -> Result<Self> {
        let endpoint = endpoint
            .parse::<Endpoint>()
            .map_err(|e| anyhow!("Invalid endpoint: {}", e))?;
        
        let channel = endpoint.connect().await?;
        let client = CompactBlockServiceClient::new(channel);
        
        Ok(GrpcClient { client })
    }

    pub async fn get_latest_block(&mut self) -> Result<serde_json::Value> {
        let request = tonic::Request::new(EmptyRequest {});
        let response = self.client.get_latest_block(request).await?;
        
        let block_id = response.into_inner();
        Ok(serde_json::json!({
            "height": block_id.height,
            "hash": hex::encode(&block_id.hash)
        }))
    }

    pub async fn get_block_range(
        &mut self,
        start: u64,
        end: u64,
    ) -> Result<Vec<serde_json::Value>> {
        let request = tonic::Request::new(BlockRange { start, end });
        let mut stream = self.client.get_block_range(request).await?;
        
        let mut blocks = Vec::new();
        while let Some(block) = stream.message().await? {
            blocks.push(serde_json::json!({
                "height": block.height,
                "hash": hex::encode(&block.hash),
                "time": block.time
            }));
        }
        
        Ok(blocks)
    }

    pub async fn get_latest_tree_state(&mut self) -> Result<serde_json::Value> {
        let request = tonic::Request::new(EmptyRequest {});
        let response = self.client.get_latest_tree_state(request).await?;
        
        let tree_state = response.into_inner();
        Ok(serde_json::json!({
            "height": tree_state.height,
            "hash": tree_state.hash,
            "time": tree_state.time,
            "tree": tree_state.tree
        }))
    }
}
```

## Step 3: Add hex Dependency

Update `Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
hex = "0.4"
```

## Step 4: Update Main gRPC Handler

Update the `execute` method to use proper request parsing:

```rust
pub async fn execute(&mut self, test_case: &TestCase) -> Result<serde_json::Value> {
    match test_case.method.as_str() {
        "GetLatestBlock" => self.get_latest_block().await,
        "GetBlockRange" => {
            let start = test_case.request["start"].as_u64().unwrap_or(0);
            let end = test_case.request["end"].as_u64().unwrap_or(100);
            let blocks = self.get_block_range(start, end).await?;
            Ok(serde_json::json!(blocks))
        },
        "GetLatestTreeState" => self.get_latest_tree_state().await,
        _ => Err(anyhow!("Unknown method: {}", test_case.method)),
    }
}
```

## Step 5: Update TestRunner

The runner needs to pass `&mut self` for the client:

```rust
// In runner.rs
pub async fn run_tests(&mut self, test_cases: &[TestCase]) -> Result<Vec<TestResult>> {
    let mut results = Vec::new();
    for test_case in test_cases {
        // ...
        let actual = self.client.execute(test_case).await;
        // ...
    }
    Ok(results)
}
```

## Step 6: Test Locally

With a local lightwalletd running on port 9067:

```bash
cargo build
cargo run -- run --profile lightwalletd --suite mvp --output reports
```

## Protobuf Notes

The generated proto modules will be in:
- `target/debug/build/cts-gate-*/out/zcash.service.rs`
- Included via `tonic::include_proto!("zcash.service");`

The module path matches the `package` directive in `proto/service.proto`.

## Debugging

If proto compilation fails:
1. Check `proto/service.proto` syntax
2. Verify `build.rs` references correct paths
3. Run `cargo clean && cargo build` to force rebuild

## Expected Response Format

After implementation, GetLatestBlock should return:

```json
{
  "height": 2050000,
  "hash": "00000000abcd1234..."
}
```

BlockRange returns array of blocks, GetLatestTreeState returns similar structure to BlockID + time + tree.
