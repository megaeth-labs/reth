# Holesky on WSL2: reth (Execution) + Lighthouse (Beacon)

**Tested:** Windows 11 + WSL2 Ubuntu 24.04 (systemd enabled), Rust 1.90, Docker 28.2.2, Lighthouse v8.

## 1) Docker on WSL2

```bash
sudo apt-get update
sudo apt-get install -y docker.io
sudo systemctl enable --now docker || sudo service docker start
sudo usermod -aG docker $USER
# close all terminals → in PowerShell: wsl --shutdown → reopen Ubuntu
docker run --rm hello-world

2) Run reth (Holesky)
mkdir -p $HOME/reth-data/holesky
./target/release/reth node \
  --chain holesky \
  --http --http.addr 0.0.0.0 --http.port 8545 \
  --datadir $HOME/reth-data/holesky \
  --authrpc.addr 127.0.0.1 --authrpc.port 8551 \
  --authrpc.jwtsecret $HOME/.ethereum/jwt.hex

3) Run Lighthouse
A) Checkpoint sync (faster)
sudo docker run --rm -it --network=host \
  -v $HOME/.ethereum:/root/.ethereum \
  -v $HOME/lighthouse:/root/.lighthouse \
  sigp/lighthouse:latest lighthouse bn \
  --network holesky \
  --execution-endpoint http://127.0.0.1:8551 \
  --execution-jwt /root/.ethereum/jwt.hex \
  --checkpoint-sync-url https://holesky.checkpoint-sync.ethdevops.io
# If DNS fails: add --dns 1.1.1.1 --dns 8.8.8.8 or try stakely/attestant.

B) Genesis sync (works anywhere)
sudo docker run --rm -it --network=host \
  -v $HOME/.ethereum:/root/.ethereum \
  -v $HOME/lighthouse:/root/.lighthouse \
  sigp/lighthouse:latest lighthouse bn \
  --network holesky \
  --execution-endpoint http://127.0.0.1:8551 \
  --execution-jwt /root/.ethereum/jwt.hex \
  --allow-insecure-genesis-sync

4) Common errors & fixes

never seen beacon client (reth): start Lighthouse with Engine API as above.

permission denied /var/run/docker.sock: add user to docker group → wsl --shutdown → reopen.

DNS fails for checkpoint hosts: add --dns or use alternate checkpoint URLs.

chainspec mismatch: use a fresh --datadir when switching networks.

5) Verify
curl -s -X POST http://127.0.0.1:8545 \
 -H "Content-Type: application/json" \
 -d '{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]}'
