
## Setup

```bash
docker build -t skaarl-tf-rs .
alias skaarl-run="docker run \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/home/root/app/target \
  --gpus all -it --rm -v $PWD:/app -w /app \
  skaarl-tf-rs"
```

## Running

```bash
# Python
skaarl-run python qbot/trainer.py

# Rust (optionally, add --release)
skaarl-run cargo run --bin skaarl 
```