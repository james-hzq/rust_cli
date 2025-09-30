use std::fs;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct PatientInfo {
    id: u32,
    patient_id: String,
    icu_id: String,
    is_sepsis: Option<char>,
    gender: Option<char>,
    age: Option<u8>,
    height: Option<f64>,
    weight: Option<f64>,
    first_care_unit: String,
    last_care_unit: String,
    in_time: String,
    out_time: String,
    doctor_id: String,
    is_died: char,
}

/// 处理CSV文件并将其转换为JSON格式
pub fn process_csv_to_json(input: &str, output: &str) -> Result<()>{
    // 从CSV输入文件路径，创建CSV读取器
    let mut reader = Reader::from_path(input)?;
    // 读取CSV文件内容，反序列化成结构体，存储在Vec中
    let patient_info_vec: Vec<PatientInfo> = reader.deserialize()
        .collect::<Result<Vec<_>, _>>()?;
    // 将结构体序列化成JSON
    let patient_info_json = serde_json::to_string_pretty(&patient_info_vec)?;
    // 将Json字符串写入文件
    fs::write(output, patient_info_json)?;
    Ok(())
}