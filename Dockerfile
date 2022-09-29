FROM ubuntu:20.04

ENV TZ=Asia/Tokyo
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN echo "LC_ALL=en_US.UTF-8" >> /etc/environment
RUN echo "LANG=en_US.UTF-8" >> /etc/environment
RUN more "/etc/environment"

RUN apt-get clean
RUN apt-get update
RUN apt-get install curl htop git zip make nano ncdu build-essential chrpath libssl-dev libxft-dev pkg-config glib2.0-dev libexpat1-dev gobject-introspection python-gi-dev apt-transport-https libgirepository1.0-dev libtiff5-dev libjpeg-turbo8-dev libgsf-1-dev fail2ban nginx python3-pip python3.9-venv python3-dev ssh -y

# Install Rust
RUN mkdir -p /user/rust-builder/src
WORKDIR /user/rust-builder/src
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly

# Cleanup
RUN apt-get update && apt-get upgrade -y && apt-get autoremove -y

WORKDIR /app
COPY . .

CMD ["make", "run"]