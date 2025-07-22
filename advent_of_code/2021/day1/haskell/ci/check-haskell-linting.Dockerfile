FROM haskell:9.12.2@sha256:ef8aa7c44fc547bb0ad2fdfb35bf3110fd742cb36a157b79da821fe24022ff4b

WORKDIR /workspace

RUN cabal update && cabal install hlint

ENTRYPOINT ["hlint"]
CMD [ "lib/Day1.hs", "src/Main.hs", "tests/Tests.hs" ]
