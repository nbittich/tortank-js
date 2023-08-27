#!/bin/sh
TEMP_IMAGE_NAME=temp-tortank-js-target-musl
TEMP_CONTAINER_NAME=tortank-js-target-musl
set -x
rm index.node

PROJECT_PATH=$(pwd)
cd ..
docker build -t $TEMP_IMAGE_NAME -f $PROJECT_PATH/Dockerfile .
cd -
docker run  --rm --detach --name $TEMP_CONTAINER_NAME -v $(pwd):/out  $TEMP_IMAGE_NAME  cp index.node /out
