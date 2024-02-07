#!/usr/bin/env bash
set -e
TAG=v`grep 'version:' pubspec.yaml | awk '{ print $2 }'`
echo "Releasing $TAG..."
git tag $TAG
git push
git push --tags
echo "Release Done."