# Headless Chromium used by `just screenshot` to capture pages of the dev stack.
FROM mcr.microsoft.com/playwright:v1.64.1-noble

WORKDIR /opt/screenshot
COPY package.json ./
RUN npm install --omit=dev --no-audit --no-fund
