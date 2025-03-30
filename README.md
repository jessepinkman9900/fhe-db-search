# fhe-db-search

## System Architecture

```mermaid
graph LR
    User((User)) --> Client
    Client --> Vault[(Vault)]
    Client --> Server
    Server --> Vault
    Server --> DB[(PostgreSQL)]
```

## Usage
```bash
# docker compose - vault, postgres, migrations, server, client
make run
```

```bash
# cargo run server + client & docker postgres + vault
make run-local
```
