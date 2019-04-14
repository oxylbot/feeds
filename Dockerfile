FROM liuchong/rustup:nightly

COPY . .

ENV YOUTUBE_KEY=${YOUTUBE_KEY}
ENV TWITCH_KEY=${TWITCH_KEY}
ENV TWITTER_KEY=${TWITTER_KEY}

RUN apt-get upgrade && apt-get update

RUN rustup update && cargo update
RUN cargo build --release

CMD ["./target/release/feeds"]