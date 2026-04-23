use anyhow::Result;
use serde_json::json;
use std::fs;
use std::path::Path;

use crate::runner::TestResult;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn new() -> Self {
        ReportGenerator
    }

    pub fn generate_reports(&self, results: &[TestResult], output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir)?;

        self.generate_json_report(results, output_dir)?;
        self.generate_markdown_report(results, output_dir)?;

        Ok(())
    }

    fn generate_json_report(&self, results: &[TestResult], output_dir: &Path) -> Result<()> {
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;

        let report = json!({
            "summary": {
                "total": total,
                "passed": passed,
                "failed": failed,
                "success_rate": if total > 0 { (passed as f64 / total as f64 * 100.0) as u32 } else { 0 }
            },
            "results": results,
            "timestamp": chrono::Local::now().to_rfc3339()
        });

        let json_path = output_dir.join("report.json");
        fs::write(&json_path, serde_json::to_string_pretty(&report)?)?;

        println!("JSON report written to: {}", json_path.display());

        Ok(())
    }

    fn generate_markdown_report(&self, results: &[TestResult], output_dir: &Path) -> Result<()> {
        use std::io::Write;

        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;

        // Pre-allocate buffer for efficiency
        let mut buffer = Vec::with_capacity(50 * 1024);
        let writer = &mut buffer;

        writeln!(writer, "# CTS-Gate Test Report\n")?;
        writeln!(
            writer,
            "## Summary\n\n- **Total**: {}\n- **Passed**: {} ✅\n- **Failed**: {} ❌\n",
            total, passed, failed
        )?;

        if total > 0 {
            let rate = (passed as f64 / total as f64 * 100.0) as u32;
            writeln!(writer, "- **Success Rate**: {}%\n", rate)?;
        }

        writeln!(writer, "---\n\n## Test Results\n")?;

        for (idx, result) in results.iter().enumerate() {
            let status = if result.passed { "✅ PASS" } else { "❌ FAIL" };

            writeln!(writer, "### Test {}: {} - {}\n", idx + 1, result.name, status)?;
            writeln!(writer, "**Method**: `{}`\n", result.method)?;
            writeln!(
                writer,
                "**Details**: {}\n\n",
                result.details.replace('\n', "\n> ")
            )?;

            writeln!(writer, "#### Expected\n\n```json")?;
            writer.write_all(
                serde_json::to_string_pretty(&result.expected)
                    .unwrap_or_default()
                    .as_bytes(),
            )?;
            writeln!(writer, "\n```\n")?;

            if let Some(actual) = &result.actual {
                writeln!(writer, "#### Actual\n\n```json")?;
                writer.write_all(
                    serde_json::to_string_pretty(actual)
                        .unwrap_or_default()
                        .as_bytes(),
                )?;
                writeln!(writer, "\n```\n")?;
            }

            if let Some(error) = &result.error {
                writeln!(writer, "#### Error\n\n```\n{}\n```\n", error)?;
            }

            writeln!(writer, "---\n")?;
        }

        let md_path = output_dir.join("report.md");
        fs::write(&md_path, buffer)?;

        println!("Markdown report written to: {}", md_path.display());

        Ok(())
    }
}
