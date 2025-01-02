# Install SP1 (https://docs.succinct.xyz/getting-started/install.html)
curl -L https://sp1.succinct.xyz | bash
sp1up

# Install Risc Zero (https://dev.risczero.com/api/zkvm/install)
curl -L https://risczero.com/install | bash
rzup install

# Install powdrVM toolchain 
rustup toolchain install nightly-2024-09-21
