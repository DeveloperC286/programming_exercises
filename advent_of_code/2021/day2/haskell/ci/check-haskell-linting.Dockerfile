FROM haskell:9.12.2@sha256:3675303b1711812361bb1470edb1da8489cd6dde87ec98310cd0155269904cd3

WORKDIR /workspace

RUN cabal update && cabal install hlint

ENTRYPOINT ["hlint"]
CMD [ "lib/Day2.hs", "src/Main.hs", "tests/Tests.hs" ]
