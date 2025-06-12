FROM haskell:9.12.2
ENV HOME=/workspace
WORKDIR /workspace
COPY . .
RUN cabal update && cabal build --enable-tests
CMD cabal update && cabal test --enable-tests
