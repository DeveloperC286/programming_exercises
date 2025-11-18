FROM haskell:9.12.2@sha256:9e362e2f9c3fdcde2d36232381c766d344aabf9f913434ecf36b0eae863248b5

WORKDIR /workspace

RUN cabal update && cabal install ormolu

ENTRYPOINT ["ormolu", "--mode", "check"]
CMD [ "lib/Day2.hs", "src/Main.hs", "tests/Tests.hs" ]
