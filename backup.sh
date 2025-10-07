#!/bin/bash
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="money-zen-checkpoint-$TIMESTAMP"
cd ..
cp -r money-zen "../money-zen-backups/$BACKUP_NAME"
echo "✅ Backup created: $BACKUP_NAME"
