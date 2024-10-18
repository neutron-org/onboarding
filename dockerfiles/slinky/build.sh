#!/bin/bash
DIR="$(dirname $0)"
COMMIT_HASH_OR_BRANCH="v1.0.12"
cd $DIR
git clone https://github.com/skip-mev/connect.git
cp ./Dockerfile ./connect
cd connect
git checkout $COMMIT_HASH_OR_BRANCH
docker buildx build --load --build-context app=https://github.com/skip-mev/slinky.git#v1.0.12 -t skip-mev/slinky-e2e-oracle .
cd ..
rm -rf ./connect
