FROM haskell:9.12.2@sha256:df562727e3f19881e3aa6476bd5dc673de8dd39ece031c7855668f4976b29e91

WORKDIR /workspace
ENV HOME=/root

RUN cabal update && cabal install --lib HUnit

ENTRYPOINT ["cabal", "test"]
