### aawaz-core

A blazing fast, easy to setup, privacy friendly, highly configurable commenting system.

#### Development.

-   [Install rust 1.57+](https://www.rust-lang.org/tools/install) and [postgres 13+](https://www.postgresql.org/download/) if you don't have it already installed.

-   Fork and clone this repository.

-   Run the following commands.

    -   Install the dependencies.

    ```sh
    cargo install
    ```

    -   Create a database with a user on `postgres`.

    ```sh
    psql -U postgres -p myPassword -f"""
        CREATE DATABASE aawaz-core
    """
    ```

    -   Create .env from the given template

    ```sh
    cp .env.sample .env
    ```

    -   Fill the `.env` file appropriately.

    -   Install diesel-cli(to generate/apply migration)

    ```sh
    cargo install diesel_cli
    ```

    -   Run the migrations.

    ```sh
    diesel migrations run
    ```

    -   Run the development server.

    ```sh
    cargo run
    ```

    To run the development server in the watch mode.

    ```sh
    cargo install cargo-watch
    cargo watch -x run
    ```

-   Set up [`pre-commit`](https://pre-commit.com) by following the instructions on the website.
