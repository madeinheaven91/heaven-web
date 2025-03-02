# Setup

1. fill up .env like .env.example
2. in backend configure diesel cli migration path
3. run `docker compose up --build`
4. run `source .env && cd backend && diesel setup --database-url postgres://${DB_USER}:${DB_PASS}@localhost:${DB_PORT}/${DB_NAME}`
