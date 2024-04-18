# Start from a base image with both Rust and Node.js
FROM rustlang/rust:nightly-slim as builder

# Install Node.js and npm
RUN apt-get update && apt-get install -y curl && \
    curl -sL https://deb.nodesource.com/setup_14.x | bash - && \
    apt-get install -y nodejs npm

# Copy your source code into the Docker image
WORKDIR /usr/src/app
COPY . .

# Build the Rust project
RUN cargo build --release

# Build the JavaScript project
WORKDIR /usr/src/app/frontend
RUN npm install
RUN npm run build

# Change back to the root directory
WORKDIR /usr/src/app

# Start the server
CMD ["./target/release/kwiez"]