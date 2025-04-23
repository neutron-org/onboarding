#!/bin/bash
DIR="$(dirname $0)"
cd $DIR
VERSION=$(cat ../../package.json | jq -r '.version')
VERSION=":$VERSION"
docker build -t ${ORG}hermes-test${VERSION} .
