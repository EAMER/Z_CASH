use anyhow::{anyhow, Result};
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};

use crate::config::TestCase;

// Include generated protobuf code from build.rs
pub mod zcash_service {
    tonic::include_proto!("cash.z.wallet.sdk.rpc");
}

use zcash_service::compact_tx_streamer_client::CompactTxStreamerClient;
use zcash_service::{BlockRange, BlockId, ChainSpec, Empty};

pub struct GrpcClient {
    client: CompactTxStreamerClient<Channel>,
}

impl GrpcClient {
    pub async fn connect(endpoint: &str) -> Result<Self> {
        let endpoint = endpoint.trim();

        let channel = if endpoint.starts_with("https://") {
            let host = endpoint
                .trim_start_matches("https://")
                .split('/')
                .next()
                .ok_or_else(|| anyhow!("Invalid HTTPS endpoint: {}", endpoint))?
                .split(':')
                .next()
                .ok_or_else(|| anyhow!("Invalid HTTPS host: {}", endpoint))?;

            Endpoint::from_shared(endpoint.to_string())
                .map_err(|e| anyhow!("Invalid endpoint '{}': {}", endpoint, e))?
                .tls_config(ClientTlsConfig::new().domain_name(host.to_string()))
                .map_err(|e| anyhow!("TLS config error for '{}': {}", endpoint, e))?
                .connect()
                .await
                .map_err(|e| anyhow!("Failed to connect to {}: {}", endpoint, e))?
        } else if endpoint.starts_with("http://") {
            Endpoint::from_shared(endpoint.to_string())
                .map_err(|e| anyhow!("Invalid endpoint '{}': {}", endpoint, e))?
                .connect()
                .await
                .map_err(|e| anyhow!("Failed to connect to {}: {}", endpoint, e))?
        } else if endpoint.ends_with(":443") {
            let full_endpoint = format!("https://{}", endpoint);
            let host = endpoint
                .split(':')
                .next()
                .ok_or_else(|| anyhow!("Invalid host: {}", endpoint))?;

            Endpoint::from_shared(full_endpoint.clone())
                .map_err(|e| anyhow!("Invalid endpoint '{}': {}", full_endpoint, e))?
                .tls_config(ClientTlsConfig::new().domain_name(host.to_string()))
                .map_err(|e| anyhow!("TLS config error for '{}': {}", full_endpoint, e))?
                .connect()
                .await
                .map_err(|e| anyhow!("Failed to connect to {}: {}", full_endpoint, e))?
        } else {
            let full_endpoint = format!("http://{}", endpoint);

            Endpoint::from_shared(full_endpoint.clone())
                .map_err(|e| anyhow!("Invalid endpoint '{}': {}", full_endpoint, e))?
                .connect()
                .await
                .map_err(|e| anyhow!("Failed to connect to {}: {}", full_endpoint, e))?
        };

        let client = CompactTxStreamerClient::new(channel);
        Ok(Self { client })
    }

    pub async fn execute(&mut self, test_case: &TestCase) -> Result<serde_json::Value> {
        match test_case.method.as_str() {
            "GetLatestBlock" => self.get_latest_block().await,
            "GetBlockRange" => {
                let req = &test_case.request;
                let mode = req["mode"].as_str().unwrap_or("explicit");

                let (start, end) = if mode == "recent" {
                    // Fetch the current tip, then walk back `depth` blocks.
                    let depth = req["depth"].as_u64().unwrap_or(1).max(1);
                    let tip = self.get_tip_height().await?;
                    let start = tip.saturating_sub(depth - 1);
                    (start, tip)
                } else {
                    // Explicit start/end supplied directly in the test config.
                    let start = req["start"].as_u64().ok_or_else(|| {
                        anyhow!("GetBlockRange: missing 'start' in request (or set mode:recent)")
                    })?;
                    let end = req["end"].as_u64().ok_or_else(|| {
                        anyhow!("GetBlockRange: missing 'end' in request (or set mode:recent)")
                    })?;
                    (start, end)
                };

                self.get_block_range(start, end).await
            }
            "GetLatestTreeState" => self.get_latest_tree_state().await,
            _ => Err(anyhow!("Unknown method: {}", test_case.method)),
        }
    }

    /// Returns the current chain tip height without exposing the full BlockId JSON.
    async fn get_tip_height(&mut self) -> Result<u64> {
        let request = tonic::Request::new(ChainSpec {});
        let response = self.client.get_latest_block(request).await?;
        Ok(response.into_inner().height)
    }

    async fn get_latest_block(&mut self) -> Result<serde_json::Value> {
        let request = tonic::Request::new(ChainSpec {});
        let response = self.client.get_latest_block(request).await?;
        let block_id = response.into_inner();

        Ok(serde_json::json!({
            "height": block_id.height,
            "hash": hex::encode(&block_id.hash)
        }))
    }

    async fn get_block_range(&mut self, start: u64, end: u64) -> Result<serde_json::Value> {
        let start_block = BlockId {
            height: start,
            hash: Vec::new(),
        };

        let end_block = BlockId {
            height: end,
            hash: Vec::new(),
        };

        let range_request = BlockRange {
            start: Some(start_block),
            end: Some(end_block),
            pool_types: Vec::new(),
        };

        let request = tonic::Request::new(range_request);
        let mut stream = self.client.get_block_range(request).await?.into_inner();

        let mut blocks = Vec::new();
        while let Some(block) = stream.message().await? {
            blocks.push(serde_json::json!({
                "height": block.height,
                "hash": hex::encode(&block.hash),
                "time": block.time
            }));
        }

        Ok(serde_json::json!(blocks))
    }

    async fn get_latest_tree_state(&mut self) -> Result<serde_json::Value> {
        let request = tonic::Request::new(Empty {});
        let response = self.client.get_latest_tree_state(request).await?;
        let tree_state = response.into_inner();

        let tree_value: Option<String> = if tree_state.sapling_tree.is_empty() {
            None
        } else {
            Some(tree_state.sapling_tree.clone())
        };

        Ok(serde_json::json!({
            "height": tree_state.height,
            "hash": tree_state.hash,
            "time": tree_state.time,
            "tree": tree_value
        }))
    }
}