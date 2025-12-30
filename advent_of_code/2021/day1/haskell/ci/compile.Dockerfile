FROM haskell:9.12.2@sha256:590e9e9c918d1c7b0a87badb0a6449105732fa0b7152be87ecfc149576d09614

WORKDIR /workspace
ENV HOME=/workspace

RUN cabal update

ENTRYPOINT ["cabal", "build"]
