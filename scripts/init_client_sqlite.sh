#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "Error: sqlx is not installed."
	echo >&2 "Use:"
	echo >&2 "    cargo install --version=0.8.6 sqlx-cli --no-default-features --features postgres,rustls,sqlite"
	echo >&2 "to install it."
	exit 1
fi

SQLITE_DB=${SQLITE_DB:=auditor-client/client.db}

MIGRATIONS_DIR=${MIGRATIONS_DIR:=auditor-client/migrations}

export DATABASE_URL=sqlite://${SQLITE_DB}
sqlx database create
sqlx migrate run --source ${MIGRATIONS_DIR}

>&2 echo "Client sqlite DB has been migrated."
