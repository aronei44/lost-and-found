# Lost & Found API

API backend untuk sistem Lost & Found menggunakan Rust, Axum, SQLx, dan MinIO.

---

## 🚀 Menjalankan Aplikasi

### 1. Jalankan dengan build biasa

```bash
cargo run
```


### 2. Jalankan dengan auto-reload

```bash
cargo install cargo-watch
```

```bash
cargo watch -x run
```

## 🧱 Migrasi Database

```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
sqlx migrate revert
```
