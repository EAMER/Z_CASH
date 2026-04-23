use anyhow::Result;
use serde_json::Value;

use crate::config::ValidationRules;

pub struct Comparator;

impl Comparator {
    // Comparator is a zero-sized type - no allocation needed

    pub fn compare(
        &self,
        expected: &Value,
        actual: &Value,
        rules: &ValidationRules,
    ) -> Result<(bool, String)> {
        if rules.exact_match {
            return Ok(self.exact_match(expected, actual));
        }

        // Check required fields
        for field in &rules.required_fields {
            if !self.field_exists(actual, field) {
                return Ok((
                    false,
                    format!("Required field missing: {}", field),
                ));
            }
        }

        // Check field types
        for (field, expected_type) in &rules.field_types {
            if let Some(value) = self.get_field(actual, field) {
                if !self.check_type(&value, expected_type) {
                    return Ok((
                        false,
                        format!(
                            "Field '{}' has wrong type. Expected: {}, Got: {}",
                            field,
                            expected_type,
                            self.type_of(&value)
                        ),
                    ));
                }
            }
        }

        Ok((true, "All validations passed".to_string()))
    }

    fn exact_match(&self, expected: &Value, actual: &Value) -> (bool, String) {
        if expected == actual {
            (true, "Exact match".to_string())
        } else {
            (
                false,
                format!(
                    "Mismatch.\nExpected: {}\nActual: {}",
                    serde_json::to_string_pretty(expected).unwrap_or_default(),
                    serde_json::to_string_pretty(actual).unwrap_or_default()
                ),
            )
        }
    }

    fn field_exists(&self, value: &Value, field: &str) -> bool {
        self.get_field(value, field).is_some()
    }

    fn get_field<'a>(&self, value: &'a Value, field: &str) -> Option<&'a Value> {
        if let Some(obj) = value.as_object() {
            obj.get(field)
        } else {
            None
        }
    }

    fn check_type(&self, value: &Value, expected_type: &str) -> bool {
        match expected_type {
            "string" => value.is_string(),
            "number" => value.is_number(),
            "integer" => value.is_i64() || value.is_u64(),
            "boolean" => value.is_boolean(),
            "array" => value.is_array(),
            "object" => value.is_object(),
            "null" => value.is_null(),
            _ => false,
        }
    }

    fn type_of(&self, value: &Value) -> &'static str {
        match value {
            Value::String(_) => "string",
            Value::Number(_) => "number",
            Value::Bool(_) => "boolean",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Null => "null",
        }
    }
}
