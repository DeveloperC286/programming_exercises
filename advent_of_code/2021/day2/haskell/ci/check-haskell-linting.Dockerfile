FROM haskell:9.12.2@sha256:99a2fe227bd90711c1c72e1bdfde4725d1bea48c9867d8ecd1203cb4d6d7b344

WORKDIR /workspace

RUN cabal update && cabal install hlint

ENTRYPOINT ["hlint"]
CMD [ "lib/Day2.hs", "src/Main.hs", "tests/Tests.hs" ]
