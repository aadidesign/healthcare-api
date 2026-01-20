use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow, sqlite::SqliteQueryResult};
use chrono::{DateTime, Utc, NaiveDateTime};
use std::env;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Patient {
    id: i64,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    date_of_birth: String,
    address: Option<String>,
    medical_history: Option<String>,
    blood_type: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct CreatePatient {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    date_of_birth: String,
    address: Option<String>,
    medical_history: Option<String>,
    blood_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdatePatient {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    medical_history: Option<String>,
    blood_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Appointment {
    id: i64,
    patient_id: i64,
    doctor_name: String,
    appointment_date: String,
    duration_minutes: i32,
    status: String,
    reason: String,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct CreateAppointment {
    patient_id: i64,
    doctor_name: String,
    appointment_date: String,
    duration_minutes: i32,
    reason: String,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateAppointment {
    doctor_name: Option<String>,
    appointment_date: Option<String>,
    duration_minutes: Option<i32>,
    status: Option<String>,
    reason: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Prescription {
    id: i64,
    patient_id: i64,
    medication_name: String,
    dosage: String,
    frequency: String,
    duration_days: i32,
    prescribing_doctor: String,
    instructions: Option<String>,
    issued_date: String,
    expiry_date: String,
    refills_remaining: i32,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct CreatePrescription {
    patient_id: i64,
    medication_name: String,
    dosage: String,
    frequency: String,
    duration_days: i32,
    prescribing_doctor: String,
    instructions: Option<String>,
    refills_remaining: i32,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

#[derive(Debug, Serialize)]
struct HealthCheck {
    status: String,
    service: String,
    version: String,
}

// Patient Handlers
async fn create_patient(
    pool: web::Data<SqlitePool>,
    patient: web::Json<CreatePatient>,
) -> Result<HttpResponse> {
    let now = Utc::now().to_rfc3339();
    
    let result = sqlx::query(
        "INSERT INTO patients (first_name, last_name, email, phone, date_of_birth, address, medical_history, blood_type, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&patient.first_name)
    .bind(&patient.last_name)
    .bind(&patient.email)
    .bind(&patient.phone)
    .bind(&patient.date_of_birth)
    .bind(&patient.address)
    .bind(&patient.medical_history)
    .bind(&patient.blood_type)
    .bind(&now)
    .bind(&now)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            let patient_id = result.last_insert_rowid();
            let created_patient = sqlx::query_as::<_, Patient>(
                "SELECT * FROM patients WHERE id = ?"
            )
            .bind(patient_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(created_patient),
                message: Some("Patient created successfully".to_string()),
            }))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<Patient> {
            success: false,
            data: None,
            message: Some(format!("Error creating patient: {}", e)),
        })),
    }
}

async fn get_patients(pool: web::Data<SqlitePool>) -> Result<HttpResponse> {
    let patients = sqlx::query_as::<_, Patient>("SELECT * FROM patients ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(patients),
        message: None,
    }))
}

async fn get_patient(
    pool: web::Data<SqlitePool>,
    patient_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let patient = sqlx::query_as::<_, Patient>("SELECT * FROM patients WHERE id = ?")
        .bind(patient_id.into_inner())
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

    match patient {
        Some(p) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(p),
            message: None,
        })),
        None => Ok(HttpResponse::NotFound().json(ApiResponse::<Patient> {
            success: false,
            data: None,
            message: Some("Patient not found".to_string()),
        })),
    }
}

async fn update_patient(
    pool: web::Data<SqlitePool>,
    patient_id: web::Path<i64>,
    updates: web::Json<UpdatePatient>,
) -> Result<HttpResponse> {
    let now = Utc::now().to_rfc3339();
    let id = patient_id.into_inner();

    let existing = sqlx::query_as::<_, Patient>("SELECT * FROM patients WHERE id = ?")
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

    if existing.is_none() {
        return Ok(HttpResponse::NotFound().json(ApiResponse::<Patient> {
            success: false,
            data: None,
            message: Some("Patient not found".to_string()),
        }));
    }

    let patient = existing.unwrap();

    sqlx::query(
        "UPDATE patients SET first_name = ?, last_name = ?, email = ?, phone = ?, address = ?, medical_history = ?, blood_type = ?, updated_at = ? WHERE id = ?"
    )
    .bind(updates.first_name.as_ref().unwrap_or(&patient.first_name))
    .bind(updates.last_name.as_ref().unwrap_or(&patient.last_name))
    .bind(updates.email.as_ref().unwrap_or(&patient.email))
    .bind(updates.phone.as_ref().unwrap_or(&patient.phone))
    .bind(updates.address.as_ref().or(patient.address.as_ref()))
    .bind(updates.medical_history.as_ref().or(patient.medical_history.as_ref()))
    .bind(updates.blood_type.as_ref().or(patient.blood_type.as_ref()))
    .bind(&now)
    .bind(id)
    .execute(pool.get_ref())
    .await
    .unwrap();

    let updated_patient = sqlx::query_as::<_, Patient>("SELECT * FROM patients WHERE id = ?")
        .bind(id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(updated_patient),
        message: Some("Patient updated successfully".to_string()),
    }))
}

