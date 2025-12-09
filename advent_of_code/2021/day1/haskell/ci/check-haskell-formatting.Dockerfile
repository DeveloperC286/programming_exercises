FROM haskell:9.12.2@sha256:1cb322f5d74f6a3d9462d04fe5c35e3f712eaf63605a8f8e042e7764cfbf6928

WORKDIR /workspace

RUN cabal update && cabal install ormolu

ENTRYPOINT ["ormolu", "--mode", "check"]
CMD [ "lib/Day1.hs", "src/Main.hs", "tests/Tests.hs" ]
