use crate::db;
use crate::state::AppState;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};
use tauri::State;

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
    pub essid: Option<String>,
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
    pub essid: Option<String>,
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

#[tauri::command]
pub fn get_all_employees(
    _state: State<AppState>,
    page: Option<u32>,
    limit: Option<u32>,
    filter: Filter,
) -> Result<EmployeeListResponse, String> {
    let conn = db::get_connection()?;
    let mut query = String::from("SELECT * FROM employees WHERE 1=1");
    if let Some(search_query) = &filter.query {
        println!("Search Query: {}", search_query);
        if !search_query.trim().is_empty() {
            query.push_str(&format!(
                " AND name LIKE '%{}%' OR essid LIKE '%{}%'",
                search_query, search_query
            ));
        }
    }
    if let Some(job_post) = &filter.job_post {
        if !job_post.trim().is_empty() {
            query.push_str(&format!(" AND job_post LIKE '%{}%'", job_post));
        }
    }
    if let Some(employment_status) = &filter.employment_status {
        if !employment_status.trim().is_empty() {
            query.push_str(&format!(" AND employment_status = '{}'", employment_status));
        }
    }
    if let Some(joining_date) = &filter.joining_date {
        if !joining_date.trim().is_empty() {
            query.push_str(&format!(" AND joining_date = '{}'", joining_date));
        }
    }
    if let Some(exit_date) = &filter.exit_date {
        if !exit_date.trim().is_empty() {
            query.push_str(&format!(" AND exit_date = '{}'", exit_date));
        }
    }
    if let Some(post) = &filter.post {
        if !post.trim().is_empty() {
            query.push_str(&format!(" AND permanent_post LIKE '%{}%'", post));
        }
    }

    query.push_str(" ORDER BY created_at DESC");

    let page = page.unwrap_or(1).max(1);
    let limit = limit.unwrap_or(10);

    let offset = (page - 1) * limit;
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    println!("Query: {}", query);
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;

    let employees = stmt
        .query_map([], |row| {
            Ok(Employee {
                id: row.get(0)?,
                name: row.get(1)?,
                father_name: row.get(2)?,
                spouse_name: row.get(3)?,
                current_place: row.get(4)?,
                current_post: row.get(5)?,
                current_address: row.get(6)?,
                phone_numbers: row.get(7)?,
                permanent_same_as_current: row.get(8)?,
                permanent_place: row.get(9)?,
                permanent_post: row.get(10)?,
                permanent_address: row.get(11)?,
                emergency_contact_name: row.get(12)?,
                emergency_contact_relation: row.get(13)?,
                emergency_contact_phone: row.get(14)?,
                police_station: row.get(15)?,
                experience: row.get(16)?,
                job_post: row.get(17)?,
                employment_status: row.get(18)?,
                joining_date: row.get(19)?,
                exit_date: row.get(20)?,
                essid: row.get(21)?,
                photo_path: row.get(22)?,
                date_of_birth: row.get(23)?,
                uan: row.get(24)?,
                esiip: row.get(25)?,
                created_at: row.get(26)?,
                updated_at: row.get(27)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let total_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM employees", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    Ok(EmployeeListResponse {
        employees,
        total_count,
    })
}

#[tauri::command]
pub fn create_employee(
    _state: State<AppState>,
    employee: EmployeeInput,
) -> Result<Employee, String> {
    let conn = db::get_connection()?;

    conn.execute(
        "INSERT INTO employees (
            name, father_name, spouse_name, current_place, current_post, current_address,
            phone_numbers, permanent_same_as_current, permanent_place, permanent_post,
            permanent_address, emergency_contact_name, emergency_contact_relation,
            emergency_contact_phone, police_station, experience, job_post, employment_status,
            joining_date, exit_date, essid, photo_path, date_of_birth, uan, esiip
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25)",
        params![
            employee.name,
            employee.father_name,
            employee.spouse_name,
            employee.current_place,
            employee.current_post,
            employee.current_address,
            employee.phone_numbers,
            employee.permanent_same_as_current,
            employee.permanent_place,
            employee.permanent_post,
            employee.permanent_address,
            employee.emergency_contact_name,
            employee.emergency_contact_relation,
            employee.emergency_contact_phone,
            employee.police_station,
            employee.experience,
            employee.job_post,
            employee.employment_status,
            employee.joining_date,
            employee.exit_date,
            employee.essid,
            employee.photo_path,
            employee.date_of_birth,
            employee.uan,
            employee.esiip,
        ],
    ).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    get_employee_by_id(&conn, id)
}

#[tauri::command]
pub fn update_employee(
    _state: State<AppState>,
    id: i64,
    employee: EmployeeInput,
) -> Result<Employee, String> {
    let conn = db::get_connection()?;

    conn.execute(
        "UPDATE employees SET
            name = ?1, father_name = ?2, spouse_name = ?3, current_place = ?4,
            current_post = ?5, current_address = ?6, phone_numbers = ?7,
            permanent_same_as_current = ?8, permanent_place = ?9, permanent_post = ?10,
            permanent_address = ?11, emergency_contact_name = ?12, emergency_contact_relation = ?13,
            emergency_contact_phone = ?14, police_station = ?15, experience = ?16,
            job_post = ?17, employment_status = ?18, joining_date = ?19, exit_date = ?20,
            essid = ?21, photo_path = ?22, date_of_birth = ?23, uan = ?24, esiip = ?25,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?26",
        params![
            employee.name,
            employee.father_name,
            employee.spouse_name,
            employee.current_place,
            employee.current_post,
            employee.current_address,
            employee.phone_numbers,
            employee.permanent_same_as_current,
            employee.permanent_place,
            employee.permanent_post,
            employee.permanent_address,
            employee.emergency_contact_name,
            employee.emergency_contact_relation,
            employee.emergency_contact_phone,
            employee.police_station,
            employee.experience,
            employee.job_post,
            employee.employment_status,
            employee.joining_date,
            employee.exit_date,
            employee.essid,
            employee.photo_path,
            employee.date_of_birth,
            employee.uan,
            employee.esiip,
            id,
        ],
    )
    .map_err(|e| e.to_string())?;

    get_employee_by_id(&conn, id)
}

#[tauri::command]
pub fn delete_employee(_state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = db::get_connection()?;

    conn.execute("DELETE FROM employees WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_db_info(_state: State<AppState>) -> Result<DbInfo, String> {
    let conn = db::get_connection()?;
    let db_path = db::get_db_path()?;
    let version = db::get_db_version(&conn)?;

    let employee_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM employees", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    Ok(DbInfo {
        path: db_path.to_string_lossy().to_string(),
        version,
        employee_count,
    })
}

