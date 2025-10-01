FROM haskell:9.12.2@sha256:5a1f384f0737a5d175324cf49ddc0b0f027d2597ba46139166b129a59d68721a

WORKDIR /workspace

RUN cabal update && cabal install ormolu

ENTRYPOINT ["ormolu", "--mode", "check"]
CMD [ "lib/Day1.hs", "src/Main.hs", "tests/Tests.hs" ]
