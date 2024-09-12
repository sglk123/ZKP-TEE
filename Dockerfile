FROM amazonlinux:2

RUN yum install -y \
    jq \
    curl \
    && yum clean all

COPY ./target/release/enclave /enclave

ENTRYPOINT ["/enclave"]
