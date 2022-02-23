FROM thienpow/wwwloader:latest

COPY ./dist /www

COPY ./start.sh /start
RUN chmod u+x /start

CMD ["/start"]