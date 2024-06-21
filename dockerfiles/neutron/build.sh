#!/bin/bash
DIR="$(dirname $0)"
COMMIT_HASH_OR_BRANCH="main"
cd $DIR
VERSION=$(cat ../../package.json | jq -r '.version')
VERSION=":$VERSION"
git clone https://github.com/neutron-org/neutron
cd neutron
git checkout $COMMIT_HASH_OR_BRANCH
docker buildx build --load --build-context app=. -t ${ORG}neutron-test${VERSION} --build-arg BINARY=neutrond .
cd ..
rm -rf ./neutron