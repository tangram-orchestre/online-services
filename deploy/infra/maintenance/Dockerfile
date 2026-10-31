FROM docker.io/library/nginx:1.27.5-alpine
# Baked in rather than bind-mounted from the release folder, so the container
# keeps serving during deploys and is only recreated when the page changes
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY maintenance.html /usr/share/nginx/html/maintenance.html
