FROM haskell:9.12.2@sha256:e9de72e956fa39636de0a0c6d96f027332f76824674f1d05e9c746fb7194e757

WORKDIR /workspace

RUN cabal update && cabal install hlint

ENTRYPOINT ["hlint"]
CMD [ "lib/Day1.hs", "src/Main.hs", "tests/Tests.hs" ]
