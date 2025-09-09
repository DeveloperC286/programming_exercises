FROM haskell:9.12.2@sha256:b85c1ab808c93d2fae116675c748dbe34f43631ffd4a31f553da8e8438b180e7

WORKDIR /workspace
ENV HOME=/root

RUN cabal update && cabal install --lib HUnit

ENTRYPOINT ["cabal", "test"]
