FROM schemaspy/schemaspy:7.0.2

# psql, to notice schema changes.
USER root
RUN apt-get update \
    && apt-get install -y --no-install-recommends postgresql-client \
    && rm -rf /var/lib/apt/lists/*
USER java

COPY watch.sh /opt/schemaspy/watch.sh
ENTRYPOINT ["bash", "/opt/schemaspy/watch.sh"]
