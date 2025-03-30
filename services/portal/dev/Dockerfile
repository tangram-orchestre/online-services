FROM node:24.4.1

ARG UID=1000
ARG GID=1000

# Install tools
RUN apt-get update && apt-get install -y inotify-tools

# Set env variables
ENV WORKDIR=/opt/backend
WORKDIR ${WORKDIR}

ENV NODE_MODULES_DIR=${WORKDIR}/node_modules
RUN mkdir ${NODE_MODULES_DIR}

ENV NUXT_DIR=${WORKDIR}/.nuxt
RUN mkdir ${NUXT_DIR}

ENV OUTPUT_DIR=${WORKDIR}/.output
RUN mkdir ${OUTPUT_DIR}

ENV CLIENT=${WORKDIR}/client
RUN mkdir ${CLIENT}

# Set permissions
RUN chown -R ${UID}:${GID} \
    ${NODE_MODULES_DIR} \
    ${NUXT_DIR} \
    ${OUTPUT_DIR} \
    ${CLIENT}

COPY entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
