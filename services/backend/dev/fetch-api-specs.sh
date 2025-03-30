#!/bin/usr/env bash

curl -s \
    -o /opt/openapi/private-spec-tmp.json \
    -X 'GET' 'http://127.0.0.1:3000/spec' \
    -H 'accept: application/json; charset=utf-8'

curl -s \
    -o /opt/openapi/public-spec-tmp.json \
    -X 'GET' 'http://127.0.0.1:3000/public/spec' \
    -H 'accept: application/json; charset=utf-8'


sed -i 's/naive-date/date/g' /opt/openapi/private-spec-tmp.json /opt/openapi/public-spec-tmp.json

md5sum /opt/openapi/private-spec-tmp.json /opt/openapi/public-spec-tmp.json > /opt/openapi/specs-tmp.md5

if [ ! -f /opt/openapi/specs.md5 ]; then
    touch /opt/openapi/specs.md5
fi

if ! cmp -s /opt/openapi/specs-tmp.md5 /opt/openapi/specs.md5; then
    echo "OpenAPI specs updated"
    mv /opt/openapi/private-spec-tmp.json /opt/openapi/private-spec.json
    mv /opt/openapi/public-spec-tmp.json /opt/openapi/public-spec.json
    mv /opt/openapi/specs-tmp.md5 /opt/openapi/specs.md5
else
    echo "OpenAPI specs unchanged"
    rm /opt/openapi/private-spec-tmp.json /opt/openapi/public-spec-tmp.json /opt/openapi/specs-tmp.md5
fi
