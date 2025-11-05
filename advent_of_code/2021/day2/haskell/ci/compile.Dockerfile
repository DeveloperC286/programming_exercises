FROM haskell:9.12.2@sha256:542cddb0c21dd33525e26e1442ac996a907bd0897ca65764676776b27bb4d13a

WORKDIR /workspace
ENV HOME=/workspace

RUN cabal update

ENTRYPOINT ["cabal", "build"]
