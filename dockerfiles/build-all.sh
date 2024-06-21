#!/bin/bash
VERSION=$(cat ./package.json | jq -r '.version')
cd dockerfiles
IMAGES=$(ls -1 | grep -v build-all.sh | grep -v '^$')
for IMAGE in $IMAGES; do
    # check if docker image is already built
    VERSION=":$VERSION"
    if [[ "$(docker images -q $DOCKERIMAGE-test_$VERSION 2> /dev/null)" == "" ]]; then
        echo "Building $DOCKERIMAGE:$VERSION"
        ./$IMAGE/build.sh
    else
        echo "Image $IMAGE:$VERSION already exists"
    fi
    echo ""
done

docker images