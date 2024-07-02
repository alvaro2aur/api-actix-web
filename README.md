# Introduction 
This service manages requests for entity summary and detail views of dashboard. It integrates with other microservices to compile and structure the necessary data that feeds the user interfaces in the frontend. Its main function is to act as an intermediary, processing and organizing information from various sources to present it coherently and efficiently in the frontend.

# Getting Started
It's developed using [Actix Web](https://actix.rs/) for Rust and utilizes a PostgreSQL database.
- Dependencies are declared in the [Cargo.toml](./Cargo.toml) file.
- Environment variables are declared in the [.env.template](./.env.template) file.
- The endpoint examples are in the file [endpoints.rest](./endpoints.rest).

# Building Locally
To run the project locally using Docker:
1. Run `docker build . -t msw-summary`
2. Run `docker run -p 8080:8080 msw-summary --env-file ./.env.template`.

# Contributing
To contribute to the repository, follow these steps:
1. Clone the repository:
    ```
    git clone <repository-url>
    ```
    ```
    cd <repository-directory>
    ```
2. Create a new branch from the `main` branch:
    ```
    git switch -c <new-branch>
    ```
3. Push changes to the new branch:
   ```
   git pull origin <new-branch>
   ```
4. Create a Pull Request.
