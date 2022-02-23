#!/bin/sh


cd /var/www/
chown -R www-data:www-data *
systemctl restart nginx