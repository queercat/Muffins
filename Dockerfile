from rust:latest

workdir /usr/src/muffins
copy /app/ .

run cargo install --path .
