use soroban_sdk::{Contract, ContractResult};

pub struct TaxCalculator;

impl Contract for TaxCalculator {
    fn execute(&self, input: &str) -> ContractResult {
        // Parse input JSON
        let input_json: serde_json::Value = serde_json::from_str(input)?;

        // Extract necessary fields from input
        let income = input_json["income"].as_f64().ok_or("Missing or invalid income")?;
        let tax_rate = input_json["tax_rate"].as_f64().ok_or("Missing or invalid tax_rate")?;

        // Calculate tax
        let tax = income * tax_rate;

        // Prepare output JSON
        let output_json = json!({
            "tax": tax,
        });

        // Return output
        Ok(output_json.to_string())
    }
}
