FROM node:16-alpine AS deps
# RUN apk add --update git python3 make g++ libc6-compat \
#   && rm -rf /var/cache/apk/*
WORKDIR /app

COPY package.json yarn.lock* package-lock.json* pnpm-lock.yaml* ./
RUN \
  if [ -f yarn.lock ]; then yarn --frozen-lockfile; \
  elif [ -f package-lock.json ]; then npm ci; \
  elif [ -f pnpm-lock.yaml ]; then yarn global add pnpm && pnpm i; \
  else echo "Lockfile not found." && exit 1; \
  fi


FROM node:16-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .

ARG NODE_ENV
COPY .env.${NODE_ENV:-development} .env.local

ENV NODE_ENV production
RUN yarn build

EXPOSE 3000

CMD ["yarn", "serve"]
