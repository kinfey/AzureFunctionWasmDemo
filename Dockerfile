FROM mcr.microsoft.com/azure-functions/dotnet:4-appservice 
ENV AzureWebJobsScriptRoot=/home/site/wwwroot \
    AzureFunctionsJobHost__Logging__Console__IsEnabled=true

SHELL ["/bin/bash", "-c"]


RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        git \
        ssh \
        clang \
        libssl-dev \
        glibc-source -y \
        pkg-config && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/rust
ENV CARGO_HOME=/cargo 
ENV PATH=/cargo/bin:/rust/bin:$PATH

RUN echo "(curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --no-modify-path) && rustup default stable" > /install-rust.sh && chmod 755 /install-rust.sh
RUN /install-rust.sh

RUN source "/cargo/env"
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash


COPY . /home/site/wwwroot


RUN chmod 755 /home/site/wwwroot/install_libwasmedge.sh
RUN  /home/site/wwwroot/install_libwasmedge.sh

# WORKDIR /home/site/wwwroot/
RUN source /home/.wasmedge/env
RUN cargo --version

RUN rustup default nightly

WORKDIR /home/site/wwwroot/
RUN cargo build --release 
RUN cp target/release/handler . 

WORKDIR .