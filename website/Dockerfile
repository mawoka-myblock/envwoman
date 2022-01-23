FROM node:16-alpine
WORKDIR /app
COPY . /app/
RUN apk add curl && \
    rm svelte.config.js && \
    mv svelte.docker.config.js svelte.config.js && \
    curl -f https://get.pnpm.io/v6.16.js | node - add --global pnpm \
    && pnpm install \
    && pnpm build
EXPOSE 3000
ENTRYPOINT ["node", "build"]