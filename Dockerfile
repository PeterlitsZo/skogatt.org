# Install the base requirements.
FROM nginx:1.19

# Copy pages' static files and install its dependment.
COPY ./pages/build/ /usr/share/nginx/html/
