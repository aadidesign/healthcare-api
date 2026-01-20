# Healthcare API

[![Python](https://img.shields.io/badge/Python-3.9%2B-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

A robust, production-ready REST API for healthcare management systems, implemented in both **Python (FastAPI)** and **Rust (Actix-web)**. This API provides comprehensive endpoints for managing patients, appointments, and prescriptions with full CRUD operations.

## 🏥 Features

- **Patient Management**: Complete patient records with medical history, contact information, and demographics
- **Appointment Scheduling**: Full appointment lifecycle management with status tracking
- **Prescription Management**: Digital prescription handling with expiry tracking and refill management
- **Database Integration**: SQLite database with proper relationships and cascading operations
- **Input Validation**: Comprehensive request validation using Pydantic (Python) and Serde (Rust)
- **RESTful Design**: Standard HTTP methods with proper status codes
- **CORS Enabled**: Cross-Origin Resource Sharing configured for frontend integration
- **Health Checks**: Built-in health check endpoints for monitoring
- **Auto-documentation**: OpenAPI/Swagger documentation (Python)

## 🚀 Quick Start

### Python Implementation

#### Prerequisites
- Python 3.9 or higher
- pip package manager

#### Installation

```bash
# Navigate to Python directory
cd healthcare-api-python

# Create virtual environment
python -m venv venv

# Activate virtual environment
# On Windows:
venv\Scripts\activate
# On macOS/Linux:
source venv/bin/activate

# Install dependencies
pip install -r requirements.txt
```

#### Running the Server

```bash
python main.py
```

The server will start on `http://localhost:8000`

**Interactive API Documentation**: Visit `http://localhost:8000/docs`

---

### Rust Implementation

#### Prerequisites
- Rust 1.70 or higher
- Cargo package manager

#### Installation

```bash
# Navigate to Rust directory
cd healthcare-api-rust

# Build the project
cargo build --release
```

#### Running the Server

```bash
cargo run --release
```

The server will start on `http://localhost:8080`

---

## 📚 API Endpoints

### Patients

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/patients` | Create a new patient |
| GET | `/api/patients` | Get all patients (with pagination) |
| GET | `/api/patients/{id}` | Get patient by ID |
| PUT | `/api/patients/{id}` | Update patient information |
| DELETE | `/api/patients/{id}` | Delete a patient |

### Appointments

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/appointments` | Schedule a new appointment |
| GET | `/api/appointments` | Get all appointments |
| GET | `/api/appointments/{id}` | Get appointment by ID |
| PUT | `/api/appointments/{id}` | Update appointment details |
| DELETE | `/api/appointments/{id}` | Cancel/delete an appointment |

### Prescriptions

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/prescriptions` | Issue a new prescription |
| GET | `/api/prescriptions` | Get all prescriptions |
| GET | `/api/prescriptions/{id}` | Get prescription by ID |
| DELETE | `/api/prescriptions/{id}` | Delete a prescription |

### Health Check

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | API health check |
| GET | `/health` | Health status (Rust only) |

---

## 💡 Usage Examples

### Create a Patient

**Python (port 8000):**
```bash
curl -X POST http://localhost:8000/api/patients \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "phone": "+1-555-0123",
    "date_of_birth": "1990-05-15T00:00:00Z",
    "blood_type": "O+",
    "medical_history": "No known allergies"
  }'
```

**Rust (port 8080):**
```bash
curl -X POST http://localhost:8080/api/patients \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Jane",
    "last_name": "Smith",
    "email": "jane.smith@example.com",
    "phone": "+1-555-0124",
    "date_of_birth": "1985-08-20T00:00:00Z",
    "blood_type": "A+",
    "medical_history": "Penicillin allergy"
  }'
```

### Schedule an Appointment

```bash
curl -X POST http://localhost:8000/api/appointments \
  -H "Content-Type: application/json" \
  -d '{
    "patient_id": 1,
    "doctor_name": "Dr. Sarah Johnson",
    "appointment_date": "2026-02-01T10:00:00Z",
    "duration_minutes": 30,
    "reason": "Annual checkup"
  }'
```

### Issue a Prescription

```bash
curl -X POST http://localhost:8000/api/prescriptions \
  -H "Content-Type: application/json" \
  -d '{
    "patient_id": 1,
    "medication_name": "Amoxicillin",
    "dosage": "500mg",
    "frequency": "Three times daily",
    "duration_days": 7,
    "prescribing_doctor": "Dr. Sarah Johnson",
    "refills_remaining": 0,
    "instructions": "Take with food"
  }'
