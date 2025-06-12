FROM haskell:9.12.2

WORKDIR /workspace
ENV HOME=/root

RUN cabal update && cabal install --lib HUnit

ENTRYPOINT ["cabal", "test"]
