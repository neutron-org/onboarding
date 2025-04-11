#!/bin/bash
DIR="$(dirname $0)"
cd $DIR
VERSION=$(cat ../../package.json | jq -r '.version')
git clone https://github.com/cosmos/gaia.git -b v19.0.0
cp ./Dockerfile ./gaia
VERSION=":$VERSION"
cd gaia
go mod tidy
cd ..
docker build gaia -t ${ORG}gaia-test${VERSION}
rm -rf ./gaia