# AWS Nitro Enclaves Rust Demo

This project demonstrates how to set up a simple AWS Nitro Enclaves Trusted Execution Environment (TEE) using Rust. It includes two components: an Enclave program and a Host program. The Host communicates with the Enclave using vsock.

## 1. Prerequisites

### Launch an AWS EC2 Instance

1. Launch an EC2 instance that supports AWS Nitro Enclaves, such as an `m5` or `c5` instance type (e.g., `m5.xlarge`).
2. Select **Amazon Linux 2** as the operating system.

### Install Nitro CLI

Once the instance is running, SSH into it and install Nitro CLI:

For Amazon Linux 2:

```bash
sudo amazon-linux-extras enable aws-nitro-enclaves-cli
sudo yum install aws-nitro-enclaves-cli -y
sudo yum install aws-nitro-enclaves-cli-devel -y
```
