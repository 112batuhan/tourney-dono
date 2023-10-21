ARG DATABASE_URL
ARG PORT
ARG DISCORD_TOKEN
ARG WS_URL

FROM rust:latest as rust-builder
WORKDIR /usr/src/tourney-dono
COPY backend .
ARG DATABASE_URL=${DATABASE_URL}
RUN cargo build --release

FROM node:latest as nuxt-builder
WORKDIR /usr/src/tourney-dono
COPY frontend .
ARG WS_URL=${WS_URL}
RUN npm install
RUN npm run generate


# I cant be bothered with libssl.so.3 error 
# so I will go with rust instead of debian images
FROM rust:slim
WORKDIR /usr/src/tourney-dono
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=rust-builder /usr/src/tourney-dono/target/release/tourney-dono .
COPY --from=nuxt-builder /usr/src/tourney-dono/.output/public public
ENV DATABASE_URL=${DATABASE_URL}
ENV PORT=${PORT}
ENV DISCORD_TOKEN=${DISCORD_TOKEN}
ENV WS_URL=${WS_URL}
EXPOSE ${PORT}
ENTRYPOINT ["./tourney-dono"]
