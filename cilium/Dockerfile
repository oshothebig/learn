FROM ubuntu:22.04

RUN apt update && \
    apt install -y bird

COPY bird.conf /etc/bird/bird.conf
RUN mkdir /run/bird

CMD ["bird", "-d"]
