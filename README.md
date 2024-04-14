### Setup

- sudo apt install libssl-dev
- cargo run

### .env

touch .env

```
DATABASE_URL=postgres://admin:admin@localhost:5432/chaincue-real-estate-db
```

### Migrate

```
cargo install sea-orm-cli
```

```
sea-orm-cli migrate init
```

```
sea-orm-cli migrate generate "camel_case"
```

```
sea-orm-cli migrate up -u postgres://admin:admin@localhost:5432/chaincue-real-estate-db
```

```
sea-orm-cli generate entity --with-serde both -u postgres://admin:admin@localhost:5432/chaincue-real-estate-db -o entity/src
```
