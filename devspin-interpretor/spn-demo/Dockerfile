# Build stage
FROM node:18-alpine AS builder
WORKDIR /app

# Install dependencies (only package files first for cache)
COPY react/package*.json ./
RUN npm ci --silent

# Copy app sources and build
COPY react/ .
RUN npm run build --silent

# Production image
FROM nginx:stable-alpine
COPY --from=builder /app/build /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
