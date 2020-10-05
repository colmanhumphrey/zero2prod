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
