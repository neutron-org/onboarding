#!/bin/bash
DIR="$(dirname $0)"
COMMIT_HASH_OR_BRANCH="v19.0.0"
cd $DIR
VERSION=$(cat ../../package.json | jq -r '.version')
VERSION=":$VERSION"
git clone https://github.com/cosmos/gaia.git -b ${COMMIT_HASH_OR_BRANCH}
docker buildx build --load --build-context app=./gaia -t ${ORG}gaia-test${VERSION} --build-arg BINARY=gaiad .
rm -rf ./gaia
