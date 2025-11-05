FROM haskell:9.12.2@sha256:542cddb0c21dd33525e26e1442ac996a907bd0897ca65764676776b27bb4d13a

WORKDIR /workspace

RUN cabal update && cabal install ormolu

ENTRYPOINT ["ormolu", "--mode", "check"]
CMD [ "lib/Day2.hs", "src/Main.hs", "tests/Tests.hs" ]
