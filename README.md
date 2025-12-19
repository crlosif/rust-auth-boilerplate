# Rocket Auth Boilerplate

A complete, production-ready authentication boilerplate built with Rust, Rocket.rs, and PostgreSQL. This boilerplate provides a solid foundation for building secure authentication systems.

## 🚀 Features

- ✅ **User Registration** - Email and password-based registration with validation
- ✅ **User Login** - Secure login with JWT token authentication
- ✅ **Password Reset** - Forgot password and reset functionality with secure tokens
- ✅ **JWT Authentication** - Token-based authentication with configurable expiration
- ✅ **Password Security** - Bcrypt password hashing
- ✅ **Protected Routes** - Request guard for protecting authenticated endpoints
- ✅ **CORS Support** - Configured for web application integration
- ✅ **Database Migrations** - Automatic schema creation and updates
- ✅ **Error Handling** - Consistent error response format
- ✅ **Environment Configuration** - Secure `.env` file support

## 📋 Prerequisites

- **Rust** (latest stable version) - [Install Rust](https://www.rust-lang.org/tools/install)
- **PostgreSQL** (12+) - [Install PostgreSQL](https://www.postgresql.org/download/)
- **Cargo** (comes with Rust)

## 🛠️ Setup

### 1. Clone the Repository

```bash
git clone https://github.com/crlosif/rust-auth-boilerplate.git
cd rocket-auth-boilerplate
```

### 2. Create Environment File

Copy the example environment file:

```bash
cp .env.example .env
```

### 3. Configure Environment Variables

Edit `.env` file with your configuration:

```env
# PostgreSQL Database Connection
ROCKET_DATABASE_URL=postgresql://username:password@localhost:5432/rocket_auth_db

# JWT Secret Key (use a strong random string in production!)
ROCKET_JWT_SECRET=your-secret-key-here-change-this-in-production
```

**Important:** Generate a strong random secret for `ROCKET_JWT_SECRET` in production:
```bash
openssl rand -base64 32
```

### 4. Create PostgreSQL Database

```sql
CREATE DATABASE rocket_auth_db;
```

Or using the command line:
```bash
createdb rocket_auth_db
```

### 5. Run the Application

```bash
cargo run
```

The server will start on `http://localhost:8000` by default.

## 📡 API Endpoints

### Base URL
```
http://localhost:8000
```

### 1. Register User

Register a new user account.

**Endpoint:** `POST /api/auth/register`

**Request:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Success Response (201 Created):**
```json
{
  "message": "User registered successfully",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

**Error Responses:**
- `400 Bad Request` - Invalid email format or password too short
- `409 Conflict` - User already exists
- `500 Internal Server Error` - Server error

**Example:**
```bash
curl -X POST http://localhost:8000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password123"}'
```

### 2. Login

Authenticate and receive a JWT token.

**Endpoint:** `POST /api/auth/login`

**Request:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Success Response (200 OK):**
```json
{
  "message": "Login successful",
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

**Error Responses:**
- `401 Unauthorized` - Invalid credentials
- `500 Internal Server Error` - Server error

**Example:**
```bash
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password123"}'
```

### 3. Forgot Password

Request a password reset token.

**Endpoint:** `POST /api/auth/forgot-password`

**Request:**
```json
{
  "email": "user@example.com"
}
```

**Success Response (200 OK):**
```json
{
  "message": "If the email exists, a password reset token has been sent."
}
```

**Note:** In production, the reset token should be sent via email. For development, the token is returned in the response (remove this in production!).

**Example:**
```bash
curl -X POST http://localhost:8000/api/auth/forgot-password \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com"}'
```

### 4. Reset Password

Reset password using a valid reset token.

**Endpoint:** `POST /api/auth/reset-password`

**Request:**
```json
{
  "token": "reset-token-from-email",
  "new_password": "newpassword123"
}
```

**Success Response (200 OK):**
```json
{
  "message": "Password reset successfully"
}
```

**Error Responses:**
- `400 Bad Request` - Invalid/expired token or password too short
- `500 Internal Server Error` - Server error

**Example:**
```bash
curl -X POST http://localhost:8000/api/auth/reset-password \
  -H "Content-Type: application/json" \
  -d '{"token":"your-reset-token","new_password":"newpassword123"}'
```

### 5. Protected Route Example

Access a protected endpoint using JWT token.

**Endpoint:** `GET /api/auth/me`

**Headers:**
```
Authorization: Bearer <your-jwt-token>
```

**Success Response (200 OK):**
```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

**Error Responses:**
- `401 Unauthorized` - Missing or invalid token
- `404 Not Found` - User not found
- `500 Internal Server Error` - Server error

**Example:**
```bash
curl http://localhost:8000/api/auth/me \
  -H "Authorization: Bearer YOUR_JWT_TOKEN_HERE"
```

## 🏗️ Project Structure

```
rocket-auth-boilerplate/
├── src/
│   ├── auth/
│   │   ├── guard.rs      # Authentication request guard
│   │   ├── jwt.rs        # JWT token generation/verification
│   │   └── mod.rs        # Auth module exports
│   ├── errors/
│   │   └── mod.rs        # Error handling utilities
│   ├── migrations.rs     # Database migration runner
│   ├── models/
│   │   ├── user.rs       # User model and DTOs
│   │   ├── password_reset.rs  # Password reset models
│   │   └── mod.rs        # Models module exports
│   ├── routes/
│   │   ├── auth.rs       # Authentication routes
│   │   └── mod.rs        # Routes module exports
│   └── main.rs           # Application entry point
├── migrations/           # SQL migration files (if using separate files)
├── Cargo.toml           # Rust dependencies
├── Cargo.lock            # Dependency lock file
├── Rocket.toml           # Rocket framework configuration
├── .env                  # Environment variables (not in git)
├── .env.example          # Environment variables template
├── .gitignore            # Git ignore rules
└── README.md             # This file
```

## 🔐 Security Features

### Password Security
- Passwords are hashed using **bcrypt** with default cost factor
- Passwords are never stored in plain text
- Minimum password length validation (6 characters)

### JWT Tokens
- Tokens expire after **24 hours**
- Signed with HMAC SHA-256
- Secret key stored in environment variables

### Password Reset
- Reset tokens expire after **1 hour**
- Tokens can only be used **once**
- Email enumeration prevention (always returns success)

### CORS
- Configurable CORS policy
- Supports credentials for authenticated requests
- Allows Authorization header

## 🔧 Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `ROCKET_DATABASE_URL` | PostgreSQL connection string | Yes |
| `ROCKET_JWT_SECRET` | Secret key for JWT signing | Yes |

### Database Schema

The application automatically creates the following tables:

- **users** - User accounts
  - `id` (UUID, Primary Key)
  - `email` (VARCHAR, Unique, Not Null)
  - `password_hash` (VARCHAR, Not Null)
  - `created_at` (TIMESTAMP)
  - `updated_at` (TIMESTAMP)

- **password_reset_tokens** - Password reset tokens
  - `id` (UUID, Primary Key)
  - `user_id` (UUID, Foreign Key → users.id)
  - `token` (VARCHAR, Unique, Not Null)
  - `expires_at` (TIMESTAMP, Not Null)
  - `used` (BOOLEAN, Default: false)
  - `created_at` (TIMESTAMP)

## 🧪 Testing

### Manual Testing with cURL

1. **Register a user:**
```bash
curl -X POST http://localhost:8000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"test123"}'
```

2. **Login:**
```bash
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"test123"}'
```

3. **Access protected route (replace TOKEN with actual token):**
```bash
curl http://localhost:8000/api/auth/me \
  -H "Authorization: Bearer TOKEN"
```

## 🚢 Production Deployment

### Security Checklist

- [ ] Change `ROCKET_JWT_SECRET` to a strong random string
- [ ] Use HTTPS (configure reverse proxy like Nginx)
- [ ] Restrict CORS origins to your frontend domain
- [ ] Set up proper email service for password reset
- [ ] Use environment-specific database credentials
- [ ] Enable database connection pooling
- [ ] Set up logging and monitoring
- [ ] Configure rate limiting
- [ ] Regular security updates

### Recommended Production Setup

1. **Use a reverse proxy (Nginx/Traefik)** for HTTPS
2. **Set up email service** (SMTP) for password reset emails
3. **Configure CORS** to only allow your frontend domain:
   ```rust
   .allowed_origins(AllowedOrigins::some_exact(&["https://yourdomain.com"]))
   ```
4. **Use environment-specific configs** (production, staging, development)
5. **Set up database backups** and monitoring

## 📚 Dependencies

Key dependencies used in this project:

- **rocket** (0.5.1) - Web framework
- **rocket_db_pools** (0.2.0) - Database connection pooling
- **sqlx** (0.7.4) - Async SQL toolkit
- **jsonwebtoken** (9.2) - JWT token handling
- **bcrypt** (0.15) - Password hashing
- **rocket_cors** (0.6) - CORS support
- **serde** - Serialization/deserialization
- **chrono** - Date and time handling
- **uuid** - UUID generation
- **dotenv** - Environment variable loading

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with [Rocket.rs](https://rocket.rs/)
- Database powered by [PostgreSQL](https://www.postgresql.org/)
- Inspired by modern authentication best practices

## 📞 Support

For issues, questions, or contributions, please open an issue on GitHub.

---

**Made with ❤️ using Rust**
