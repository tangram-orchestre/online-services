FROM node:24.22.2

ARG UID=1000
ARG GID=1000

# pnpm, at the version of the packageManager field of package.json (from the
# `project` build context). `pnpm run` installs the dependencies when they are
# out of date (verifyDepsBeforeRun).
ENV COREPACK_HOME=/opt/corepack
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
COPY --from=project package.json /tmp/project/
RUN corepack enable pnpm && cd /tmp/project && corepack install \
    # Downloads its native binary on first run
    && pnpm --version

# Shared by the containers through a volume, see docker-compose.yaml.
ENV PNPM_CONFIG_STORE_DIR=/opt/pnpm-store
RUN mkdir ${PNPM_CONFIG_STORE_DIR}

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
    ${COREPACK_HOME} \
    ${PNPM_CONFIG_STORE_DIR} \
    ${NODE_MODULES_DIR} \
    ${NUXT_DIR} \
    ${OUTPUT_DIR} \
    ${CLIENT}