// Helper function
fn get_employee_by_id(conn: &rusqlite::Connection, id: i64) -> Result<Employee, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM employees WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    stmt.query_row([id], |row| {
        Ok(Employee {
            id: row.get(0)?,
            name: row.get(1)?,
            father_name: row.get(2)?,
            spouse_name: row.get(3)?,
            current_place: row.get(4)?,
            current_post: row.get(5)?,
            current_address: row.get(6)?,
            phone_numbers: row.get(7)?,
            permanent_same_as_current: row.get(8)?,
            permanent_place: row.get(9)?,
            permanent_post: row.get(10)?,
            permanent_address: row.get(11)?,
            emergency_contact_name: row.get(12)?,
            emergency_contact_relation: row.get(13)?,
            emergency_contact_phone: row.get(14)?,
            police_station: row.get(15)?,
            experience: row.get(16)?,
            job_post: row.get(17)?,
            employment_status: row.get(18)?,
            joining_date: row.get(19)?,
            exit_date: row.get(20)?,
            essid: row.get(21)?,
            photo_path: row.get(22)?,
            date_of_birth: row.get(23)?,
            uan: row.get(24)?,
            esiip: row.get(25)?,
            created_at: row.get(26)?,
            updated_at: row.get(27)?,
        })
    })
    .map_err(|e| e.to_string())
}

use crate::db::backup;

#[tauri::command]
pub fn create_database_backup(_state: State<AppState>) -> Result<String, String> {
    let conn = db::get_connection()?;
    let db_path = db::get_db_path()?;

    let backup_dir = db_path
        .parent()
        .ok_or_else(|| "Failed to get database directory".to_string())?
        .join("backups");

    let backup_path = backup::create_backup(&conn, &backup_dir)?;

    // Clean old backups (keep last 10)
    backup::clean_old_backups(&backup_dir, 10)?;

    Ok(backup_path.to_string_lossy().to_string())
}
