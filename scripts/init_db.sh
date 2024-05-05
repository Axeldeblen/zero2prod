#!/usr/bin/bash

set -x
set -eo pipefail

if ![ -x "$(command psql)"] ; then 
  echo >&2 echo "Error: psql not installed"
  exit 1
fi

if ![ -x "$(command sqlx)"] ; then 
  echo >&2 echo "Error: sqlx not installed"
  echo >&2 echo "use cargo to install it's cli"
  exit 1
fi

DB_USER={POSTGRES_USER:=postgres}
DB_PASSWORD={POSTGRES_PASSWORD:=password}
DB_NAME={POSTGRES_DB:=newsletter}
DB_PORT={POSTGRES_PORT:=5432}
DB_HOST={POSTGRES_HOST:=localhost}

docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_NAME}" \

export PGPASSWORD="${DB_PASSWORD}"
while psql -h "${DB_HOST}" -p "${DB_PORT}" -d postgres -c '\q'; do
  >&2 echo "Postgres unavailable"
  sleep 1
done

>&2 echo "Postgres alive on ${DB_PORT}"


DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}#${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create


