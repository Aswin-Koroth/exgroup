use std::path::Path;

use rusqlite::params;

use crate::files::delete_image;

use super::types::Employee;

pub fn get_employee_by_id(
    conn: &rusqlite::Connection,
    id: i64,
) -> Result<Option<Employee>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM employees WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    match stmt.query_row([id], |row| {
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
    }) {
        Ok(employee) => Ok(Some(employee)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_employee_by_essid(
    conn: &rusqlite::Connection,
    essid: &str,
) -> Result<Option<Employee>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM employees WHERE essid = ?1")
        .map_err(|e| e.to_string())?;

    match stmt.query_row([essid], |row| {
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
    }) {
        Ok(employee) => Ok(Some(employee)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn remove_employee_photo(conn: &rusqlite::Connection, employee_id: i64) -> Result<(), String> {
    let employee = get_employee_by_id(conn, employee_id)?.ok_or("Employee not found")?;

    let photo_path = match employee.photo_path {
        Some(path) => path,
        None => return Ok(()),
    };

    conn.execute(
        "UPDATE employees SET
               photo_path = NULL,
               updated_at = CURRENT_TIMESTAMP
            WHERE id = ?",
        params![employee_id],
    )
    .map_err(|e| e.to_string())?;

    delete_image(Path::new(&photo_path))?;

    Ok(())
}
