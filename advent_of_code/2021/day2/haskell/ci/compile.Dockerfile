FROM haskell:9.12.2@sha256:78c823f5c5723101cd3a50139ccc247d916d1bea7480c6aabd38eff59e28e647

WORKDIR /workspace
ENV HOME=/workspace

RUN cabal update

ENTRYPOINT ["cabal", "build"]
