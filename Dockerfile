FROM vibbioinfocore/rust-node-ci

USER node

# Set up front-end project
RUN mkdir -p /home/node/client
WORKDIR /home/node/client
COPY --chown=node:node client/package*.json ./
RUN npm install

# Set up back-end project
RUN rustup default stable

# Copy across front-end
COPY --chown=node:node client/. .
RUN npm run build

# Copy across back-end
RUN mkdir -p /home/node/server
WORKDIR /home/node/server
COPY --chown=node:node server/. .
RUN cargo build --release

WORKDIR /home/node

EXPOSE 80

CMD ["./server/target/release/kascreech", "0.0.0.0:80", "./client/build"]