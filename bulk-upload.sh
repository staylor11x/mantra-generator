#!/bin/bash

# Bulk upload of github issue from markdown file
# Usage: sh create-issue.sh <relative-folder-path> [labels] [assignee]
# Example: sh create-issue.sh milestones\milestone-2 "enhancement" "@me"

if [ $# -lt 1 ]; then
    echo "Usage: $0 <relative-folder-path> [labels] [assignee]"
    echo ""
    echo "Examples:"
    echo "  $0 milestones\milestone-2"
    echo "  $0 milestones\milestone-2 \"enhancement,ui\""
    echo "  $0 milestones\milestone-2 \"enhancement,ui\" \"@me\""
    exit 1
fi

FOLDER="$1"
LABELS="${2:-enhancement}"
ASSIGNEE="${3:-@me}"

# Check if folder exists
if [ ! -d "$FOLDER" ]; then
    echo "Error: Folder '$FOLDER' not found"
    exit 1
fi

for file in $FOLDER/*.md; do  # change me!
    ./create-issue.sh "$file" $LABELS $ASSIGNEE 
done