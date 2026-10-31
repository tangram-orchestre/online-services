FROM traefik:v3.6.7
# Baked in rather than bind-mounted from the release folder, so Traefik (which
# also serves the maintenance page) is only recreated when its config changes
COPY traefik.yml /etc/traefik/traefik.yml
COPY dynamic-conf /etc/traefik/dynamic-conf
