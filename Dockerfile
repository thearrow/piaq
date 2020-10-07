FROM ubuntu:focal
COPY piaq_release .
CMD ["./piaq_release"]