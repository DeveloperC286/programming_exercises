FROM haskell:9.12.2@sha256:b85c1ab808c93d2fae116675c748dbe34f43631ffd4a31f553da8e8438b180e7

WORKDIR /workspace

RUN cabal update && cabal install ormolu

ENTRYPOINT ["ormolu", "--mode", "check"]
CMD [ "lib/Day1.hs", "src/Main.hs", "tests/Tests.hs" ]
