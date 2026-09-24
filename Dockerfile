FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fL \
    https://github.com/ronilan/rewordle/releases/latest/download/rewordle-terminal-linux.zip \
    -o /tmp/rewordle.zip \
    && unzip -o /tmp/rewordle.zip -d /usr/local/bin \
    && rm /tmp/rewordle.zip \
    && chmod +x /usr/local/bin/rewordle

CMD ["/bin/bash", "-i"]