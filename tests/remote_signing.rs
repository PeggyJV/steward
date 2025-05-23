#[cfg(test)]
mod tests {
    use ethers::core::types::{Signature, H256};
    use ethers::signers::{LocalWallet, Signer};
    use ethers_gcp_kms_signer::{GcpKeyRingRef, GcpKmsProvider, GcpKmsSigner};

    // Helper function to have signers sign a fixed 32-byte message hash
    async fn sign_message<S: Signer>(
        signer: &S,
        message: &[u8],
    ) -> Result<Signature, Box<dyn std::error::Error>>
    where
        S::Error: 'static,
    {
        let signature = signer.sign_message(message).await?;
        Ok(signature)
    }

    #[tokio::test]
    async fn test_remote_and_local_signing_agree() {
        // Skip this test if we don't have GCP credentials configured
        if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_err() {
            println!("Skipping remote signing test - no GCP credentials found");
            return;
        }

        // Create a fixed message to sign
        let message = b"Hello, World!";
        println!("Message to sign: {}", String::from_utf8_lossy(message));
        println!("Message bytes: {:?}", message);

        // Get the PEM formatted private key from environment
        let pem_key = std::env::var("TEST_PRIVATE_KEY")
            .expect("TEST_PRIVATE_KEY environment variable not set");

        // Parse the PEM key into bytes
        let secret_key: k256::elliptic_curve::SecretKey<k256::Secp256k1> =
            pem_key.parse().expect("Failed to parse PEM private key");
        let secret_key_bytes = secret_key.to_bytes();
        let secret_key = ethers::core::k256::SecretKey::from_bytes(&secret_key_bytes).unwrap();
        let signing_key = ethers::core::k256::ecdsa::SigningKey::from(secret_key);

        // Convert to LocalWallet
        let local_wallet = LocalWallet::from(signing_key);
        let local_address = local_wallet.address();
        println!("Local wallet address: {}", local_address);

        // For the remote signer, assume that the key in GCP KMS is set up to match the same private key
        // These values should be set via environment variables for CI/testing
        let project_id =
            std::env::var("GCP_PROJECT_ID").expect("GCP_PROJECT_ID environment variable not set");
        let location =
            std::env::var("GCP_LOCATION").expect("GCP_LOCATION environment variable not set");
        let key_ring =
            std::env::var("GCP_KEY_RING").expect("GCP_KEY_RING environment variable not set");
        let key_name =
            std::env::var("GCP_KEY_NAME").expect("GCP_KEY_NAME environment variable not set");
        let key_version: u64 = std::env::var("GCP_KEY_VERSION")
            .expect("GCP_KEY_VERSION environment variable not set")
            .parse()
            .expect("GCP_KEY_VERSION is not a valid integer");

        // Create the GCP KMS provider and remote signer
        let keyring = GcpKeyRingRef::new(&project_id, &location, &key_ring);
        let provider = GcpKmsProvider::new(keyring)
            .await
            .expect("Failed to create GCP KMS provider");
        let remote_signer = GcpKmsSigner::new(provider, key_name.to_string(), key_version, 1)
            .await
            .expect("Failed to create remote signer");
        let remote_address = remote_signer.address();
        println!("Remote signer address: {}", remote_address);

        // Verify addresses match
        assert_eq!(
            local_address, remote_address,
            "Local and remote addresses don't match"
        );

        // Sign the message with both signers
        let local_signature = sign_message(&local_wallet, message)
            .await
            .expect("Local signing failed");
        let remote_signature = sign_message(&remote_signer, message)
            .await
            .expect("Remote signing failed");

        // Verify both signatures are valid and recover to the same address
        let recovered_local = local_signature
            .recover(message.as_slice())
            .expect("Failed to recover local signature");
        let recovered_remote = remote_signature
            .recover(message.as_slice())
            .expect("Failed to recover remote signature");

        assert_eq!(
            recovered_local, local_address,
            "Local signature recovery failed"
        );
        assert_eq!(
            recovered_remote, remote_address,
            "Remote signature recovery failed"
        );

        println!("Both signatures are valid and recover to the same address!");
    }
}
