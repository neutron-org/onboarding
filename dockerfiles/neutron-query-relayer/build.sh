#!/bin/bash
DIR="$(dirname $0)"
COMMIT_HASH_OR_BRANCH="main"
cd $DIR
VERSION=$(cat ../../package.json | jq -r '.version')
VERSION=":$VERSION"
git clone https://github.com/neutron-org/neutron-query-relayer -b ${COMMIT_HASH_OR_BRANCH}
cd neutron-query-relayer
GVERSION=$(echo $(git describe --tags) | sed 's/^v//')
COMMIT=$(git log -1 --format='%H')
ldflags="-X github.com/neutron-org/neutron-query-relayer/internal/app.Version=$GVERSION -X github.com/neutron-org/neutron-query-relayer/internal/app.Commit=$COMMIT" 
docker build --build-arg LDFLAGS="$ldflags" . -t ${ORG}neutron-query-relayer-test${VERSION}
cd ..
rm -rf ./neutron-query-relayer
