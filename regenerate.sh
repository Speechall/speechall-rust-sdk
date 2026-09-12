#!/bin/sh
set -eu

cd "$(dirname "$0")"

spec="../../Speechall-Repositories/speechall-openapi/openapi.yaml"
generator="${OAPI_TO_RUST:-oapi-to-rust}"

if [ ! -f "$spec" ]; then
  echo "OpenAPI document not found at $spec" >&2
  exit 1
fi

"$generator" "$spec" --generate types,client --output-dir src/generated

