FROM haskell:9.12.2@sha256:5a1f384f0737a5d175324cf49ddc0b0f027d2597ba46139166b129a59d68721a

WORKDIR /workspace
ENV HOME=/root

RUN cabal update && cabal install --lib HUnit

ENTRYPOINT ["cabal", "test"]
