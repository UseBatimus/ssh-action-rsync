# SSH Key Generator for GitHub Actions

This Rust project automates the generation of SSH keys for GitHub Actions or any other SSH authentication system. It generates an RSA SSH key pair, appends the public key to the `authorized_keys` file for authentication, and prints the private key so it can be added to GitHub Secrets.

## Features

- Allows the user to specify a custom SSH key name (defaults to `github-actions`).
- Automatically creates the `~/.ssh` directory if it doesn't exist.
- Uses `ssh-keygen` to generate a 4096-bit RSA key pair.
- Appends the public key to the `authorized_keys` file, allowing SSH access.
- Outputs the private key so it can be added to GitHub repository secrets.

## Getting Started

### Prerequisites

- Rust installed on your machine. If you don't have Rust installed, you can get it [here](https://www.rust-lang.org/tools/install).
- OpenSSH installed (for `ssh-keygen`). On Linux or macOS, it's typically pre-installed. On Windows, you may need to install it via optional features or through WSL.

### Installation

1. Clone this repository:

```bash
  git clone https://github.com/your-username/your-repository.git
```

2. Navigate to the project directory:

````bash
   ```bash
   cd your-repository
````

3. Build the project:

```bash
   cargo build --release
```

4. Run the project:

```bash
   cargo run --release
```

## Usage

1. **Run the program**:
   When you run the program, you'll be prompted to enter the name you want to use for your SSH key. If you leave it blank, the default will be `github-actions`.

```bash
   Enter the name you want to use for the SSH key (default: github-actions):
```

2. **SSH Key Generation**:
   The program generates a 4096-bit RSA key pair, with the private key stored in `~/.ssh/{key_name}` and the public key in `~/.ssh/{key_name}.pub`.

3. **Adding to GitHub Secrets**:
   The private key will be displayed in the terminal. You can copy this private key and add it to your GitHub repository secrets (Settings > Secrets and variables > Actions > New repository secret).

4. **SSH Authentication**:
   The public key is automatically added to `~/.ssh/authorized_keys`, allowing you to use the private key for SSH authentication.

### Example

```bash
$ cargo run --release
Enter the name you want to use for the SSH key (default: github-actions):
my-custom-key
SSH key generated successfully.
Public key added to authorized_keys.
Private key to add to GitHub Secrets:
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA4...
...
-----END RSA PRIVATE KEY-----
```

## Using it with GitHub Action

Here is an example usage in GitHub action

```yaml
name: Deployment Action

on:
  push:
    branches: [master]

  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [16.x]

    steps:
      - uses: actions/checkout@v3
      - name: Node LTS versions
        uses: msimerson/node-lts-versions@v1.1.1
      - name: Use Node.js 16
        uses: actions/setup-node@v3
      - name: Install all packages
        run: npm install
        env:
          NPM_AUTH_TOKEN: ${{ secrets.NPM_AUTH_TOKEN }}
      - name: Build
        run: npm run build
      - name: rsync deployments
        uses: burnett01/rsync-deployments@5.1
        with:
          switches: -avzr --delete
          path: ./*
          remote_path: "path/on/your/server/where/you/want/the/code/to/be/deployed/to"
          remote_host: "${{secrets.SEVER_IP_ADDRESS}}"
          remote_user: root
          remote_key: "${{ secrets.SSH_PRIVATE_KEY_STAGING }}"
```

## License

This project is licensed under the MIT License

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or improvements.
