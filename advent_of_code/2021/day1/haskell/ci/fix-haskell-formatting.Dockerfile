FROM haskell:9.12.2@sha256:959738aa548fcfe073058463f3e0c56f349bd627f39559006af7b290f734110e

WORKDIR /workspace

RUN cabal update && cabal install ormolu

ENTRYPOINT ["ormolu", "--mode", "inplace"]
CMD [ "lib/Day1.hs", "src/Main.hs", "tests/Tests.hs" ]
