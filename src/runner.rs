use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::client::GrpcClient;
use crate::comparator::Comparator;
use crate::config::TestCase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub method: String,
    pub passed: bool,
    pub expected: serde_json::Value,
    pub actual: Option<serde_json::Value>,
    pub error: Option<String>,
    pub details: String,
}

pub struct TestRunner {
    client: Arc<Mutex<GrpcClient>>,
}

impl TestRunner {
    pub fn new(client: GrpcClient) -> Self {
        TestRunner {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn run_tests(&self, test_cases: &[TestCase]) -> Result<Vec<TestResult>> {
        let mut results = Vec::with_capacity(test_cases.len());
        let cases_with_idx: Vec<_> = test_cases.iter().enumerate().collect();

        // Process tests with up to 4 in parallel
        for chunk in cases_with_idx.chunks(4) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|(_, tc)| {
                    let client = Arc::clone(&self.client);
                    async move {
                        let name = tc.name.clone();
                        info!("Running test: {}", name);
                        
                        let actual = {
                            let mut client_lock = client.lock().await;
                            client_lock.execute(tc).await
                        };

                        let comparator = Comparator;
                        let (passed, details) = match actual {
                            Ok(ref actual_value) => {
                                match comparator.compare(&tc.expected, &actual_value, &tc.validation) {
                                    Ok((is_match, comparison_details)) => (is_match, comparison_details),
                                    Err(ref e) => (false, format!("Comparison error: {}", e)),
                                }
                            }
                            Err(ref e) => {
                                if tc.validation.allow_error {
                                    (true, "Error expected and received".to_string())
                                } else {
                                    (false, format!("Unexpected error: {}", e))
                                }
                            }
                        };

                        TestResult {
                            name: tc.name.clone(),
                            method: tc.method.clone(),
                            passed,
                            expected: tc.expected.clone(),
                            actual: actual.ok(),
                            error: None,
                            details,
                        }
                    }
                })
                .collect();

            let chunk_results = futures::future::join_all(futures).await;

            for result in chunk_results {
                if result.passed {
                    info!("  ✅ PASS: {}", result.name);
                } else {
                    warn!("  ❌ FAIL: {}", result.details);
                }
                results.push(result);
            }
        }

        Ok(results)
    }

    #[allow(dead_code)]
    async fn run_single_test(&self, test_case: &TestCase) -> Result<TestResult> {
        let actual = {
            let mut client_lock = self.client.lock().await;
            client_lock.execute(test_case).await
        };
        let comparator = Comparator;

        let (passed, details) = match actual {
            Ok(ref actual_value) => {
                let (is_match, comparison_details) =
                    comparator.compare(&test_case.expected, &actual_value, &test_case.validation)?;

                (is_match, comparison_details)
            }
            Err(ref e) => {
                if test_case.validation.allow_error {
                    (true, "Error expected and received".to_string())
                } else {
                    (false, format!("Unexpected error: {}", e))
                }
            }
        };

        Ok(TestResult {
            name: test_case.name.clone(),
            method: test_case.method.clone(),
            passed,
            expected: test_case.expected.clone(),
            actual: actual.ok(),
            error: None,
            details,
        })
    }
}
