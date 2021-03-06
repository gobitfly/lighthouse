use crate::test_utils::AttesterSlashingTestTask;
use crate::*;
use tree_hash::TreeHash;

/// Builds an `AttesterSlashing`.
///
/// This struct should **never be used for production purposes.**
pub struct TestingAttesterSlashingBuilder();

impl TestingAttesterSlashingBuilder {
    /// Builds an `AttesterSlashing` that is a double vote.
    ///
    /// The `signer` function is used to sign the double-vote and accepts:
    ///
    /// - `validator_index: u64`
    /// - `message: &[u8]`
    /// - `epoch: Epoch`
    /// - `domain: Domain`
    ///
    /// Where domain is a domain "constant" (e.g., `spec.domain_attestation`).
    pub fn double_vote<F, T: EthSpec>(
        test_task: AttesterSlashingTestTask,
        validator_indices: &[u64],
        signer: F,
    ) -> AttesterSlashing<T>
    where
        F: Fn(u64, &[u8], Epoch, Domain) -> Signature,
    {
        let slot = Slot::new(1);
        let index = 0;
        let epoch_1 = Epoch::new(1);
        let epoch_2 = Epoch::new(2);
        let hash_1 = Hash256::from_low_u64_le(1);
        let hash_2 = Hash256::from_low_u64_le(2);
        let checkpoint_1 = Checkpoint {
            epoch: epoch_1,
            root: hash_1,
        };
        let checkpoint_2 = Checkpoint {
            epoch: epoch_1,
            root: hash_2,
        };

        let data_1 = AttestationData {
            slot,
            index,
            beacon_block_root: hash_1,
            source: checkpoint_1.clone(),
            target: checkpoint_1,
        };

        let data_2 = if test_task == AttesterSlashingTestTask::NotSlashable {
            AttestationData { ..data_1.clone() }
        } else {
            AttestationData {
                target: checkpoint_2,
                ..data_1.clone()
            }
        };

        let mut attestation_1 = IndexedAttestation {
            attesting_indices: if test_task == AttesterSlashingTestTask::IndexedAttestation1Invalid
            {
                // Trigger bad validator indices ordering error.
                vec![1, 0].into()
            } else {
                validator_indices.to_vec().into()
            },
            data: data_1,
            signature: AggregateSignature::new(),
        };

        let mut attestation_2 = IndexedAttestation {
            attesting_indices: if test_task == AttesterSlashingTestTask::IndexedAttestation2Invalid
            {
                // Trigger bad validator indices ordering error.
                vec![1, 0].into()
            } else {
                validator_indices.to_vec().into()
            },
            data: data_2,
            signature: AggregateSignature::new(),
        };

        let add_signatures = |attestation: &mut IndexedAttestation<T>| {
            let message = attestation.data.tree_hash_root();

            for validator_index in validator_indices {
                let signature = signer(
                    *validator_index,
                    &message[..],
                    epoch_2,
                    Domain::BeaconAttester,
                );
                attestation.signature.add(&signature);
            }
        };

        add_signatures(&mut attestation_1);
        add_signatures(&mut attestation_2);

        AttesterSlashing {
            attestation_1,
            attestation_2,
        }
    }
}
