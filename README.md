# Rust ScyllaDB Project

This project is a Rust application that interfaces with [ScyllaDB](https://scylladb.com), 
a highly performant NoSQL database, for basic CRUD (Create, Read, Update, Delete) operations. 
It uses the `scylla` crate to interact with the database and demonstrates operations like 
adding, reading, updating, and deleting user data.

## Prerequisites

Before you start, ensure you have the following installed:

- Rust and Cargo ([Rust Installation Guide](https://www.rust-lang.org/tools/install))
- Docker and Docker Compose ([Docker Install Documentation](https://docs.docker.com/get-docker/))

## Setting Up ScyllaDB with Docker Compose

To run ScyllaDB locally, you can use Docker Compose. Here's a simple setup:

1. **Run the ScyllaDB container**:

    ```bash
    docker-compose up -d
    ```

   This command will download the ScyllaDB image and start a ScyllaDB instance.

## Running the Rust Application

To run the Rust application:

1. **Build the project**:

    ```bash
    cargo build
    ```

2. **Run the application**:

    ```bash
    cargo run
    ```

   This will start the application and perform any defined database operations.

## Environment Configuration

The application expects a `DATABASE_URL` environment variable for connecting to ScyllaDB. 
Ensure this is set in your environment, or use a `.env` file in your project root with the 
following content:

    ```
    DATABASE_URL=0.0.0.0:9042
    ```

If using a `.env` file, ensure you have the `dotenv` crate in your dependencies and it's 
initialized in your main function.


## Contributing

Contributions to this project are welcome! Please fork the repository and submit a pull 
request with your changes.

---