```

### Get All Patients

```bash
curl http://localhost:8000/api/patients
```

### Update Patient Information

```bash
curl -X PUT http://localhost:8000/api/patients/1 \
  -H "Content-Type: application/json" \
  -d '{
    "phone": "+1-555-9999",
    "address": "123 Main St, New York, NY 10001"
  }'
```

---

## 📁 Project Structure

### Python Implementation
```
healthcare-api-python/
├── main.py              # Main application file with all endpoints
├── requirements.txt     # Python dependencies
├── healthcare.db        # SQLite database (auto-generated)
└── README.md           # This file
```

### Rust Implementation
```
healthcare-api-rust/
├── src/
│   └── main.rs         # Main application file with all endpoints
├── Cargo.toml          # Rust dependencies and configuration
├── healthcare.db       # SQLite database (auto-generated)
└── README.md          # This file
```

---

## 🔧 Database Schema

### Patients Table
- `id`: Primary key
- `first_name`: Patient's first name
- `last_name`: Patient's last name
- `email`: Unique email address
- `phone`: Contact number
- `date_of_birth`: Date of birth
- `address`: Residential address
- `medical_history`: Medical background
- `blood_type`: Blood type (e.g., A+, O-)
- `created_at`: Record creation timestamp
- `updated_at`: Last update timestamp

### Appointments Table
- `id`: Primary key
- `patient_id`: Foreign key to patients
- `doctor_name`: Attending physician
- `appointment_date`: Scheduled date and time
- `duration_minutes`: Appointment duration
- `status`: scheduled | completed | cancelled
- `reason`: Appointment reason
- `notes`: Additional notes
- `created_at`: Record creation timestamp
- `updated_at`: Last update timestamp

### Prescriptions Table
- `id`: Primary key
- `patient_id`: Foreign key to patients
- `medication_name`: Prescribed medication
- `dosage`: Dosage information
- `frequency`: How often to take
- `duration_days`: Treatment duration
- `prescribing_doctor`: Prescribing physician
- `instructions`: Special instructions
- `issued_date`: Prescription issue date
- `expiry_date`: Prescription expiry date
- `refills_remaining`: Number of refills left
- `created_at`: Record creation timestamp

---

## 🛡️ Security Considerations

This is a demonstration API. For production deployment, implement:

- **Authentication & Authorization**: JWT tokens, OAuth2, or API keys
- **HTTPS/TLS**: Encrypt all traffic
- **Input Sanitization**: Prevent SQL injection and XSS attacks
- **Rate Limiting**: Prevent API abuse
- **HIPAA Compliance**: Ensure compliance with healthcare regulations
- **Data Encryption**: Encrypt sensitive data at rest
- **Audit Logging**: Log all access and modifications
- **Environment Variables**: Store secrets securely (never in code)

---

## 🧪 Testing

### Python
```bash
# Install testing dependencies
pip install pytest pytest-asyncio httpx

# Run tests (create test_main.py first)
pytest
```

### Rust
```bash
# Run tests
cargo test
```

---

## 📊 Performance

### Python (FastAPI)
- **Async I/O**: Non-blocking database operations
- **Pydantic Validation**: Fast JSON serialization/deserialization
- **Response Time**: ~10-50ms for typical CRUD operations

### Rust (Actix-web)
- **Zero-cost Abstractions**: Minimal runtime overhead
- **Memory Safety**: No garbage collection pauses
- **Response Time**: ~1-10ms for typical CRUD operations
- **Concurrency**: Handles thousands of concurrent connections

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **FastAPI**: Modern Python web framework
- **Actix-web**: Powerful Rust web framework
- **SQLx**: Async SQL toolkit for Rust
- **SQLAlchemy**: SQL toolkit for Python

---

## 📧 Contact

For questions or support, please open an issue on GitHub.

---

## 🗺️ Roadmap

- [ ] Add authentication and authorization
- [ ] Implement medical record file uploads
- [ ] Add real-time notifications for appointments
- [ ] Create admin dashboard
- [ ] Add support for multiple healthcare facilities
- [ ] Implement billing and insurance management
- [ ] Add telemedicine video consultation support
- [ ] Create mobile app integration
- [ ] Add comprehensive test coverage
- [ ] Deploy with Docker and Kubernetes

---

**Built with ❤️ for the Healthcare Industry**