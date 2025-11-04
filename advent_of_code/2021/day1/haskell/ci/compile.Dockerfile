FROM haskell:9.12.2@sha256:6ee35d05b3df154b38c0ebac2f7c948c41f47a39260da2236080db2c1df6ee83

WORKDIR /workspace
ENV HOME=/workspace

RUN cabal update

ENTRYPOINT ["cabal", "build"]
