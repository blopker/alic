#!/usr/bin/env bash

TAG=v`grep 'version:' pubspec.yaml | awk '{ print $2 }'`
echo "Releasing $TAG..."
git tag $TAG
git push --tags
echo "Release Done."