async fn delete_patient(
    pool: web::Data<SqlitePool>,
    patient_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let result = sqlx::query("DELETE FROM patients WHERE id = ?")
        .bind(patient_id.into_inner())
        .execute(pool.get_ref())
        .await
        .unwrap();

    if result.rows_affected() > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Patient not found".to_string()),
        }))
    }
}

// Appointment Handlers
async fn create_appointment(
    pool: web::Data<SqlitePool>,
    appointment: web::Json<CreateAppointment>,
) -> Result<HttpResponse> {
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        "INSERT INTO appointments (patient_id, doctor_name, appointment_date, duration_minutes, status, reason, notes, created_at, updated_at)
         VALUES (?, ?, ?, ?, 'scheduled', ?, ?, ?, ?)"
    )
    .bind(appointment.patient_id)
    .bind(&appointment.doctor_name)
    .bind(&appointment.appointment_date)
    .bind(appointment.duration_minutes)
    .bind(&appointment.reason)
    .bind(&appointment.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            let appointment_id = result.last_insert_rowid();
            let created_appointment = sqlx::query_as::<_, Appointment>(
                "SELECT * FROM appointments WHERE id = ?"
            )
            .bind(appointment_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(created_appointment),
                message: Some("Appointment created successfully".to_string()),
            }))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<Appointment> {
            success: false,
            data: None,
            message: Some(format!("Error creating appointment: {}", e)),
        })),
    }
}

async fn get_appointments(pool: web::Data<SqlitePool>) -> Result<HttpResponse> {
    let appointments = sqlx::query_as::<_, Appointment>(
        "SELECT * FROM appointments ORDER BY appointment_date DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(appointments),
        message: None,
    }))
}

async fn get_appointment(
    pool: web::Data<SqlitePool>,
    appointment_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let appointment = sqlx::query_as::<_, Appointment>(
        "SELECT * FROM appointments WHERE id = ?"
    )
    .bind(appointment_id.into_inner())
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    match appointment {
        Some(a) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(a),
            message: None,
        })),
        None => Ok(HttpResponse::NotFound().json(ApiResponse::<Appointment> {
            success: false,
            data: None,
            message: Some("Appointment not found".to_string()),
        })),
    }
}

async fn update_appointment(
    pool: web::Data<SqlitePool>,
    appointment_id: web::Path<i64>,
    updates: web::Json<UpdateAppointment>,
) -> Result<HttpResponse> {
    let now = Utc::now().to_rfc3339();
    let id = appointment_id.into_inner();

    let existing = sqlx::query_as::<_, Appointment>(
        "SELECT * FROM appointments WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    if existing.is_none() {
        return Ok(HttpResponse::NotFound().json(ApiResponse::<Appointment> {
            success: false,
            data: None,
            message: Some("Appointment not found".to_string()),
        }));
    }

    let appointment = existing.unwrap();

    sqlx::query(
        "UPDATE appointments SET doctor_name = ?, appointment_date = ?, duration_minutes = ?, status = ?, reason = ?, notes = ?, updated_at = ? WHERE id = ?"
    )
    .bind(updates.doctor_name.as_ref().unwrap_or(&appointment.doctor_name))
    .bind(updates.appointment_date.as_ref().unwrap_or(&appointment.appointment_date))
    .bind(updates.duration_minutes.unwrap_or(appointment.duration_minutes))
    .bind(updates.status.as_ref().unwrap_or(&appointment.status))
    .bind(updates.reason.as_ref().unwrap_or(&appointment.reason))
    .bind(updates.notes.as_ref().or(appointment.notes.as_ref()))
    .bind(&now)
    .bind(id)
    .execute(pool.get_ref())
    .await
    .unwrap();

    let updated_appointment = sqlx::query_as::<_, Appointment>(
        "SELECT * FROM appointments WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(updated_appointment),
        message: Some("Appointment updated successfully".to_string()),
    }))
}

async fn delete_appointment(
    pool: web::Data<SqlitePool>,
    appointment_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let result = sqlx::query("DELETE FROM appointments WHERE id = ?")
        .bind(appointment_id.into_inner())
        .execute(pool.get_ref())
        .await
        .unwrap();

    if result.rows_affected() > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Appointment not found".to_string()),
        }))
    }
}

