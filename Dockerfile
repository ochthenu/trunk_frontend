# ---------- Build Stage ----------
FROM rust:1.88-bullseye AS builder

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN cargo install --locked trunk

WORKDIR /app

# Copy project
COPY . .

# Build the frontend
RUN trunk build --release


# ---------- Runtime Stage ----------
FROM nginx:alpine

# Remove default nginx site
RUN rm -rf /usr/share/nginx/html/*

# Copy built site
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx SPA config
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
