#!/usr/bin/env bash
set -x
set -eo pipefail

# check if user set, else use 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# check if password set, else use 'password'
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
# check if custom database name has been set, else default to 'newsletter'
DB_NAME=${POSTGRES_DB:=newsletter}
# check if custom post set, else default to '5432'
DB_PORT=${POSTGRES_PORT:=5432}

# Launch postgres in Docker
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    # ^ increased max connectios for testing purposes

# ping postgres until it's ready
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
