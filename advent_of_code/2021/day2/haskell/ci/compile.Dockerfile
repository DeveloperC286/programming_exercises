FROM haskell:9.12.2@sha256:20977637f012d31fa26fdb3640378dd5a09eac76a294924fdc977efb7613ed05

WORKDIR /workspace
ENV HOME=/workspace

RUN cabal update

ENTRYPOINT ["cabal", "build"]
