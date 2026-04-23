use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub profile: String,
    pub endpoint: String,
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub method: String,
    pub request: serde_json::Value,
    pub expected: serde_json::Value,
    pub validation: ValidationRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    #[serde(default)]
    pub required_fields: Vec<String>,
    #[serde(default)]
    pub field_types: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub exact_match: bool,
    #[serde(default)]
    pub allow_error: bool,
}

impl Config {
    pub fn load_profile(profile_name: &str) -> Result<Self> {
        let profile_path = Path::new("profiles").join(format!("{}.json", profile_name));

        let content = fs::read_to_string(&profile_path).or_else(|_| {
            fs::read_to_string(Path::new("profiles").join(format!("{}.yaml", profile_name)))
        })?;

        let profile = if profile_path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            serde_yaml::from_str(&content)?
        } else {
            serde_json::from_str(&content)?
        };
        
        Ok(profile)
    }

    pub fn load_test_cases(&self, suite: &str) -> Result<Vec<TestCase>> {
        let cases_dir = Path::new("cases").join(suite);

        let mut cases = Vec::new();

        for entry in fs::read_dir(&cases_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()).map_or(false, |ext| {
                ext == "json" || ext == "yaml" || ext == "yml"
            }) {
                let content = fs::read_to_string(&path)?;
                let test_case: TestCase = if path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map_or(false, |ext| ext == "yaml" || ext == "yml")
                {
                    serde_yaml::from_str(&content)?
                } else {
                    serde_json::from_str(&content)?
                };

                cases.push(test_case);
            }
        }

        cases.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(cases)
    }
}
