#!/bin/bash

for file in milestones/milestone-1-foundation/*.md; do
    ./create-issue.sh "$file"
done