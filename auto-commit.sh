#!/bin/bash

# Auto-commit script - rulează periodic în background
while true; do
  # Așteaptă 10 minute
  sleep 600
  
  # Check dacă există modificări
  if [[ -n $(git status -s) ]]; then
    TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
    git add .
    git commit -m "auto-save: $TIMESTAMP" --no-verify
    echo "✅ Auto-commit: $TIMESTAMP"
  fi
done
