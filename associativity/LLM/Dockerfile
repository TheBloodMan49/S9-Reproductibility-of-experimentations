FROM gcc:13.2.0 AS build
WORKDIR /app
COPY main.c .
RUN gcc -O2 -o main main.c -lm

FROM alpine:3.18
WORKDIR /app
COPY --from=build /app/main .
CMD ["./main"]