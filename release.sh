#!/bin/sh

trunk build --release

SOURCE_FOLDER='dist/'
TARGET_FOLDER='/var/www/html'
PROD_SERVER='pi.home'
USER='root'

rsync -av $SOURCE_FOLDER $USER@$PROD_SERVER:$TARGET_FOLDER --exclude-from='.rsyncignore'
ssh $USER@$PROD_SERVER 'bash -s' < remote_script.sh