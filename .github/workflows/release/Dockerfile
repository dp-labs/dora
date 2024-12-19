FROM ubuntu:22.04 AS image
# Run `make dist` before this command or build the artifact in CI.
COPY dist/bin/dora /usr/local/bin/dora

RUN chmod +x /usr/local/bin/dora
RUN dora --version

ENV LANG=en_US.utf8
