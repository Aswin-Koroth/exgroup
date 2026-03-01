use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub father_name: Option<String>,
    pub spouse_name: Option<String>,
    pub current_place: Option<String>,
    pub current_post: Option<String>,
    pub current_address: Option<String>,
    pub phone_numbers: Option<String>,
    pub permanent_same_as_current: i32,
    pub permanent_place: Option<String>,
    pub permanent_post: Option<String>,
    pub permanent_address: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_relation: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub police_station: Option<String>,
    pub experience: Option<String>,
    pub job_post: Option<String>,
    pub employment_status: String,
    pub joining_date: Option<String>,
    pub exit_date: Option<String>,
    pub essid: String,
    pub photo_path: Option<String>,
    pub date_of_birth: Option<String>,
    pub uan: Option<String>,
    pub esiip: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeInput {
    pub name: String,
    pub father_name: Option<String>,
    pub spouse_name: Option<String>,
    pub current_place: Option<String>,
    pub current_post: Option<String>,
    pub current_address: Option<String>,
    pub phone_numbers: Option<String>,
    pub permanent_same_as_current: i32,
    pub permanent_place: Option<String>,
    pub permanent_post: Option<String>,
    pub permanent_address: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_relation: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub police_station: Option<String>,
    pub experience: Option<String>,
    pub job_post: Option<String>,
    pub employment_status: String,
    pub joining_date: Option<String>,
    pub exit_date: Option<String>,
    pub essid: String,
    pub photo_path: Option<String>,
    pub date_of_birth: Option<String>,
    pub uan: Option<String>,
    pub esiip: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DbInfo {
    pub path: String,
    pub version: i32,
    pub employee_count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub query: Option<String>,
    pub post: Option<String>,
    pub job_post: Option<String>,
    pub exit_date: Option<String>,
    pub joining_date: Option<String>,
    pub employment_status: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeListResponse {
    pub employees: Vec<Employee>,
    pub total_count: i64,
}
