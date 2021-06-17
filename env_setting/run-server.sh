#!/bin/sh

RUST_LOG=INFO nohup /usr/server/peterlits-com-server \
    > /var/server/server.log \
    2> /var/server/server.log &
