use std::fs::OpenOptions;
use std::io::stdin;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Constant for the SSH directory path.
/// By default, this points to the `~/.ssh` directory.
const SSH_DIR: &str = "~/.ssh";

/// Path to the `authorized_keys` file where public keys are appended.
/// This file controls which SSH keys are allowed to authenticate.
const AUTHORIZED_KEYS_PATH: &str = "~/.ssh/authorized_keys";

/// Main function to handle SSH key generation and setup.
///
/// This function will:
/// 1. Ask the user for a name for the SSH key. If no name is provided,
///    it defaults to `github-actions`.
/// 2. Generate an RSA SSH keypair using the provided or default name.
/// 3. Append the generated public key to the `authorized_keys` file
///    for SSH authentication.
/// 4. Print the private key so it can be added to GitHub secrets.
///
/// # Returns
/// An `io::Result<()>` indicating whether the process completed successfully.
fn main() -> io::Result<()> {
    // Prompt user to enter the SSH key name.
    println!("Enter the name you want to use for the SSH key (default: github-actions):");
    let mut key_name = String::new();
    stdin().read_line(&mut key_name)?;

    // Remove any whitespace and set a default if the input is empty.
    let key_name = key_name.trim();
    let key_name = if key_name.is_empty() {
        "github-actions"
    } else {
        key_name
    };

    // Define paths for the private and public key files based on the key name.
    let private_key_path = format!("~/.ssh/{}", key_name);
    let public_key_path = format!("~/.ssh/{}.pub", key_name);

    // Ensure that the .ssh directory exists.
    ensure_ssh_directory_exists()?;

    // Generate SSH keypair with the given name using `ssh-keygen` command.
    generate_ssh_key(&key_name, &private_key_path)?;

    // Append the public key to `authorized_keys` for SSH authentication.
    append_public_key_to_authorized_keys(&public_key_path)?;

    // Read the private key and print it to be added to GitHub Secrets.
    let private_key = std::fs::read_to_string(shellexpand::tilde(&private_key_path).to_string())?;
    println!("Private key to add to GitHub Secrets:\n{}", private_key);

    Ok(())
}

/// Ensures that the `.ssh` directory exists.
/// If the directory doesn't exist, it is created.
///
/// # Returns
/// An `io::Result<()>` indicating success or failure.
fn ensure_ssh_directory_exists() -> io::Result<()> {
    let ssh_dir = shellexpand::tilde(SSH_DIR).to_string();
    let path = Path::new(&ssh_dir);

    // Check if the SSH directory exists, if not, create it.
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
        println!("Created directory: {}", SSH_DIR);
    }

    Ok(())
}

/// Generates an RSA SSH keypair using the given key name and saves it to the
/// specified path.
///
/// This function uses the `ssh-keygen` command to generate the key.
///
/// # Arguments
/// * `key_name` - The name of the key, used as a comment in the key.
/// * `private_key_path` - The path to store the private key.
///
/// # Returns
/// An `io::Result<()>` indicating success or failure.
fn generate_ssh_key(key_name: &str, private_key_path: &str) -> io::Result<()> {
    // Execute `ssh-keygen` to generate the SSH keypair.
    let keygen_output = Command::new("ssh-keygen")
        .arg("-t")
        .arg("rsa")
        .arg("-b")
        .arg("4096")
        .arg("-C")
        .arg(key_name) // Use user-provided key name as a comment.
        .arg("-f")
        .arg(shellexpand::tilde(private_key_path).to_string()) // Save private key.
        .arg("-N") // No passphrase.
        .arg("")
        .output()
        .expect("Failed to generate SSH key");

    if !keygen_output.status.success() {
        // Print an error if key generation fails.
        println!(
            "Error generating SSH key: {}",
            String::from_utf8_lossy(&keygen_output.stderr)
        );
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "SSH key generation failed",
        ));
    }

    println!("SSH key generated successfully.");
    Ok(())
}

/// Appends the public SSH key to the `authorized_keys` file.
///
/// This allows SSH access using the newly generated public key.
///
/// # Arguments
/// * `public_key_path` - The path to the public key.
///
/// # Returns
/// An `io::Result<()>` indicating success or failure.
fn append_public_key_to_authorized_keys(public_key_path: &str) -> io::Result<()> {
    let public_key_path = shellexpand::tilde(public_key_path).to_string();
    let authorized_keys_path = shellexpand::tilde(AUTHORIZED_KEYS_PATH).to_string();

    // Read the public key content.
    let public_key = std::fs::read_to_string(public_key_path)?;

    // Open `authorized_keys` for appending.
    let mut authorized_keys_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(authorized_keys_path)?;

    // Append the public key to the `authorized_keys` file.
    authorized_keys_file.write_all(public_key.as_bytes())?;
    println!("Public key added to authorized_keys.");

    Ok(())
}
