#!/bin/bash

echo
echo ":::: GIT Commit:"
git log -1

echo
echo ":::: GIT Tags:"
git tag --points-at HEAD

taglistStr=$(git tag --points-at HEAD)
declare -a taglist=($taglistStr)
tags=""

for tag in "${taglist[@]}"; do
	tags="${tags} ${tag}"
done

echo
echo ":::: Set Action Output: Tags:"
echo "tags=$tags" >> $GITHUB_OUTPUT
echo $tags
echo

