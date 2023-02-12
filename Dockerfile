# stage 0, build our app
from rust:latest

workdir /usr/src/muffins
copy /app/ .

# install everything
run cargo install --path .

# grab only the stuff we want
from debian:bullseye-slim
run apt-get update 
copy --from=0 /usr/local/cargo/bin/muffins /usr/local/bin/muffins

CMD muffins
