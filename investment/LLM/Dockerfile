FROM gcc:13.2.0 AS build
WORKDIR /app
COPY main.c .
RUN gcc -O2 -o main main.c -lm

FROM debian:stable-slim
WORKDIR /app
COPY --from=build /app/main .
CMD ["./main"]