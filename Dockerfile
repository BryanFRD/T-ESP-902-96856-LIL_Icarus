# ---- Build Stage ----
FROM node:22-alpine AS builder

WORKDIR /app

COPY package.json package-lock.json* ./
RUN npm ci

COPY tsconfig.json ./
COPY next.config.ts ./
COPY postcss.config.mjs ./
COPY src ./src

RUN npm run build

# ---- Production Stage ----
FROM nginx:alpine AS runner

RUN rm -rf /usr/share/nginx/html/*

COPY --from=builder /app/out /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 3000

CMD ["nginx", "-g", "daemon off;"]
