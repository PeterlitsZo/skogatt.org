# Install the base requirements.
FROM nginx:1.19

# Copy pages' static files and install its dependment.
COPY ./pages/build/ /usr/html/
COPY ./server/target/release/peterlits-com-server /usr/server/

# Set the entrypoint
COPY ./env_setting/run-server.sh /docker-entrypoint.d/
RUN chmod +x /docker-entrypoint.d/run-server.sh
COPY ./env_setting/nginx.conf /etc/nginx/
