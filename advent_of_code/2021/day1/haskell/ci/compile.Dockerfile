FROM haskell:9.12.2@sha256:959738aa548fcfe073058463f3e0c56f349bd627f39559006af7b290f734110e

WORKDIR /workspace
ENV HOME=/workspace

RUN cabal update

ENTRYPOINT ["cabal", "build"]
