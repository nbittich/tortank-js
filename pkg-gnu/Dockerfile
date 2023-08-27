FROM node:16
RUN wget https://sh.rustup.rs -O - | sh -s -- -y
RUN ln -s "${HOME}"/.cargo/bin/* /usr/local/bin
RUN apt update && apt upgrade -y && ap install build-essential
COPY . /app

WORKDIR /app
RUN npm i
CMD ["sh", "$1"]
