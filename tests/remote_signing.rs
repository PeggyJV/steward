#[cfg(test)]
mod tests {
    use ethers::core::types::H256;
    use ethers::signers::{LocalWallet, Signer};
    use ethers_gcp_kms_signer::{GcpKeyRingRef, GcpKmsProvider, GcpKmsSigner};
    use std::str::FromStr;

    // Helper function to have signers sign a fixed 32-byte message hash
    async fn sign_message<S: Signer>(signer: &S) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where
        S::Error: 'static,
    {
        // For simplicity, sign a fixed 32-byte message
        let message_hash = H256::from_low_u64_be(42);
        let signature = signer.sign_message(message_hash).await?;
        Ok(signature.to_vec())
    }

    #[tokio::test]
    async fn test_remote_and_local_signing_agree() {
        // Skip this test if we don't have GCP credentials configured
        if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_err() {
            println!("Skipping remote signing test - no GCP credentials found");
            return;
        }

        // Use a fixed private key for local signing
        let private_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let local_wallet = LocalWallet::from_str(private_key).expect("Invalid private key");

        // For the remote signer, assume that the key in GCP KMS is set up to match the same private key
        // These values should be set via environment variables for CI/testing
        let project_id =
            std::env::var("GCP_PROJECT_ID").unwrap_or_else(|_| "test-project".to_string());
        let location = std::env::var("GCP_LOCATION").unwrap_or_else(|_| "global".to_string());
        let key_ring = std::env::var("GCP_KEY_RING").unwrap_or_else(|_| "test-keyring".to_string());
        let key_name = std::env::var("GCP_KEY_NAME").unwrap_or_else(|_| "test-key".to_string());

        // Create the GCP KMS provider and remote signer
        let keyring = GcpKeyRingRef::new(&project_id, &location, &key_ring);
        let provider = GcpKmsProvider::new(keyring)
            .await
            .expect("Failed to create GCP KMS provider");
        let remote_signer = GcpKmsSigner::new(provider, key_name, 1, 1)
            .await
            .expect("Failed to create remote signer");

        // Sign the message with both signers
        let local_signature = sign_message(&local_wallet)
            .await
            .expect("Local signing failed");
        let remote_signature = sign_message(&remote_signer)
            .await
            .expect("Remote signing failed");

        // Compare the signatures - they should be identical since we're using the same key
        assert_eq!(
            local_signature, remote_signature,
            "Local and remote signatures don't match"
        );
    }
}
