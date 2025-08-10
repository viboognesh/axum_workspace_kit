# Axum Workspace Kit

**Axum Workspace Kit** is a backend application that provides a robust starting point for building multi-tenant, workspace-oriented applications. It includes user authentication, workspace management, role-based access control (RBAC), and more, all built on the high-performance Axum web framework.

-----

## Features

  - **User Authentication**: Secure user registration, login, and password management (forgot/reset password).
  - **Workspace Management**: Users can create, update, delete, and switch between multiple workspaces.
  - **Role-Based Access Control (RBAC)**:
      - Pre-defined "Admin" and "Manager" roles with a set of permissions.
      - Ability to create custom roles with specific permissions.
      - Permissions are enforced at the route level using middleware.
  - **User Invitations**: Invite users to a workspace using a unique invite code.
  - **Email Notifications**: Email verification, welcome emails, and password reset emails are sent to users.
  - **Database Migrations**: SQL-based migrations to set up and manage the database schema.
  - **CORS Configuration**: Pre-configured Cross-Origin Resource Sharing (CORS) for easy integration with frontend applications.
  - **Centralized Application State**: `AppState` struct to manage shared resources like the database connection pool and configuration.

-----

## Getting Started

### Prerequisites

  - [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
  - [Docker](https://www.docker.com/get-started) (for running the PostgreSQL database)
  - An SMTP server for sending emails

### Installation

1.  **Clone the repository**:

    ```bash
    git clone <repository-url>
    cd axum-workspace-kit
    ```

2.  **Set up environment variables**:

    Create a `.env` file in the root of the project and add the following environment variables:

    ```env
    DATABASE_URL=postgresql://postgres:password@localhost:5432/workspace-kit
    JWT_SECRET_KEY=your-super-secret-key
    JWT_MAXAGE=60 # in minutes
    PORT=8000
    BACKEND_BASE_URL=http://localhost:8000/api
    FRONTEND_BASE_URL=http://localhost:3000

    # Mail Configuration
    SMTP_SERVER=your-smtp-server.com
    SMTP_PORT=587
    SMTP_USERNAME=your-smtp-username
    SMTP_PASSWORD=your-smtp-password
    SMTP_FROM_ADDRESS=your-email@example.com
    MAIL_TEMPLATE_PATH=src/mail/templates
    ```

3.  **Start the database and run migrations**:

    ```bash
    ./start_db_and_migrate.sh
    ```

    This script will start a PostgreSQL container, wait for it to be ready, and then run the database migrations located in the `migrations` directory.

4.  **Run the application**:

    ```bash
    cargo run
    ```

    The server will start on the port specified in your `.env` file (e.g., `http://localhost:8000`).

-----

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── migrations
│   └── 20250729_tables.sql
├── postman_collections.json
├── src
│   ├── config
│   │   ├── config.rs
│   │   ├── mail_config.rs
│   │   └── mod.rs
│   ├── constants.rs
│   ├── database
│   │   ├── auth.rs
│   │   ├── mod.rs
│   │   ├── permissions.rs
│   │   ├── role.rs
│   │   ├── user.rs
│   │   ├── workspace.rs
│   │   └── workspace_user.rs
│   ├── dtos
│   │   ├── auth.rs
│   │   ├── mod.rs
│   │   ├── permissions.rs
│   │   ├── role.rs
│   │   ├── user.rs
│   │   ├── workspace.rs
│   │   └── workspace_user.rs
│   ├── error.rs
│   ├── handlers
│   │   ├── auth.rs
│   │   ├── mod.rs
│   │   ├── permissions.rs
│   │   ├── role.rs
│   │   ├── user.rs
│   │   ├── workspace.rs
│   │   └── workspace_user.rs
│   ├── mail
│   │   ├── mail.rs
│   │   ├── mod.rs
│   │   ├── sendmail.rs
│   │   └── templates
│   ├── main.rs
│   ├── middleware
│   │   ├── jwt_auth_middleware.rs
│   │   ├── mod.rs
│   │   └── workspace_middleware.rs
│   ├── models.rs
│   ├── routes.rs
│   └── utils
│       ├── mod.rs
│       ├── password.rs
│       └── token.rs
└── start_db_and_migrate.sh

```

-----

## API Endpoints

A Postman collection is available at `postman_collections.json` to help you get started with the API. You can import this collection into Postman to see all the available endpoints and their usage.

### Authentication

  - `POST /api/auth/register`: Register a new user.
  - `POST /api/auth/login`: Log in a user and get a JWT token.
  - `GET /api/auth/verify?token=<token>`: Verify a user's email address.
  - `POST /api/auth/forgot-password`: Send a password reset email.
  - `POST /api/auth/reset-password`: Reset a user's password.

### User

  - `GET /api/user/me`: Get the currently logged-in user's details.
  - `PUT /api/user/update-password`: Update the current user's password.
  - `PUT /api/user/change-email`: Request an email change for the current user.
  - `GET /api/user/verify-email?token=<token>`: Verify the new email address.

### Workspace

  - `POST /api/workspace/create`: Create a new workspace.
  - `PUT /api/workspace/update`: Update the current workspace.
  - `DELETE /api/workspace/delete`: Delete the current workspace.
  - `GET /api/workspace`: Get a list of all workspaces for the current user.
  - `GET /api/workspace/{workspace_id}`: Get details for a specific workspace.

### Roles and Permissions

  - `GET /api/role`: Get a list of all roles in the current workspace.
  - `POST /api/role`: Create a new role.
  - `PUT /api/role/{role_id}`: Update a role.
  - `DELETE /api/role/{role_id}`: Delete a role.
  - `GET /api/permissions`: Get a list of all available permissions.

### Workspace Users

  - `GET /api/workspace_user/invite/{invite_code}`: Join a workspace using an invite code.
  - `DELETE /api/workspace_user/remove`: Remove a user from the current workspace.
  - `GET /api/workspace_user`: Get a list of all users in the current workspace.
  - `PATCH /api/workspace_user/{user_id}`: Update a user's role in the workspace.

-----

## Dependencies

The project uses the following key dependencies:

  - **Web Framework**:
      - [`axum`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/axum%5D\(https://crates.io/crates/axum\)): A high-performance, ergonomic web framework built by the Tokio team.
  - **Database**:
      - [`sqlx`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/sqlx%5D\(https://crates.io/crates/sqlx\)): A modern, async-ready SQL toolkit for Rust.
      - [`sqlx-postgres`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/sqlx-postgres%5D\(https://crates.io/crates/sqlx-postgres\)): PostgreSQL driver for `sqlx`.
  - **Authentication & Authorization**:
      - [`jsonwebtoken`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/jsonwebtoken%5D\(https://crates.io/crates/jsonwebtoken\)): For creating and verifying JSON Web Tokens (JWTs).
      - [`argon2`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/argon2%5D\(https://crates.io/crates/argon2\)): For hashing passwords securely.
  - **Email**:
      - [`lettre`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/lettre%5D\(https://crates.io/crates/lettre\)): A modern and easy-to-use email library for Rust.
  - **Configuration**:
      - [`dotenv`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/dotenv%5D\(https://crates.io/crates/dotenv\)): For loading environment variables from a `.env` file.
  - **Serialization & Deserialization**:
      - [`serde`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/serde%5D\(https://crates.io/crates/serde\)): A framework for serializing and deserializing Rust data structures efficiently and generically.
      - [`serde_json`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/serde_json%5D\(https://crates.io/crates/serde_json\)): For working with JSON data.
  - **Validation**:
      - [`validator`](https://www.google.com/search?q=%5Bhttps://crates.io/crates/validator%5D\(https://crates.io/crates/validator\)): For validating structs and their fields.

