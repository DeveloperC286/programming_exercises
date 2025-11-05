FROM haskell:9.12.2@sha256:78c823f5c5723101cd3a50139ccc247d916d1bea7480c6aabd38eff59e28e647

WORKDIR /workspace

RUN cabal update && cabal install ormolu

ENTRYPOINT ["ormolu", "--mode", "check"]
CMD [ "lib/Day1.hs", "src/Main.hs", "tests/Tests.hs" ]
