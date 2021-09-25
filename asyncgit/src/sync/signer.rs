use std::io::Write;
use std::path::Path;

use anyhow::Context;
use openpgp::armor;
use openpgp::cert::prelude::*;
use openpgp::crypto::KeyPair;
use openpgp::packet::prelude::*;
use openpgp::parse::Parse;
use openpgp::policy::{Policy, StandardPolicy};
use openpgp::serialize::stream::{Armorer, Message, Signer};
use openpgp::types::SignatureType;
use sequoia_openpgp as openpgp;

pub fn get_signing_keys(
	cert: &openpgp::Cert,
	p: &dyn Policy,
) -> openpgp::Result<Vec<KeyPair>> {
	let mut final_keys = Vec::new();

	let cert_keys = cert
		.keys()
		.with_policy(p, None)
		.alive()
		.revoked(false)
		.for_signing()
		.supported();

	'cert: for key in cert_keys.map(|ka| ka.key()) {
		if let Some(secret) = key.optional_secret() {
			let unencrypted = match secret {
				SecretKeyMaterial::Encrypted(ref _e) => {
					return Err(anyhow::anyhow!(format!(
                        "Signing of commits with encrypted secret not currently supported")
                    ));
				}
				SecretKeyMaterial::Unencrypted(ref u) => u.clone(),
			};

			final_keys.push(
				KeyPair::new(key.clone(), unencrypted)
					.expect("Failed to create keypair"),
			);

			break 'cert;
		}

		return Err(anyhow::anyhow!(format!(
			"No suitable signing key for: {}, ensure you have properly exported your private key.",
			cert
		)));
	}

	Ok(final_keys)
}

pub fn create_signature(
	commit: &str,
	signature: &mut (dyn Write + Send + Sync),
	cert: &Path,
) -> openpgp::Result<()> {
	let cert = Cert::from_file(cert)
		.context("Failed to read signing key")?;

	let mut keypairs =
		get_signing_keys(&cert, &StandardPolicy::new())?;

	let message = Message::new(signature);
	let message =
		Armorer::new(message).kind(armor::Kind::Signature).build()?;

	let builder = SignatureBuilder::new(SignatureType::Binary);
	let mut signer = Signer::with_template(
		message,
		keypairs.pop().context("No key for signing")?,
		builder,
	);

	signer = signer.detached();

	let mut signer =
		signer.build().context("Failed to create signer")?;
	signer.write_all(commit.as_bytes())?;
	signer.finalize().context("Failed to sign commit")?;

	Ok(())
}
