#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use core::marker::PhantomData;
use frame_support::weights::constants::RocksDbWeight;

use frame_support::weights::Weight;
use pallet_contracts::weights::WeightInfo;
use sp_core::Get;

/// Weights for pallet_contracts using the Substrate node and recommended hardware.
pub struct PinkWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PinkWeights<T> {
    /// Storage: Contracts DeletionQueue (r:1 w:0)
    /// Proof: Contracts DeletionQueue (max_values: Some(1), max_size: Some(16642), added: 17137, mode: Measured)
    fn on_process_deletion_queue_batch() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `604`
        // Minimum execution time: 2_591 nanoseconds.
        Weight::from_ref_time(2_817_000)
            .saturating_add(Weight::from_proof_size(604))
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `k` is `[0, 1024]`.
    fn on_initialize_per_trie_key(k: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `481 + k * (69 ±0)`
        //  Estimated: `471 + k * (70 ±0)`
        // Minimum execution time: 10_190 nanoseconds.
        Weight::from_ref_time(6_642_117)
            .saturating_add(Weight::from_proof_size(471))
            // Standard Error: 992
            .saturating_add(Weight::from_ref_time(919_828).saturating_mul(k.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k.into())))
            .saturating_add(Weight::from_proof_size(70).saturating_mul(k.into()))
    }
    /// Storage: Contracts DeletionQueue (r:1 w:1)
    /// Proof: Contracts DeletionQueue (max_values: Some(1), max_size: Some(16642), added: 17137, mode: Measured)
    /// The range of component `q` is `[0, 128]`.
    fn on_initialize_per_queue_item(q: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `281 + q * (33 ±0)`
        //  Estimated: `763 + q * (33 ±0)`
        // Minimum execution time: 2_598 nanoseconds.
        Weight::from_ref_time(10_288_252)
            .saturating_add(Weight::from_proof_size(763))
            // Standard Error: 2_886
            .saturating_add(Weight::from_ref_time(1_092_420).saturating_mul(q.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(Weight::from_proof_size(33).saturating_mul(q.into()))
    }
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// The range of component `c` is `[0, 61717]`.
    fn reinstrument(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `270 + c * (1 ±0)`
        //  Estimated: `3025 + c * (2 ±0)`
        // Minimum execution time: 34_338 nanoseconds.
        Weight::from_ref_time(32_159_677)
            .saturating_add(Weight::from_proof_size(3025))
            // Standard Error: 53
            .saturating_add(Weight::from_ref_time(51_034).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(Weight::from_proof_size(2).saturating_mul(c.into()))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `c` is `[0, 125952]`.
    fn call_with_code_per_byte(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `803`
        //  Estimated: `16930 + c * (5 ±0)`
        // Minimum execution time: 385_587 nanoseconds.
        Weight::from_ref_time(395_545_811)
            .saturating_add(Weight::from_proof_size(16930))
            // Standard Error: 27
            .saturating_add(Weight::from_ref_time(31_342).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_proof_size(5).saturating_mul(c.into()))
    }
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// The range of component `c` is `[0, 61717]`.
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate_with_code(c: u32, i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `270`
        //  Estimated: `20267`
        // Minimum execution time: 3_799_742 nanoseconds.
        Weight::from_ref_time(670_115_588)
            .saturating_add(Weight::from_proof_size(20267))
            // Standard Error: 287
            .saturating_add(Weight::from_ref_time(93_885).saturating_mul(c.into()))
            // Standard Error: 16
            .saturating_add(Weight::from_ref_time(1_367).saturating_mul(i.into()))
            // Standard Error: 16
            .saturating_add(Weight::from_ref_time(1_781).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(10_u64))
    }
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate(i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `546`
        //  Estimated: `22039`
        // Minimum execution time: 1_949_008 nanoseconds.
        Weight::from_ref_time(214_033_418)
            .saturating_add(Weight::from_proof_size(22039))
            // Standard Error: 8
            .saturating_add(Weight::from_ref_time(1_666).saturating_mul(i.into()))
            // Standard Error: 8
            .saturating_add(Weight::from_ref_time(1_801).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(7_u64))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    fn call() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `855`
        //  Estimated: `17145`
        // Minimum execution time: 146_654 nanoseconds.
        Weight::from_ref_time(147_528_000)
            .saturating_add(Weight::from_proof_size(17145))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1 w:1)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// The range of component `c` is `[0, 61717]`.
    fn upload_code(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `5386`
        // Minimum execution time: 387_889 nanoseconds.
        Weight::from_ref_time(391_379_335)
            .saturating_add(Weight::from_proof_size(5386))
            // Standard Error: 89
            .saturating_add(Weight::from_ref_time(94_810).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1 w:1)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    fn remove_code() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `287`
        //  Estimated: `6098`
        // Minimum execution time: 26_014 nanoseconds.
        Weight::from_ref_time(26_510_000)
            .saturating_add(Weight::from_proof_size(6098))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:2 w:2)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    fn set_code() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `666`
        //  Estimated: `16848`
        // Minimum execution time: 30_177 nanoseconds.
        Weight::from_ref_time(30_639_000)
            .saturating_add(Weight::from_proof_size(16848))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_caller(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `877 + r * (480 ±0)`
        //  Estimated: `17295 + r * (2400 ±0)`
        // Minimum execution time: 373_786 nanoseconds.
        Weight::from_ref_time(377_332_691)
            .saturating_add(Weight::from_proof_size(17295))
            // Standard Error: 51_211
            .saturating_add(Weight::from_ref_time(17_715_615).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2400).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_is_contract(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `917 + r * (21778 ±0)`
        //  Estimated: `17295 + r * (306895 ±0)`
        // Minimum execution time: 374_009 nanoseconds.
        Weight::from_ref_time(238_991_986)
            .saturating_add(Weight::from_proof_size(17295))
            // Standard Error: 464_711
            .saturating_add(Weight::from_ref_time(249_099_538).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((80_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(306895).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `921 + r * (22099 ±0)`
        //  Estimated: `17340 + r * (308500 ±0)`
        // Minimum execution time: 375_058 nanoseconds.
        Weight::from_ref_time(238_765_068)
            .saturating_add(Weight::from_proof_size(17340))
            // Standard Error: 662_617
            .saturating_add(Weight::from_ref_time(302_175_089).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((80_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(308500).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_own_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `884 + r * (480 ±0)`
        //  Estimated: `17330 + r * (2400 ±0)`
        // Minimum execution time: 374_747 nanoseconds.
        Weight::from_ref_time(376_482_380)
            .saturating_add(Weight::from_proof_size(17330))
            // Standard Error: 61_919
            .saturating_add(Weight::from_ref_time(22_376_795).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2400).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_caller_is_origin(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `874 + r * (240 ±0)`
        //  Estimated: `17265 + r * (1200 ±0)`
        // Minimum execution time: 372_287 nanoseconds.
        Weight::from_ref_time(376_250_858)
            .saturating_add(Weight::from_proof_size(17265))
            // Standard Error: 40_119
            .saturating_add(Weight::from_ref_time(11_359_647).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(1200).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_address(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `878 + r * (480 ±0)`
        //  Estimated: `17260 + r * (2400 ±0)`
        // Minimum execution time: 374_445 nanoseconds.
        Weight::from_ref_time(377_243_521)
            .saturating_add(Weight::from_proof_size(17260))
            // Standard Error: 53_032
            .saturating_add(Weight::from_ref_time(17_684_246).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2400).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_gas_left(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `879 + r * (480 ±0)`
        //  Estimated: `17250 + r * (2405 ±0)`
        // Minimum execution time: 374_029 nanoseconds.
        Weight::from_ref_time(380_415_186)
            .saturating_add(Weight::from_proof_size(17250))
            // Standard Error: 60_562
            .saturating_add(Weight::from_ref_time(17_152_599).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2405).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:2 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_balance(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1049 + r * (480 ±0)`
        //  Estimated: `19849 + r * (2456 ±0)`
        // Minimum execution time: 373_999 nanoseconds.
        Weight::from_ref_time(381_757_033)
            .saturating_add(Weight::from_proof_size(19849))
            // Standard Error: 97_983
            .saturating_add(Weight::from_ref_time(98_290_984).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2456).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_value_transferred(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `888 + r * (480 ±0)`
        //  Estimated: `17360 + r * (2400 ±0)`
        // Minimum execution time: 374_197 nanoseconds.
        Weight::from_ref_time(377_755_896)
            .saturating_add(Weight::from_proof_size(17360))
            // Standard Error: 60_542
            .saturating_add(Weight::from_ref_time(17_442_065).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2400).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_minimum_balance(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `886 + r * (480 ±0)`
        //  Estimated: `17290 + r * (2400 ±0)`
        // Minimum execution time: 373_888 nanoseconds.
        Weight::from_ref_time(377_825_771)
            .saturating_add(Weight::from_proof_size(17290))
            // Standard Error: 38_026
            .saturating_add(Weight::from_ref_time(17_147_903).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2400).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_block_number(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `883 + r * (480 ±0)`
        //  Estimated: `17315 + r * (2400 ±0)`
        // Minimum execution time: 373_904 nanoseconds.
        Weight::from_ref_time(378_652_372)
            .saturating_add(Weight::from_proof_size(17315))
            // Standard Error: 43_833
            .saturating_add(Weight::from_ref_time(16_936_781).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2400).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_now(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `874 + r * (480 ±0)`
        //  Estimated: `17245 + r * (2400 ±0)`
        // Minimum execution time: 373_473 nanoseconds.
        Weight::from_ref_time(376_386_312)
            .saturating_add(Weight::from_proof_size(17245))
            // Standard Error: 46_945
            .saturating_add(Weight::from_ref_time(17_336_462).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2400).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
    /// Proof: TransactionPayment NextFeeMultiplier (max_values: Some(1), max_size: Some(16), added: 511, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_weight_to_fee(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `951 + r * (800 ±0)`
        //  Estimated: `19046 + r * (4805 ±0)`
        // Minimum execution time: 373_661 nanoseconds.
        Weight::from_ref_time(385_824_015)
            .saturating_add(Weight::from_proof_size(19046))
            // Standard Error: 75_964
            .saturating_add(Weight::from_ref_time(88_530_074).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(4805).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_gas(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `841 + r * (320 ±0)`
        //  Estimated: `17120 + r * (1600 ±0)`
        // Minimum execution time: 133_849 nanoseconds.
        Weight::from_ref_time(137_283_391)
            .saturating_add(Weight::from_proof_size(17120))
            // Standard Error: 13_312
            .saturating_add(Weight::from_ref_time(8_055_328).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(1600).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_input(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `876 + r * (480 ±0)`
        //  Estimated: `17245 + r * (2400 ±0)`
        // Minimum execution time: 373_468 nanoseconds.
        Weight::from_ref_time(376_121_093)
            .saturating_add(Weight::from_proof_size(17245))
            // Standard Error: 61_857
            .saturating_add(Weight::from_ref_time(15_868_414).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2400).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_input_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1356`
        //  Estimated: `19650`
        // Minimum execution time: 390_668 nanoseconds.
        Weight::from_ref_time(419_608_449)
            .saturating_add(Weight::from_proof_size(19650))
            // Standard Error: 4_890
            .saturating_add(Weight::from_ref_time(9_672_288).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_return(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `864 + r * (45 ±0)`
        //  Estimated: `17190 + r * (225 ±0)`
        // Minimum execution time: 371_309 nanoseconds.
        Weight::from_ref_time(373_625_402)
            .saturating_add(Weight::from_proof_size(17190))
            // Standard Error: 419_605
            .saturating_add(Weight::from_ref_time(1_737_397).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(225).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_return_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `874`
        //  Estimated: `17285`
        // Minimum execution time: 374_094 nanoseconds.
        Weight::from_ref_time(375_965_200)
            .saturating_add(Weight::from_proof_size(17285))
            // Standard Error: 1_127
            .saturating_add(Weight::from_ref_time(232_645).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:4 w:4)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts DeletionQueue (r:1 w:1)
    /// Proof: Contracts DeletionQueue (max_values: Some(1), max_size: Some(16642), added: 17137, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_terminate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `906 + r * (452 ±0)`
        //  Estimated: `20242 + r * (15004 ±0)`
        // Minimum execution time: 373_123 nanoseconds.
        Weight::from_ref_time(374_924_634)
            .saturating_add(Weight::from_proof_size(20242))
            // Standard Error: 378_010
            .saturating_add(Weight::from_ref_time(70_441_665).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_proof_size(15004).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
    /// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_random(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `921 + r * (800 ±0)`
        //  Estimated: `18835 + r * (4805 ±0)`
        // Minimum execution time: 373_291 nanoseconds.
        Weight::from_ref_time(385_684_344)
            .saturating_add(Weight::from_proof_size(18835))
            // Standard Error: 99_025
            .saturating_add(Weight::from_ref_time(111_308_793).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(4805).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_deposit_event(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `874 + r * (800 ±0)`
        //  Estimated: `17250 + r * (4000 ±0)`
        // Minimum execution time: 371_900 nanoseconds.
        Weight::from_ref_time(384_166_626)
            .saturating_add(Weight::from_proof_size(17250))
            // Standard Error: 205_255
            .saturating_add(Weight::from_ref_time(229_214_157).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(4000).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:322 w:322)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 4]`.
    /// The range of component `n` is `[0, 16]`.
    fn seal_deposit_event_per_topic_and_kb(t: u32, n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1821 + t * (2608 ±0) + n * (7 ±0)`
        //  Estimated: `21870 + t * (211030 ±0) + n * (50 ±0)`
        // Minimum execution time: 1_289_873 nanoseconds.
        Weight::from_ref_time(581_702_206)
            .saturating_add(Weight::from_proof_size(21870))
            // Standard Error: 665_638
            .saturating_add(Weight::from_ref_time(181_470_553).saturating_mul(t.into()))
            // Standard Error: 182_816
            .saturating_add(Weight::from_ref_time(71_635_250).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((80_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((80_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_proof_size(211030).saturating_mul(t.into()))
            .saturating_add(Weight::from_proof_size(50).saturating_mul(n.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_debug_message(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `873 + r * (560 ±0)`
        //  Estimated: `17240 + r * (2800 ±0)`
        // Minimum execution time: 148_635 nanoseconds.
        Weight::from_ref_time(154_095_712)
            .saturating_add(Weight::from_proof_size(17240))
            // Standard Error: 77_790
            .saturating_add(Weight::from_ref_time(14_837_085).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(2800).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[0, 1024]`.
    fn seal_debug_message_per_kb(i: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `125824`
        //  Estimated: `265128`
        // Minimum execution time: 501_014 nanoseconds.
        Weight::from_ref_time(505_634_218)
            .saturating_add(Weight::from_proof_size(265128))
            // Standard Error: 2_441
            .saturating_add(Weight::from_ref_time(819_257).saturating_mul(i.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 10]`.
    fn seal_set_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `911 + r * (23420 ±0)`
        //  Estimated: `911 + r * (23418 ±0)`
        // Minimum execution time: 375_301 nanoseconds.
        Weight::from_ref_time(291_498_841)
            .saturating_add(Weight::from_proof_size(911))
            // Standard Error: 809_989
            .saturating_add(Weight::from_ref_time(464_550_291).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((80_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((80_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_proof_size(23418).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 8]`.
    fn seal_set_storage_per_new_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `12672 + n * (11945 ±0)`
        //  Estimated: `8529 + n * (12814 ±61)`
        // Minimum execution time: 506_318 nanoseconds.
        Weight::from_ref_time(676_935_313)
            .saturating_add(Weight::from_proof_size(8529))
            // Standard Error: 1_589_291
            .saturating_add(Weight::from_ref_time(97_839_399).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(52_u64))
            .saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(50_u64))
            .saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_proof_size(12814).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 8]`.
    fn seal_set_storage_per_old_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `15170 + n * (175775 ±0)`
        //  Estimated: `9914 + n * (176858 ±74)`
        // Minimum execution time: 506_148 nanoseconds.
        Weight::from_ref_time(648_278_778)
            .saturating_add(Weight::from_proof_size(9914))
            // Standard Error: 1_343_586
            .saturating_add(Weight::from_ref_time(65_789_595).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(51_u64))
            .saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(49_u64))
            .saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_proof_size(176858).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 10]`.
    fn seal_clear_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `903 + r * (23099 ±0)`
        //  Estimated: `908 + r * (23099 ±0)`
        // Minimum execution time: 374_344 nanoseconds.
        Weight::from_ref_time(293_272_061)
            .saturating_add(Weight::from_proof_size(908))
            // Standard Error: 810_412
            .saturating_add(Weight::from_ref_time(453_315_956).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((80_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((80_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_proof_size(23099).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 8]`.
    fn seal_clear_storage_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `14895 + n * (175768 ±0)`
        //  Estimated: `9551 + n * (176867 ±75)`
        // Minimum execution time: 478_564 nanoseconds.
        Weight::from_ref_time(630_839_142)
            .saturating_add(Weight::from_proof_size(9551))
            // Standard Error: 1_427_520
            .saturating_add(Weight::from_ref_time(66_813_592).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(51_u64))
            .saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(48_u64))
            .saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_proof_size(176867).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 10]`.
    fn seal_get_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `896 + r * (23744 ±0)`
        //  Estimated: `909 + r * (23740 ±0)`
        // Minimum execution time: 374_479 nanoseconds.
        Weight::from_ref_time(311_839_315)
            .saturating_add(Weight::from_proof_size(909))
            // Standard Error: 666_553
            .saturating_add(Weight::from_ref_time(371_213_042).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((80_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(23740).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 8]`.
    fn seal_get_storage_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `15501 + n * (175775 ±0)`
        //  Estimated: `10042 + n * (176900 ±76)`
        // Minimum execution time: 460_639 nanoseconds.
        Weight::from_ref_time(591_187_094)
            .saturating_add(Weight::from_proof_size(10042))
            // Standard Error: 1_233_792
            .saturating_add(Weight::from_ref_time(160_874_477).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(51_u64))
            .saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(176900).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 10]`.
    fn seal_contains_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `914 + r * (23098 ±0)`
        //  Estimated: `920 + r * (23098 ±0)`
        // Minimum execution time: 374_272 nanoseconds.
        Weight::from_ref_time(311_446_269)
            .saturating_add(Weight::from_proof_size(920))
            // Standard Error: 630_307
            .saturating_add(Weight::from_ref_time(357_134_931).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((80_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(23098).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 8]`.
    fn seal_contains_storage_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `14839 + n * (175789 ±0)`
        //  Estimated: `9532 + n * (176874 ±75)`
        // Minimum execution time: 456_013 nanoseconds.
        Weight::from_ref_time(575_116_352)
            .saturating_add(Weight::from_proof_size(9532))
            // Standard Error: 1_122_298
            .saturating_add(Weight::from_ref_time(61_786_107).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(51_u64))
            .saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(176874).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 10]`.
    fn seal_take_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `911 + r * (23740 ±0)`
        //  Estimated: `913 + r * (23739 ±0)`
        // Minimum execution time: 374_621 nanoseconds.
        Weight::from_ref_time(299_689_489)
            .saturating_add(Weight::from_proof_size(913))
            // Standard Error: 757_735
            .saturating_add(Weight::from_ref_time(465_213_246).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((80_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((80_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_proof_size(23739).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 8]`.
    fn seal_take_storage_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `15502 + n * (175775 ±0)`
        //  Estimated: `10042 + n * (176898 ±76)`
        // Minimum execution time: 481_980 nanoseconds.
        Weight::from_ref_time(647_289_053)
            .saturating_add(Weight::from_proof_size(10042))
            // Standard Error: 1_556_155
            .saturating_add(Weight::from_ref_time(166_592_657).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(51_u64))
            .saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(48_u64))
            .saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_proof_size(176898).saturating_mul(n.into()))
    }
    /// Storage: System Account (r:1602 w:1601)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_transfer(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1457 + r * (3604 ±0)`
        //  Estimated: `21583 + r * (216101 ±0)`
        // Minimum execution time: 374_962 nanoseconds.
        Weight::from_ref_time(313_416_386)
            .saturating_add(Weight::from_proof_size(21583))
            // Standard Error: 710_675
            .saturating_add(Weight::from_ref_time(1_396_551_156).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().reads((80_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(T::DbWeight::get().writes((80_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_proof_size(216101).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1601)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:2 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:1602 w:1602)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1609 + r * (23073 ±0)`
        //  Estimated: `22098 + r * (511456 ±1)`
        // Minimum execution time: 375_916 nanoseconds.
        Weight::from_ref_time(376_468_000)
            .saturating_add(Weight::from_proof_size(22098))
            // Standard Error: 7_246_855
            .saturating_add(Weight::from_ref_time(28_982_425_139).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().reads((160_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((160_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_proof_size(511456).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1536 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:1537 w:1537)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_delegate_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + r * (71670 ±0)`
        //  Estimated: `17285 + r * (659930 ±563)`
        // Minimum execution time: 375_412 nanoseconds.
        Weight::from_ref_time(376_493_000)
            .saturating_add(Weight::from_proof_size(17285))
            // Standard Error: 8_239_575
            .saturating_add(Weight::from_ref_time(28_716_347_183).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((150_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((75_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_proof_size(659930).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:82 w:81)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:81 w:81)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:2 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:82 w:82)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `c` is `[0, 1024]`.
    fn seal_call_per_transfer_clone_kb(t: u32, c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `24269 + t * (16910 ±0)`
        //  Estimated: `532690 + t * (285025 ±0)`
        // Minimum execution time: 10_443_315 nanoseconds.
        Weight::from_ref_time(9_342_574_069)
            .saturating_add(Weight::from_proof_size(532690))
            // Standard Error: 7_237_279
            .saturating_add(Weight::from_ref_time(1_390_221_936).saturating_mul(t.into()))
            // Standard Error: 10_851
            .saturating_add(Weight::from_ref_time(9_842_151).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(167_u64))
            .saturating_add(T::DbWeight::get().reads((81_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(163_u64))
            .saturating_add(T::DbWeight::get().writes((81_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_proof_size(285025).saturating_mul(t.into()))
    }
    /// Storage: System Account (r:3202 w:3202)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1601)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1601 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1600 w:1600)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1602 w:1602)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_instantiate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1775 + r * (25568 ±0)`
        //  Estimated: `26563 + r * (1367114 ±2)`
        // Minimum execution time: 376_418 nanoseconds.
        Weight::from_ref_time(377_292_000)
            .saturating_add(Weight::from_proof_size(26563))
            // Standard Error: 32_312_545
            .saturating_add(Weight::from_ref_time(35_904_826_312).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((480_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(5_u64))
            .saturating_add(T::DbWeight::get().writes((400_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_proof_size(1367114).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:162 w:162)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:81 w:81)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:2 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:82 w:82)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `i` is `[0, 960]`.
    /// The range of component `s` is `[0, 960]`.
    fn seal_instantiate_per_transfer_input_salt_kb(t: u32, i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `5785 + t * (33 ±0)`
        //  Estimated: `850985 + t * (2671 ±3)`
        // Minimum execution time: 132_157_340 nanoseconds.
        Weight::from_ref_time(11_329_968_948)
            .saturating_add(Weight::from_proof_size(850985))
            // Standard Error: 99_102_968
            .saturating_add(Weight::from_ref_time(84_719_458).saturating_mul(t.into()))
            // Standard Error: 161_609
            .saturating_add(Weight::from_ref_time(126_156_627).saturating_mul(i.into()))
            // Standard Error: 161_609
            .saturating_add(Weight::from_ref_time(126_628_313).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(329_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(326_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_proof_size(2671).saturating_mul(t.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_hash_sha2_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `871 + r * (642 ±0)`
        //  Estimated: `17225 + r * (3210 ±0)`
        // Minimum execution time: 373_559 nanoseconds.
        Weight::from_ref_time(375_166_904)
            .saturating_add(Weight::from_proof_size(17225))
            // Standard Error: 125_024
            .saturating_add(Weight::from_ref_time(42_291_595).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(3210).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_hash_sha2_256_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1673`
        //  Estimated: `21160`
        // Minimum execution time: 416_233 nanoseconds.
        Weight::from_ref_time(416_785_000)
            .saturating_add(Weight::from_proof_size(21160))
            // Standard Error: 56_223
            .saturating_add(Weight::from_ref_time(324_513_835).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_hash_keccak_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `873 + r * (642 ±0)`
        //  Estimated: `17235 + r * (3210 ±0)`
        // Minimum execution time: 371_735 nanoseconds.
        Weight::from_ref_time(375_979_430)
            .saturating_add(Weight::from_proof_size(17235))
            // Standard Error: 968_037
            .saturating_add(Weight::from_ref_time(57_780_769).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(3210).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_hash_keccak_256_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1675`
        //  Estimated: `21205`
        // Minimum execution time: 428_196 nanoseconds.
        Weight::from_ref_time(429_438_000)
            .saturating_add(Weight::from_proof_size(21205))
            // Standard Error: 57_860
            .saturating_add(Weight::from_ref_time(260_917_896).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_hash_blake2_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `873 + r * (642 ±0)`
        //  Estimated: `17235 + r * (3210 ±0)`
        // Minimum execution time: 371_412 nanoseconds.
        Weight::from_ref_time(373_635_818)
            .saturating_add(Weight::from_proof_size(17235))
            // Standard Error: 222_973
            .saturating_add(Weight::from_ref_time(33_347_181).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(3210).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_hash_blake2_256_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1675`
        //  Estimated: `21180`
        // Minimum execution time: 405_858 nanoseconds.
        Weight::from_ref_time(406_498_000)
            .saturating_add(Weight::from_proof_size(21180))
            // Standard Error: 48_388
            .saturating_add(Weight::from_ref_time(103_283_157).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_hash_blake2_128(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `873 + r * (679 ±0)`
        //  Estimated: `17235 + r * (3395 ±0)`
        // Minimum execution time: 371_746 nanoseconds.
        Weight::from_ref_time(373_538_171)
            .saturating_add(Weight::from_proof_size(17235))
            // Standard Error: 387_332
            .saturating_add(Weight::from_ref_time(35_933_528).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(3395).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_hash_blake2_128_per_kb(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1675`
        //  Estimated: `21225`
        // Minimum execution time: 405_752 nanoseconds.
        Weight::from_ref_time(406_417_000)
            .saturating_add(Weight::from_proof_size(21225))
            // Standard Error: 47_051
            .saturating_add(Weight::from_ref_time(103_325_027).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_ecdsa_recover(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `917 + r * (6083 ±0)`
        //  Estimated: `17455 + r * (30415 ±0)`
        // Minimum execution time: 373_882 nanoseconds.
        Weight::from_ref_time(376_553_787)
            .saturating_add(Weight::from_proof_size(17455))
            // Standard Error: 912_833
            .saturating_add(Weight::from_ref_time(3_021_100_412).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(30415).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_ecdsa_to_eth_address(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `886 + r * (3362 ±0)`
        //  Estimated: `17300 + r * (16810 ±0)`
        // Minimum execution time: 373_673 nanoseconds.
        Weight::from_ref_time(375_712_961)
            .saturating_add(Weight::from_proof_size(17300))
            // Standard Error: 596_297
            .saturating_add(Weight::from_ref_time(738_257_638).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(16810).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1536 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1536 w:1536)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1538 w:1538)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_set_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + r * (79300 ±0)`
        //  Estimated: `64844 + r * (942952 ±833)`
        // Minimum execution time: 374_097 nanoseconds.
        Weight::from_ref_time(374_985_000)
            .saturating_add(Weight::from_proof_size(64844))
            // Standard Error: 3_772_336
            .saturating_add(Weight::from_ref_time(1_546_402_854).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((225_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((150_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_proof_size(942952).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_reentrance_count(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `869 + r * (240 ±0)`
        //  Estimated: `17215 + r * (1200 ±0)`
        // Minimum execution time: 374_249 nanoseconds.
        Weight::from_ref_time(377_990_998)
            .saturating_add(Weight::from_proof_size(17215))
            // Standard Error: 38_133
            .saturating_add(Weight::from_ref_time(11_483_273).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(1200).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_account_reentrance_count(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2102 + r * (3154 ±0)`
        //  Estimated: `21980 + r * (15875 ±2)`
        // Minimum execution time: 375_552 nanoseconds.
        Weight::from_ref_time(400_624_032)
            .saturating_add(Weight::from_proof_size(21980))
            // Standard Error: 82_523
            .saturating_add(Weight::from_ref_time(18_057_327).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_proof_size(15875).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 20]`.
    fn seal_instantiation_nonce(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `872 + r * (240 ±0)`
        //  Estimated: `18598 + r * (1440 ±0)`
        // Minimum execution time: 373_899 nanoseconds.
        Weight::from_ref_time(379_733_943)
            .saturating_add(Weight::from_proof_size(18598))
            // Standard Error: 32_022
            .saturating_add(Weight::from_ref_time(9_381_180).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_proof_size(1440).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64const(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 834 nanoseconds.
        Weight::from_ref_time(1_009_646)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 388
            .saturating_add(Weight::from_ref_time(411_979).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64load(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 882 nanoseconds.
        Weight::from_ref_time(1_416_377)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 1_133
            .saturating_add(Weight::from_ref_time(1_075_838).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64store(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 878 nanoseconds.
        Weight::from_ref_time(1_343_056)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 426
            .saturating_add(Weight::from_ref_time(1_001_214).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_select(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 796 nanoseconds.
        Weight::from_ref_time(1_079_086)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 409
            .saturating_add(Weight::from_ref_time(1_149_188).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_if(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 800 nanoseconds.
        Weight::from_ref_time(1_044_184)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 707
            .saturating_add(Weight::from_ref_time(1_315_686).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_br(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 807 nanoseconds.
        Weight::from_ref_time(1_049_633)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 361
            .saturating_add(Weight::from_ref_time(640_530).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_br_if(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 774 nanoseconds.
        Weight::from_ref_time(1_124_053)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 784
            .saturating_add(Weight::from_ref_time(949_398).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_br_table(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 810 nanoseconds.
        Weight::from_ref_time(676_581)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 2_356
            .saturating_add(Weight::from_ref_time(1_163_465).saturating_mul(r.into()))
    }
    /// The range of component `e` is `[1, 256]`.
    fn instr_br_table_per_entry(e: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_580 nanoseconds.
        Weight::from_ref_time(2_835_656)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 71
            .saturating_add(Weight::from_ref_time(4_686).saturating_mul(e.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 826 nanoseconds.
        Weight::from_ref_time(1_625_698)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 1_740
            .saturating_add(Weight::from_ref_time(2_332_187).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_call_indirect(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 901 nanoseconds.
        Weight::from_ref_time(2_338_620)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 1_642
            .saturating_add(Weight::from_ref_time(2_924_090).saturating_mul(r.into()))
    }
    /// The range of component `p` is `[0, 128]`.
    fn instr_call_indirect_per_param(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 4_670 nanoseconds.
        Weight::from_ref_time(5_556_246)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 1_491
            .saturating_add(Weight::from_ref_time(228_965).saturating_mul(p.into()))
    }
    /// The range of component `l` is `[0, 1024]`.
    fn instr_call_per_local(l: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 3_099 nanoseconds.
        Weight::from_ref_time(3_896_177)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 99
            .saturating_add(Weight::from_ref_time(91_304).saturating_mul(l.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_local_get(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 3_042 nanoseconds.
        Weight::from_ref_time(3_334_621)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 793
            .saturating_add(Weight::from_ref_time(459_346).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_local_set(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_968 nanoseconds.
        Weight::from_ref_time(3_235_286)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 427
            .saturating_add(Weight::from_ref_time(485_454).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_local_tee(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 3_012 nanoseconds.
        Weight::from_ref_time(3_303_555)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 371
            .saturating_add(Weight::from_ref_time(657_811).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_global_get(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 865 nanoseconds.
        Weight::from_ref_time(1_249_987)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 417
            .saturating_add(Weight::from_ref_time(896_704).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_global_set(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 866 nanoseconds.
        Weight::from_ref_time(1_216_218)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 503
            .saturating_add(Weight::from_ref_time(919_719).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_memory_current(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 921 nanoseconds.
        Weight::from_ref_time(1_228_408)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 309
            .saturating_add(Weight::from_ref_time(813_007).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 1]`.
    fn instr_memory_grow(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 820 nanoseconds.
        Weight::from_ref_time(914_830)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 6_018
            .saturating_add(Weight::from_ref_time(237_062_769).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64clz(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 812 nanoseconds.
        Weight::from_ref_time(1_554_406)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 9_979
            .saturating_add(Weight::from_ref_time(625_434).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64ctz(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 798 nanoseconds.
        Weight::from_ref_time(1_095_113)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 243
            .saturating_add(Weight::from_ref_time(634_204).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64popcnt(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 792 nanoseconds.
        Weight::from_ref_time(1_109_845)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 14_944
            .saturating_add(Weight::from_ref_time(658_834).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64eqz(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 815 nanoseconds.
        Weight::from_ref_time(1_068_916)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 327
            .saturating_add(Weight::from_ref_time(652_897).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64extendsi32(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 798 nanoseconds.
        Weight::from_ref_time(1_069_745)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 306
            .saturating_add(Weight::from_ref_time(618_481).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64extendui32(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 799 nanoseconds.
        Weight::from_ref_time(1_398_001)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 6_234
            .saturating_add(Weight::from_ref_time(611_399).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i32wrapi64(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 811 nanoseconds.
        Weight::from_ref_time(1_098_083)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 297
            .saturating_add(Weight::from_ref_time(617_692).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64eq(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 815 nanoseconds.
        Weight::from_ref_time(1_046_922)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 335
            .saturating_add(Weight::from_ref_time(909_196).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64ne(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 805 nanoseconds.
        Weight::from_ref_time(1_093_667)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 233
            .saturating_add(Weight::from_ref_time(907_378).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64lts(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 805 nanoseconds.
        Weight::from_ref_time(1_290_591)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 3_201
            .saturating_add(Weight::from_ref_time(902_006).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64ltu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 783 nanoseconds.
        Weight::from_ref_time(1_159_977)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 2_310
            .saturating_add(Weight::from_ref_time(906_489).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64gts(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 790 nanoseconds.
        Weight::from_ref_time(1_109_719)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 261
            .saturating_add(Weight::from_ref_time(906_614).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64gtu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 776 nanoseconds.
        Weight::from_ref_time(1_076_567)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 348
            .saturating_add(Weight::from_ref_time(919_374).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64les(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 780 nanoseconds.
        Weight::from_ref_time(1_069_663)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 265
            .saturating_add(Weight::from_ref_time(908_037).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64leu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 832 nanoseconds.
        Weight::from_ref_time(930_920)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 2_170
            .saturating_add(Weight::from_ref_time(929_811).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64ges(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 787 nanoseconds.
        Weight::from_ref_time(1_087_325)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 315
            .saturating_add(Weight::from_ref_time(908_321).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64geu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 798 nanoseconds.
        Weight::from_ref_time(1_029_132)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 2_095
            .saturating_add(Weight::from_ref_time(913_553).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64add(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 791 nanoseconds.
        Weight::from_ref_time(1_086_314)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 197
            .saturating_add(Weight::from_ref_time(896_392).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64sub(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 793 nanoseconds.
        Weight::from_ref_time(1_078_172)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 404
            .saturating_add(Weight::from_ref_time(886_329).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64mul(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 799 nanoseconds.
        Weight::from_ref_time(1_095_010)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 431
            .saturating_add(Weight::from_ref_time(886_513).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64divs(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 810 nanoseconds.
        Weight::from_ref_time(1_114_325)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 452
            .saturating_add(Weight::from_ref_time(1_521_849).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64divu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 784 nanoseconds.
        Weight::from_ref_time(1_123_153)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 475
            .saturating_add(Weight::from_ref_time(1_457_746).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64rems(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 809 nanoseconds.
        Weight::from_ref_time(1_145_906)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 718
            .saturating_add(Weight::from_ref_time(1_549_964).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64remu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 803 nanoseconds.
        Weight::from_ref_time(1_110_328)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 627
            .saturating_add(Weight::from_ref_time(1_453_013).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64and(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 786 nanoseconds.
        Weight::from_ref_time(1_108_792)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 286
            .saturating_add(Weight::from_ref_time(897_035).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64or(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 787 nanoseconds.
        Weight::from_ref_time(830_000)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 15_995
            .saturating_add(Weight::from_ref_time(963_344).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64xor(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 773 nanoseconds.
        Weight::from_ref_time(1_082_459)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 330
            .saturating_add(Weight::from_ref_time(897_077).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64shl(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 810 nanoseconds.
        Weight::from_ref_time(1_325_815)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 3_352
            .saturating_add(Weight::from_ref_time(899_555).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64shrs(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 808 nanoseconds.
        Weight::from_ref_time(1_085_903)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 430
            .saturating_add(Weight::from_ref_time(903_249).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64shru(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 792 nanoseconds.
        Weight::from_ref_time(1_091_261)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 312
            .saturating_add(Weight::from_ref_time(902_245).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64rotl(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 807 nanoseconds.
        Weight::from_ref_time(1_121_052)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 506
            .saturating_add(Weight::from_ref_time(902_772).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64rotr(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 823 nanoseconds.
        Weight::from_ref_time(1_317_597)
            .saturating_add(Weight::from_proof_size(0))
            // Standard Error: 6_219
            .saturating_add(Weight::from_ref_time(896_692).saturating_mul(r.into()))
    }
}
