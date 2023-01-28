#!/bin/bash
set -e

# Upgrade to a new major version of postgres by using pg_dumpall and psql
# (to be run on pi)

# run the new database in parallel on a different port by adding
# the service to docker-compose:
#  db15:
#    image: postgres:15.1
#    restart: always
#    environment:
#      POSTGRES_DB: postgres
#      POSTGRES_USER: postgres
#      POSTGRES_PASSWORD: postgres
#    ports:
#      - "127.0.0.1:5433:5432"
#    volumes:
#      - ./data/postgres15:/var/lib/postgresql/data

OLD_DB_CONTAINER=piaq_db_1
NEW_DB_CONTAINER=piaq_db15_1
BACKUP_FILE=pg_backup.dat
BACKUP_PATH=/var/lib/postgresql/data

echo "backing up old db..."
docker exec $OLD_DB_CONTAINER pg_dumpall -l postgres -U postgres -f $BACKUP_PATH/$BACKUP_FILE

echo "copying backup file to host..."
docker cp $OLD_DB_CONTAINER:$BACKUP_PATH/$BACKUP_FILE $BACKUP_FILE

echo "copying backup file to new container..."
docker cp $BACKUP_FILE $NEW_DB_CONTAINER:$BACKUP_PATH/$BACKUP_FILE

echo "restoring backup to new db..."
docker exec $NEW_DB_CONTAINER psql -d postgres -U postgres -f $BACKUP_PATH/$BACKUP_FILE

echo "cleaning up backup files..."
docker exec $OLD_DB_CONTAINER rm $BACKUP_PATH/$BACKUP_FILE
docker exec $NEW_DB_CONTAINER rm $BACKUP_PATH/$BACKUP_FILE

echo ""
echo "done!"

# switch piaq to point to new db and restart
# switch grafana datasource to new db

# spin down old db and remove from compose
