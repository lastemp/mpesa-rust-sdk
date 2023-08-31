# mpesa-system

This is a RESTful Actix Web API that connects to MySQL database, it is a full working example which uses the [mpesa_rust_sdk](https://github.com/lastemp/mpesa_rust_sdk).
The Kenyan Telco "Safaricom" has provided M-Pesa API endpoints for B2C, C2B and B2B (https://developer.safaricom.co.ke/Documentation). 

The RESTful Actix Web API has below listed dependencies:
- [Actix Web](https://github.com/actix/actix-web) web framework for Rust
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client
- [MySQL](https://github.com/mysql/mysql-server) MySQL database server
- [mysql](https://github.com/blackbeam/rust-mysql-simple) MySql database driver
- [chrono](https://github.com/chronotope/chrono) provides all functionality needed to do correct operations on dates and times
- [base64](https://github.com/marshallpierce/rust-base64) Decode from Base64 format or encode into it
- [dotenv](https://github.com/dotenv-rs/dotenv) Loads environment variables from a .env file
- [mpesa_rust_sdk](https://github.com/lastemp/mpesa_rust_sdk) an sdk to seamlessly integrate with Safaricom M-Pesa Mobile Money Gateway

You'll need to have a MySQL (or compatible) server running on your machine to test this example.

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../mpesa-system
```

1. Create database, tables and stored-procedures:

   The `sql` directory contains the SQL files used for database setup:
   
   Database
   ```sh
   mysql -u root -p < sql/0_create_database.sql
   ```
   
   Tables
   ```sh
   mysql -u root -p my_mpesa < sql/tables/*.sql
   ```
   
   Stored procedures
   ```sh
   mysql -u root -p my_mpesa < sql/stored-procedures/*.sql
   ```
   
   NB: The Database tables and stored-procedures have not been uploaded!

   For each step you will be prompted for the root user's password. If there's no password set on the root use, just hit enter again.

1. Create a `.env` file in this this directory:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   MYSQL_USER=root
   MYSQL_PASSWORD=<password>
   MYSQL_HOST=127.0.0.1
   MYSQL_PORT=3306
   MYSQL_DBNAME=my_mpesa
   ```

   Update "MYSQL_USER" and "MYSQL_PASSWORD" values with the correct MySQL user/password.

1. Run the server:

   ```sh
   cargo run
   ```

1. Using a different terminal send requests to the running server. For example, using [HTTPie]:

   ```sh
   http GET :8080/registerclienturls
   ```

   See [the API documentation pages](./apis/) for more info.

[HTTPie]: https://httpie.io/cli
