FROM haskell:9.12.2@sha256:9e362e2f9c3fdcde2d36232381c766d344aabf9f913434ecf36b0eae863248b5

WORKDIR /workspace
ENV HOME=/workspace

RUN cabal update

ENTRYPOINT ["cabal", "build"]
