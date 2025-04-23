#!/bin/bash
DIR="$(dirname $0)"
COMMIT_HASH_OR_BRANCH="v1.2.0"
cd $DIR
docker buildx build --load --build-context app=https://github.com/skip-mev/slinky.git#${COMMIT_HASH_OR_BRANCH} -t skip-mev/slinky-e2e-oracle .
