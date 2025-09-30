use rust_cli::process_csv_to_json;

const TEST_CSV_FILE_PATH: &str = "assets/patient_info.csv";
const TEST_JSON_FILE_PATH: &str = "assets/output.json";

#[test]
fn test_csv_to_json_conversion() {
    process_csv_to_json(TEST_CSV_FILE_PATH, TEST_JSON_FILE_PATH).unwrap();
}