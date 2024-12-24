# Getting started

## Requirements

Please make sure you have the following tools installed on your system:

- [Docker](https://docker.com)
- [Git](https://git-scm.com)

## Running the Docker services

To get Jade up and running, please follow these steps:

- 1.) Clone this repository.
- 2.) Change directory into the repository's root.
- 3.) Set the following environment variables: 
    - `POSTGRES_PASSWORD`: The password for your PostgreSQL database.
    - `API_DOMAIN`: The domain from which your JAde's API will be running.
    - `SMTP_SERVER`: The address for SMTP services from a mail provider of your choice.
- 4.) Start the containers with the command: `docker compose up -d`.