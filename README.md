install postgres before if needed

´´´
cargo install diesel_cli --no-default-features --features postgres
cargo install diesel_cli_ext
diesel print-schema > src/schema.rs
diesel_ext > src_db_models.rs
´´´