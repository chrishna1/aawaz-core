### aawaz-core

A blazing fast, easy to setup, privacy friendly, highly configurable commenting system.


#### Development.

- Install `rust` if you don't have it already installed.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

- Fork, and clone this repository.

- Run the following commands.
    - Install the dependencies.
    ```sh
    cargo install
    ```

    - Create a database with a user on `postgres`.
    ```sh
    psql -U postgres -p myPassword -f"""
        CREATE DATABASE aawaz-core
    """
    ```
    - Edit the `.env` file and set the variable `DATABASE_URL`. Replace the string `krishnajha` with your password.

    - Create the tables.
    ```sh
    diesel migrations run
    ```

    - Run the development server.
    ```sh
    cargo run
    ```

- Set up [`pre-commit`](https://pre-commit.com) by following the instructions on the website.
