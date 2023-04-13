FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y libpq5
RUN rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-gnu/release/blogapi /app/blogapi

CMD ["/app/blogapi", "--port", "8080"]