// Prescription Handlers
async fn create_prescription(
    pool: web::Data<SqlitePool>,
    prescription: web::Json<CreatePrescription>,
) -> Result<HttpResponse> {
    let now = Utc::now();
    let issued = now.to_rfc3339();
    let expiry = (now + chrono::Duration::days((prescription.duration_days + 90) as i64)).to_rfc3339();

    let result = sqlx::query(
        "INSERT INTO prescriptions (patient_id, medication_name, dosage, frequency, duration_days, prescribing_doctor, instructions, issued_date, expiry_date, refills_remaining, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(prescription.patient_id)
    .bind(&prescription.medication_name)
    .bind(&prescription.dosage)
    .bind(&prescription.frequency)
    .bind(prescription.duration_days)
    .bind(&prescription.prescribing_doctor)
    .bind(&prescription.instructions)
    .bind(&issued)
    .bind(&expiry)
    .bind(prescription.refills_remaining)
    .bind(&issued)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            let prescription_id = result.last_insert_rowid();
            let created_prescription = sqlx::query_as::<_, Prescription>(
                "SELECT * FROM prescriptions WHERE id = ?"
            )
            .bind(prescription_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(created_prescription),
                message: Some("Prescription created successfully".to_string()),
            }))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<Prescription> {
            success: false,
            data: None,
            message: Some(format!("Error creating prescription: {}", e)),
        })),
    }
}

async fn get_prescriptions(pool: web::Data<SqlitePool>) -> Result<HttpResponse> {
    let prescriptions = sqlx::query_as::<_, Prescription>(
        "SELECT * FROM prescriptions ORDER BY issued_date DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(prescriptions),
        message: None,
    }))
}

async fn get_prescription(
    pool: web::Data<SqlitePool>,
    prescription_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let prescription = sqlx::query_as::<_, Prescription>(
        "SELECT * FROM prescriptions WHERE id = ?"
    )
    .bind(prescription_id.into_inner())
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    match prescription {
        Some(p) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(p),
            message: None,
        })),
        None => Ok(HttpResponse::NotFound().json(ApiResponse::<Prescription> {
            success: false,
            data: None,
            message: Some("Prescription not found".to_string()),
        })),
    }
}

async fn delete_prescription(
    pool: web::Data<SqlitePool>,
    prescription_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let result = sqlx::query("DELETE FROM prescriptions WHERE id = ?")
        .bind(prescription_id.into_inner())
        .execute(pool.get_ref())
        .await
        .unwrap();

    if result.rows_affected() > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Prescription not found".to_string()),
        }))
    }
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HealthCheck {
        status: "healthy".to_string(),
        service: "Healthcare API (Rust)".to_string(),
        version: "1.0.0".to_string(),
    }))
}

async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS patients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            phone TEXT NOT NULL,
            date_of_birth TEXT NOT NULL,
            address TEXT,
            medical_history TEXT,
            blood_type TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS appointments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            doctor_name TEXT NOT NULL,
            appointment_date TEXT NOT NULL,
            duration_minutes INTEGER DEFAULT 30,
            status TEXT DEFAULT 'scheduled',
            reason TEXT NOT NULL,
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        )"
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS prescriptions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            medication_name TEXT NOT NULL,
            dosage TEXT NOT NULL,
            frequency TEXT NOT NULL,
            duration_days INTEGER NOT NULL,
            prescribing_doctor TEXT NOT NULL,
            instructions TEXT,
            issued_date TEXT NOT NULL,
            expiry_date TEXT NOT NULL,
            refills_remaining INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:healthcare.db".to_string());
    let pool = SqlitePool::connect(&database_url).await.expect("Failed to connect to database");

    init_db(&pool).await.expect("Failed to initialize database");

    println!("🚀 Server starting on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add(("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE"))
                    .add(("Access-Control-Allow-Headers", "Content-Type"))
            )
            .route("/", web::get().to(health_check))
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/patients")
                            .route("", web::post().to(create_patient))
                            .route("", web::get().to(get_patients))
                            .route("/{id}", web::get().to(get_patient))
                            .route("/{id}", web::put().to(update_patient))
                            .route("/{id}", web::delete().to(delete_patient))
                    )
                    .service(
                        web::scope("/appointments")
                            .route("", web::post().to(create_appointment))
                            .route("", web::get().to(get_appointments))
                            .route("/{id}", web::get().to(get_appointment))
                            .route("/{id}", web::put().to(update_appointment))
                            .route("/{id}", web::delete().to(delete_appointment))
                    )
                    .service(
                        web::scope("/prescriptions")
                            .route("", web::post().to(create_prescription))
                            .route("", web::get().to(get_prescriptions))
                            .route("/{id}", web::get().to(get_prescription))
                            .route("/{id}", web::delete().to(delete_prescription))
                    )
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}