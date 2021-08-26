#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod node_runtime {
    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 1)]
        Utility(utility::Event),
        #[codec(index = 5)]
        Indices(indices::Event),
        #[codec(index = 6)]
        Balances(balances::Event),
        #[codec(index = 8)]
        ElectionProviderMultiPhase(election_provider_multi_phase::Event),
        #[codec(index = 9)]
        Staking(staking::Event),
        #[codec(index = 10)]
        Session(session::Event),
        #[codec(index = 11)]
        Democracy(democracy::Event),
        #[codec(index = 12)]
        Council(council::Event),
        #[codec(index = 13)]
        TechnicalCommittee(technical_committee::Event),
        #[codec(index = 14)]
        Elections(elections::Event),
        #[codec(index = 15)]
        TechnicalMembership(technical_membership::Event),
        #[codec(index = 16)]
        Grandpa(grandpa::Event),
        #[codec(index = 17)]
        Treasury(treasury::Event),
        #[codec(index = 18)]
        Contracts(contracts::Event),
        #[codec(index = 19)]
        Sudo(sudo::Event),
        #[codec(index = 20)]
        ImOnline(im_online::Event),
        #[codec(index = 22)]
        Offences(offences::Event),
        #[codec(index = 25)]
        Identity(identity::Event),
        #[codec(index = 26)]
        Society(society::Event),
        #[codec(index = 27)]
        Recovery(recovery::Event),
        #[codec(index = 28)]
        Vesting(vesting::Event),
        #[codec(index = 29)]
        Scheduler(scheduler::Event),
        #[codec(index = 30)]
        Proxy(proxy::Event),
        #[codec(index = 31)]
        Multisig(multisig::Event),
        #[codec(index = 32)]
        Bounties(bounties::Event),
        #[codec(index = 33)]
        Tips(tips::Event),
        #[codec(index = 34)]
        Assets(assets::Event),
        #[codec(index = 36)]
        Lottery(lottery::Event),
        #[codec(index = 37)]
        Gilt(gilt::Event),
        #[codec(index = 38)]
        Uniques(uniques::Event),
        #[codec(index = 39)]
        TransactionStorage(transaction_storage::Event),
    }
    pub mod system {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct FillBlock {
                ratio: __runtime_types::sp_arithmetic::per_things::Perbill,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Remark {
                remark: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHeapPages {
                pages: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetCode {
                code: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetCodeWithoutChecks {
                code: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetChangesTrieConfig {
                changes_trie_config:
                    Option<__runtime_types::sp_core::changes_trie::ChangesTrieConfiguration>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetStorage {
                items: Vec<(Vec<u8>, Vec<u8>)>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct KillStorage {
                keys: Vec<Vec<u8>>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct KillPrefix {
                prefix: Vec<u8>,
                subkeys: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemarkWithEvent {
                remark: Vec<u8>,
            }
        }
        pub type Event = __runtime_types::frame_system::pallet::Event;
    }
    pub mod utility {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Batch {
                calls: Vec<__runtime_types::node_runtime::Call>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AsDerivative {
                index: u16,
                call: __runtime_types::node_runtime::Call,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct BatchAll {
                calls: Vec<__runtime_types::node_runtime::Call>,
            }
        }
        pub type Event = __runtime_types::pallet_utility::pallet::Event;
    }
    pub mod babe {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportEquivocation {
                equivocation_proof: __runtime_types::sp_consensus_slots::EquivocationProof<
                    __runtime_types::sp_runtime::generic::header::Header<
                        u32,
                        __runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                    __runtime_types::sp_consensus_babe::app::Public,
                >,
                key_owner_proof: __runtime_types::sp_session::MembershipProof,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                equivocation_proof: __runtime_types::sp_consensus_slots::EquivocationProof<
                    __runtime_types::sp_runtime::generic::header::Header<
                        u32,
                        __runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                    __runtime_types::sp_consensus_babe::app::Public,
                >,
                key_owner_proof: __runtime_types::sp_session::MembershipProof,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct PlanConfigChange {
                config: __runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
        }
    }
    pub mod timestamp {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Set {
                now: u64,
            }
        }
    }
    pub mod authorship {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetUncles {
                new_uncles: Vec<
                    __runtime_types::sp_runtime::generic::header::Header<
                        u32,
                        __runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
        }
    }
    pub mod indices {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Claim {
                index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Transfer {
                new: __runtime_types::sp_core::crypto::AccountId32,
                index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Free {
                index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceTransfer {
                new: __runtime_types::sp_core::crypto::AccountId32,
                index: u32,
                freeze: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Freeze {
                index: u32,
            }
        }
        pub type Event = __runtime_types::pallet_indices::pallet::Event;
    }
    pub mod balances {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Transfer {
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetBalance {
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                new_free: u128,
                new_reserved: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceTransfer {
                source: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransferKeepAlive {
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransferAll {
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                keep_alive: bool,
            }
        }
        pub type Event = __runtime_types::pallet_balances::pallet::Event;
    }
    pub mod transaction_payment {
        use super::__runtime_types;
    }
    pub mod election_provider_multi_phase {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SubmitUnsigned {
                raw_solution: __runtime_types::pallet_election_provider_multi_phase::RawSolution<
                    __runtime_types::node_runtime::NposSolution16,
                >,
                witness:
                    __runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMinimumUntrustedScore {
                maybe_next_score: Option<[u128; 3usize]>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetEmergencyElectionResult {
                supports: Vec<(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::sp_npos_elections::Support<
                        __runtime_types::sp_core::crypto::AccountId32,
                    >,
                )>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Submit {
                raw_solution: __runtime_types::pallet_election_provider_multi_phase::RawSolution<
                    __runtime_types::node_runtime::NposSolution16,
                >,
                num_signed_submissions: u32,
            }
        }
        pub type Event = __runtime_types::pallet_election_provider_multi_phase::pallet::Event;
    }
    pub mod staking {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Bond {
                controller: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                value: u128,
                payee: __runtime_types::pallet_staking::RewardDestination<
                    __runtime_types::sp_core::crypto::AccountId32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct BondExtra {
                max_additional: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unbond {
                value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct WithdrawUnbonded {
                num_slashing_spans: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Validate {
                prefs: __runtime_types::pallet_staking::ValidatorPrefs,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Nominate {
                targets: Vec<
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Chill {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetPayee {
                payee: __runtime_types::pallet_staking::RewardDestination<
                    __runtime_types::sp_core::crypto::AccountId32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetController {
                controller: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetValidatorCount {
                new: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct IncreaseValidatorCount {
                additional: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScaleValidatorCount {
                factor: __runtime_types::sp_arithmetic::per_things::Percent,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceNoEras {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceNewEra {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetInvulnerables {
                invulnerables: Vec<__runtime_types::sp_core::crypto::AccountId32>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceUnstake {
                stash: __runtime_types::sp_core::crypto::AccountId32,
                num_slashing_spans: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceNewEraAlways {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelDeferredSlash {
                era: u32,
                slash_indices: Vec<u32>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct PayoutStakers {
                validator_stash: __runtime_types::sp_core::crypto::AccountId32,
                era: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Rebond {
                value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetHistoryDepth {
                new_history_depth: u32,
                era_items_deleted: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReapStash {
                stash: __runtime_types::sp_core::crypto::AccountId32,
                num_slashing_spans: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Kick {
                who: Vec<
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetStakingLimits {
                min_nominator_bond: u128,
                min_validator_bond: u128,
                max_nominator_count: Option<u32>,
                max_validator_count: Option<u32>,
                threshold: Option<__runtime_types::sp_arithmetic::per_things::Percent>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ChillOther {
                controller: __runtime_types::sp_core::crypto::AccountId32,
            }
        }
        pub type Event = __runtime_types::pallet_staking::pallet::pallet::Event;
    }
    pub mod session {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetKeys {
                keys: __runtime_types::node_runtime::SessionKeys,
                proof: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct PurgeKeys {}
        }
        pub type Event = __runtime_types::pallet_session::Event;
    }
    pub mod democracy {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Propose {
                proposal_hash: __runtime_types::primitive_types::H256,
                value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Second {
                proposal: u32,
                seconds_upper_bound: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vote {
                ref_index: u32,
                vote: __runtime_types::pallet_democracy::vote::AccountVote<u128>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct EmergencyCancel {
                ref_index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExternalPropose {
                proposal_hash: __runtime_types::primitive_types::H256,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExternalProposeMajority {
                proposal_hash: __runtime_types::primitive_types::H256,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExternalProposeDefault {
                proposal_hash: __runtime_types::primitive_types::H256,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct FastTrack {
                proposal_hash: __runtime_types::primitive_types::H256,
                voting_period: u32,
                delay: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct VetoExternal {
                proposal_hash: __runtime_types::primitive_types::H256,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelReferendum {
                ref_index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelQueued {
                which: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Delegate {
                to: __runtime_types::sp_core::crypto::AccountId32,
                conviction: __runtime_types::pallet_democracy::conviction::Conviction,
                balance: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Undelegate {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearPublicProposals {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct NotePreimage {
                encoded_proposal: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct NotePreimageOperational {
                encoded_proposal: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct NoteImminentPreimage {
                encoded_proposal: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct NoteImminentPreimageOperational {
                encoded_proposal: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReapPreimage {
                proposal_hash: __runtime_types::primitive_types::H256,
                proposal_len_upper_bound: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unlock {
                target: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveVote {
                index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveOtherVote {
                target: __runtime_types::sp_core::crypto::AccountId32,
                index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct EnactProposal {
                proposal_hash: __runtime_types::primitive_types::H256,
                index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Blacklist {
                proposal_hash: __runtime_types::primitive_types::H256,
                maybe_ref_index: Option<u32>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelProposal {
                prop_index: u32,
            }
        }
        pub type Event = __runtime_types::pallet_democracy::pallet::Event;
    }
    pub mod council {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMembers {
                new_members: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                prime: Option<__runtime_types::sp_core::crypto::AccountId32>,
                old_count: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Execute {
                proposal: __runtime_types::node_runtime::Call,
                length_bound: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Propose {
                threshold: u32,
                proposal: __runtime_types::node_runtime::Call,
                length_bound: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vote {
                proposal: __runtime_types::primitive_types::H256,
                index: u32,
                approve: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Close {
                proposal_hash: __runtime_types::primitive_types::H256,
                index: u32,
                proposal_weight_bound: u64,
                length_bound: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct DisapproveProposal {
                proposal_hash: __runtime_types::primitive_types::H256,
            }
        }
        pub type Event = __runtime_types::pallet_collective::RawEvent<
            __runtime_types::primitive_types::H256,
            __runtime_types::sp_core::crypto::AccountId32,
            __runtime_types::pallet_collective::Instance1,
        >;
    }
    pub mod technical_committee {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMembers {
                new_members: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                prime: Option<__runtime_types::sp_core::crypto::AccountId32>,
                old_count: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Execute {
                proposal: __runtime_types::node_runtime::Call,
                length_bound: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Propose {
                threshold: u32,
                proposal: __runtime_types::node_runtime::Call,
                length_bound: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vote {
                proposal: __runtime_types::primitive_types::H256,
                index: u32,
                approve: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Close {
                proposal_hash: __runtime_types::primitive_types::H256,
                index: u32,
                proposal_weight_bound: u64,
                length_bound: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct DisapproveProposal {
                proposal_hash: __runtime_types::primitive_types::H256,
            }
        }
        pub type Event = __runtime_types::pallet_collective::RawEvent<
            __runtime_types::primitive_types::H256,
            __runtime_types::sp_core::crypto::AccountId32,
            __runtime_types::pallet_collective::Instance2,
        >;
    }
    pub mod elections {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vote {
                votes: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveVoter {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SubmitCandidacy {
                candidate_count: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RenounceCandidacy {
                renouncing: __runtime_types::pallet_elections_phragmen::Renouncing,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveMember {
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                has_replacement: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CleanDefunctVoters {
                num_voters: u32,
                num_defunct: u32,
            }
        }
        pub type Event = __runtime_types::pallet_elections_phragmen::pallet::Event;
    }
    pub mod technical_membership {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddMember {
                who: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveMember {
                who: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SwapMember {
                remove: __runtime_types::sp_core::crypto::AccountId32,
                add: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ResetMembers {
                members: Vec<__runtime_types::sp_core::crypto::AccountId32>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ChangeKey {
                new: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetPrime {
                who: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearPrime {}
        }
        pub type Event = __runtime_types::pallet_membership::RawEvent<
            __runtime_types::sp_core::crypto::AccountId32,
            __runtime_types::node_runtime::Event,
            __runtime_types::pallet_membership::Instance1,
        >;
    }
    pub mod grandpa {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportEquivocation {
                equivocation_proof: __runtime_types::sp_finality_grandpa::EquivocationProof<
                    __runtime_types::primitive_types::H256,
                    u32,
                >,
                key_owner_proof: __runtime_types::sp_session::MembershipProof,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                equivocation_proof: __runtime_types::sp_finality_grandpa::EquivocationProof<
                    __runtime_types::primitive_types::H256,
                    u32,
                >,
                key_owner_proof: __runtime_types::sp_session::MembershipProof,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct NoteStalled {
                delay: u32,
                best_finalized_block_number: u32,
            }
        }
        pub type Event = __runtime_types::pallet_grandpa::pallet::Event;
    }
    pub mod treasury {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProposeSpend {
                value: u128,
                beneficiary: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RejectProposal {
                proposal_id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ApproveProposal {
                proposal_id: u32,
            }
        }
        pub type Event = __runtime_types::pallet_treasury::pallet::Event;
    }
    pub mod contracts {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Call {
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                value: u128,
                gas_limit: u64,
                data: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct InstantiateWithCode {
                endowment: u128,
                gas_limit: u64,
                code: Vec<u8>,
                data: Vec<u8>,
                salt: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Instantiate {
                endowment: u128,
                gas_limit: u64,
                code_hash: __runtime_types::primitive_types::H256,
                data: Vec<u8>,
                salt: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClaimSurcharge {
                dest: __runtime_types::sp_core::crypto::AccountId32,
                aux_sender: Option<__runtime_types::sp_core::crypto::AccountId32>,
            }
        }
        pub type Event = __runtime_types::pallet_contracts::pallet::Event;
    }
    pub mod sudo {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Sudo {
                call: __runtime_types::node_runtime::Call,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SudoUncheckedWeight {
                call: __runtime_types::node_runtime::Call,
                weight: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetKey {
                new: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SudoAs {
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                call: __runtime_types::node_runtime::Call,
            }
        }
        pub type Event = __runtime_types::pallet_sudo::pallet::Event;
    }
    pub mod im_online {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Heartbeat {
                heartbeat: __runtime_types::pallet_im_online::Heartbeat<u32>,
                signature: __runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
            }
        }
        pub type Event = __runtime_types::pallet_im_online::pallet::Event;
    }
    pub mod authority_discovery {
        use super::__runtime_types;
    }
    pub mod offences {
        use super::__runtime_types;
        pub type Event = __runtime_types::pallet_offences::pallet::Event;
    }
    pub mod historical {
        use super::__runtime_types;
    }
    pub mod randomness_collective_flip {
        use super::__runtime_types;
    }
    pub mod identity {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddRegistrar {
                account: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetIdentity {
                info: __runtime_types::pallet_identity::types::IdentityInfo,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetSubs {
                subs: Vec<(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::pallet_identity::types::Data,
                )>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearIdentity {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RequestJudgement {
                reg_index: u32,
                max_fee: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelRequest {
                reg_index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetFee {
                index: u32,
                fee: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetAccountId {
                index: u32,
                new: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetFields {
                index: u32,
                fields: __runtime_types::pallet_identity::types::BitFlags<
                    __runtime_types::pallet_identity::types::IdentityField,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProvideJudgement {
                reg_index: u32,
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                judgement: __runtime_types::pallet_identity::types::Judgement<u128>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct KillIdentity {
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddSub {
                sub: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                data: __runtime_types::pallet_identity::types::Data,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RenameSub {
                sub: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                data: __runtime_types::pallet_identity::types::Data,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveSub {
                sub: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct QuitSub {}
        }
        pub type Event = __runtime_types::pallet_identity::pallet::Event;
    }
    pub mod society {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Bid {
                value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unbid {
                pos: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vouch {
                who: __runtime_types::sp_core::crypto::AccountId32,
                value: u128,
                tip: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unvouch {
                pos: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vote {
                candidate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                approve: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct DefenderVote {
                approve: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Payout {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Found {
                founder: __runtime_types::sp_core::crypto::AccountId32,
                max_members: u32,
                rules: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Unfound {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct JudgeSuspendedMember {
                who: __runtime_types::sp_core::crypto::AccountId32,
                forgive: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct JudgeSuspendedCandidate {
                who: __runtime_types::sp_core::crypto::AccountId32,
                judgement: __runtime_types::pallet_society::Judgement,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMaxMembers {
                max: u32,
            }
        }
        pub type Event = __runtime_types::pallet_society::RawEvent<
            __runtime_types::sp_core::crypto::AccountId32,
            u128,
            __runtime_types::pallet_society::DefaultInstance,
        >;
    }
    pub mod recovery {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AsRecovered {
                account: __runtime_types::sp_core::crypto::AccountId32,
                call: __runtime_types::node_runtime::Call,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetRecovered {
                lost: __runtime_types::sp_core::crypto::AccountId32,
                rescuer: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CreateRecovery {
                friends: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                threshold: u16,
                delay_period: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct InitiateRecovery {
                account: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct VouchRecovery {
                lost: __runtime_types::sp_core::crypto::AccountId32,
                rescuer: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClaimRecovery {
                account: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CloseRecovery {
                rescuer: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveRecovery {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelRecovered {
                account: __runtime_types::sp_core::crypto::AccountId32,
            }
        }
        pub type Event = __runtime_types::pallet_recovery::pallet::Event;
    }
    pub mod vesting {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Vest {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct VestOther {
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct VestedTransfer {
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                schedule: __runtime_types::pallet_vesting::vesting_info::VestingInfo<u128, u32>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceVestedTransfer {
                source: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                schedule: __runtime_types::pallet_vesting::vesting_info::VestingInfo<u128, u32>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct MergeSchedules {
                schedule1_index: u32,
                schedule2_index: u32,
            }
        }
        pub type Event = __runtime_types::pallet_vesting::pallet::Event;
    }
    pub mod scheduler {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Schedule {
                when: u32,
                maybe_periodic: Option<(u32, u32)>,
                priority: u8,
                call: __runtime_types::node_runtime::Call,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Cancel {
                when: u32,
                index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScheduleNamed {
                id: Vec<u8>,
                when: u32,
                maybe_periodic: Option<(u32, u32)>,
                priority: u8,
                call: __runtime_types::node_runtime::Call,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelNamed {
                id: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScheduleAfter {
                after: u32,
                maybe_periodic: Option<(u32, u32)>,
                priority: u8,
                call: __runtime_types::node_runtime::Call,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScheduleNamedAfter {
                id: Vec<u8>,
                after: u32,
                maybe_periodic: Option<(u32, u32)>,
                priority: u8,
                call: __runtime_types::node_runtime::Call,
            }
        }
        pub type Event = __runtime_types::pallet_scheduler::pallet::Event;
    }
    pub mod proxy {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Proxy {
                real: __runtime_types::sp_core::crypto::AccountId32,
                force_proxy_type: Option<__runtime_types::node_runtime::ProxyType>,
                call: __runtime_types::node_runtime::Call,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AddProxy {
                delegate: __runtime_types::sp_core::crypto::AccountId32,
                proxy_type: __runtime_types::node_runtime::ProxyType,
                delay: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveProxy {
                delegate: __runtime_types::sp_core::crypto::AccountId32,
                proxy_type: __runtime_types::node_runtime::ProxyType,
                delay: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveProxies {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Anonymous {
                proxy_type: __runtime_types::node_runtime::ProxyType,
                delay: u32,
                index: u16,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct KillAnonymous {
                spawner: __runtime_types::sp_core::crypto::AccountId32,
                proxy_type: __runtime_types::node_runtime::ProxyType,
                index: u16,
                height: u32,
                ext_index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Announce {
                real: __runtime_types::sp_core::crypto::AccountId32,
                call_hash: __runtime_types::primitive_types::H256,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RemoveAnnouncement {
                real: __runtime_types::sp_core::crypto::AccountId32,
                call_hash: __runtime_types::primitive_types::H256,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RejectAnnouncement {
                delegate: __runtime_types::sp_core::crypto::AccountId32,
                call_hash: __runtime_types::primitive_types::H256,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProxyAnnounced {
                delegate: __runtime_types::sp_core::crypto::AccountId32,
                real: __runtime_types::sp_core::crypto::AccountId32,
                force_proxy_type: Option<__runtime_types::node_runtime::ProxyType>,
                call: __runtime_types::node_runtime::Call,
            }
        }
        pub type Event = __runtime_types::pallet_proxy::pallet::Event;
    }
    pub mod multisig {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AsMultiThreshold1 {
                other_signatories: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                call: __runtime_types::node_runtime::Call,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AsMulti {
                threshold: u16,
                other_signatories: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                maybe_timepoint: Option<__runtime_types::pallet_multisig::Timepoint<u32>>,
                call: Vec<u8>,
                store_call: bool,
                max_weight: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ApproveAsMulti {
                threshold: u16,
                other_signatories: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                maybe_timepoint: Option<__runtime_types::pallet_multisig::Timepoint<u32>>,
                call_hash: [u8; 32usize],
                max_weight: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelAsMulti {
                threshold: u16,
                other_signatories: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                timepoint: __runtime_types::pallet_multisig::Timepoint<u32>,
                call_hash: [u8; 32usize],
            }
        }
        pub type Event = __runtime_types::pallet_multisig::pallet::Event;
    }
    pub mod bounties {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProposeBounty {
                value: u128,
                description: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ApproveBounty {
                bounty_id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProposeCurator {
                bounty_id: u32,
                curator: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                fee: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct UnassignCurator {
                bounty_id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AcceptCurator {
                bounty_id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AwardBounty {
                bounty_id: u32,
                beneficiary: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClaimBounty {
                bounty_id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CloseBounty {
                bounty_id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ExtendBountyExpiry {
                bounty_id: u32,
                _remark: Vec<u8>,
            }
        }
        pub type Event = __runtime_types::pallet_bounties::RawEvent<
            u128,
            __runtime_types::sp_core::crypto::AccountId32,
        >;
    }
    pub mod tips {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReportAwesome {
                reason: Vec<u8>,
                who: __runtime_types::sp_core::crypto::AccountId32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RetractTip {
                hash: __runtime_types::primitive_types::H256,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct TipNew {
                reason: Vec<u8>,
                who: __runtime_types::sp_core::crypto::AccountId32,
                tip_value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Tip {
                hash: __runtime_types::primitive_types::H256,
                tip_value: u128,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CloseTip {
                hash: __runtime_types::primitive_types::H256,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SlashTip {
                hash: __runtime_types::primitive_types::H256,
            }
        }
        pub type Event = __runtime_types::pallet_tips::RawEvent<
            u128,
            __runtime_types::sp_core::crypto::AccountId32,
            __runtime_types::primitive_types::H256,
        >;
    }
    pub mod assets {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Create {
                id: u32,
                admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                min_balance: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceCreate {
                id: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                is_sufficient: bool,
                min_balance: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Destroy {
                id: u32,
                witness: __runtime_types::pallet_assets::types::DestroyWitness,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Mint {
                id: u32,
                beneficiary: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Burn {
                id: u32,
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Transfer {
                id: u32,
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransferKeepAlive {
                id: u32,
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceTransfer {
                id: u32,
                source: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Freeze {
                id: u32,
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Thaw {
                id: u32,
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct FreezeAsset {
                id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ThawAsset {
                id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransferOwnership {
                id: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetTeam {
                id: u32,
                issuer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                freezer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMetadata {
                id: u32,
                name: Vec<u8>,
                symbol: Vec<u8>,
                decimals: u8,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearMetadata {
                id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceSetMetadata {
                id: u32,
                name: Vec<u8>,
                symbol: Vec<u8>,
                decimals: u8,
                is_frozen: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceClearMetadata {
                id: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceAssetStatus {
                id: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                issuer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                freezer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                min_balance: u64,
                is_sufficient: bool,
                is_frozen: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ApproveTransfer {
                id: u32,
                delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelApproval {
                id: u32,
                delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceCancelApproval {
                id: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransferApproved {
                id: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                destination: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
        }
        pub type Event = __runtime_types::pallet_assets::pallet::Event;
    }
    pub mod mmr {
        use super::__runtime_types;
    }
    pub mod lottery {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct BuyTicket {
                call: __runtime_types::node_runtime::Call,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetCalls {
                calls: Vec<__runtime_types::node_runtime::Call>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct StartLottery {
                price: u128,
                length: u32,
                delay: u32,
                repeat: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct StopRepeat {}
        }
        pub type Event = __runtime_types::pallet_lottery::pallet::Event;
    }
    pub mod gilt {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct PlaceBid {
                amount: u128,
                duration: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RetractBid {
                amount: u128,
                duration: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetTarget {
                target: __runtime_types::sp_arithmetic::per_things::Perquintill,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Thaw {
                index: u32,
            }
        }
        pub type Event = __runtime_types::pallet_gilt::pallet::Event;
    }
    pub mod uniques {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Create {
                class: u32,
                admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceCreate {
                class: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                free_holding: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Destroy {
                class: u32,
                witness: __runtime_types::pallet_uniques::types::DestroyWitness,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Mint {
                class: u32,
                instance: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Burn {
                class: u32,
                instance: u32,
                check_owner: Option<
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Transfer {
                class: u32,
                instance: u32,
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Redeposit {
                class: u32,
                instances: Vec<u32>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Freeze {
                class: u32,
                instance: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Thaw {
                class: u32,
                instance: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct FreezeClass {
                class: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ThawClass {
                class: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransferOwnership {
                class: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetTeam {
                class: u32,
                issuer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                freezer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ApproveTransfer {
                class: u32,
                instance: u32,
                delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CancelApproval {
                class: u32,
                instance: u32,
                maybe_check_delegate: Option<
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                >,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ForceAssetStatus {
                class: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                issuer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                freezer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                free_holding: bool,
                is_frozen: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetAttribute {
                class: u32,
                maybe_instance: Option<u32>,
                key: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                value: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearAttribute {
                class: u32,
                maybe_instance: Option<u32>,
                key: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetMetadata {
                class: u32,
                instance: u32,
                data: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                is_frozen: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearMetadata {
                class: u32,
                instance: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SetClassMetadata {
                class: u32,
                data: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                is_frozen: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ClearClassMetadata {
                class: u32,
            }
        }
        pub type Event = __runtime_types::pallet_uniques::pallet::Event;
    }
    pub mod transaction_storage {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Store {
                data: Vec<u8>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Renew {
                block: u32,
                index: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct CheckProof {
                proof: __runtime_types::sp_transaction_storage_proof::TransactionStorageProof,
            }
        }
        pub type Event = __runtime_types::pallet_transaction_storage::pallet::Event;
    }
    pub mod __runtime_types {
        use super::__runtime_types;
        pub mod finality_grandpa {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::__runtime_types;
            pub mod storage {
                use super::__runtime_types;
                pub mod bounded_btree_map {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct BoundedBTreeMap<_0, _1>(pub std::collections::BTreeMap<_0, _1>);
                }
                pub mod bounded_vec {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct BoundedVec<_0>(pub Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct WeakBoundedVec<_0>(pub Vec<_0>);
                }
            }
            pub mod traits {
                use super::__runtime_types;
                pub mod tokens {
                    use super::__runtime_types;
                    pub mod misc {
                        use super::__runtime_types;
                        #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                        pub enum BalanceStatus {
                            Free,
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum DispatchClass {
                    Normal,
                    Operational,
                    Mandatory,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct DispatchInfo {
                    pub weight: u64,
                    pub class: __runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: __runtime_types::frame_support::weights::Pays,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Pays {
                    Yes,
                    No,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct RuntimeDbWeight {
                    pub read: u64,
                    pub write: u64,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: __runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: bool,
                    pub degree: u8,
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct PalletId(pub [u8; 8usize]);
        }
        pub mod frame_system {
            use super::__runtime_types;
            pub mod extensions {
                use super::__runtime_types;
                pub mod check_genesis {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct CheckGenesis {}
                }
                pub mod check_mortality {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct CheckMortality(pub __runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_nonce {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct CheckNonce(pub u32);
                }
                pub mod check_spec_version {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct CheckSpecVersion {}
                }
                pub mod check_tx_version {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct CheckTxVersion {}
                }
                pub mod check_weight {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct CheckWeight {}
                }
            }
            pub mod limits {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct BlockLength {
                    pub max: __runtime_types::frame_support::weights::PerDispatchClass<u32>,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct BlockWeights {
                    pub base_block: u64,
                    pub max_block: u64,
                    pub per_class: __runtime_types::frame_support::weights::PerDispatchClass<
                        __runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: u64,
                    pub max_extrinsic: Option<u64>,
                    pub max_total: Option<u64>,
                    pub reserved: Option<u64>,
                }
            }
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    fill_block {
                        ratio: __runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    remark {
                        remark: Vec<u8>,
                    },
                    set_heap_pages {
                        pages: u64,
                    },
                    set_code {
                        code: Vec<u8>,
                    },
                    set_code_without_checks {
                        code: Vec<u8>,
                    },
                    set_changes_trie_config {
                        changes_trie_config: Option<
                            __runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                        >,
                    },
                    set_storage {
                        items: Vec<(Vec<u8>, Vec<u8>)>,
                    },
                    kill_storage {
                        keys: Vec<Vec<u8>>,
                    },
                    kill_prefix {
                        prefix: Vec<u8>,
                        subkeys: u32,
                    },
                    remark_with_event {
                        remark: Vec<u8>,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    InvalidSpecName,
                    SpecVersionNeedsToIncrease,
                    FailedToExtractRuntimeVersion,
                    NonDefaultComposite,
                    NonZeroRefCount,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    ExtrinsicSuccess(__runtime_types::frame_support::weights::DispatchInfo),
                    ExtrinsicFailed(
                        __runtime_types::sp_runtime::DispatchError,
                        __runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    CodeUpdated,
                    NewAccount(__runtime_types::sp_core::crypto::AccountId32),
                    KilledAccount(__runtime_types::sp_core::crypto::AccountId32),
                    Remarked(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                    ),
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct EventRecord<_0, _1> {
                pub phase: __runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: Vec<_1>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct LastRuntimeUpgradeInfo {
                pub spec_version: u32,
                pub spec_name: String,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Phase {
                ApplyExtrinsic(u32),
                Finalization,
                Initialization,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum RawOrigin<_0> {
                Root,
                Signed(_0),
                None,
            }
        }
        pub mod node_runtime {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Call {
                System(__runtime_types::frame_system::pallet::Call),
                Utility(__runtime_types::pallet_utility::pallet::Call),
                Babe(__runtime_types::pallet_babe::pallet::Call),
                Timestamp(__runtime_types::pallet_timestamp::pallet::Call),
                Authorship(__runtime_types::pallet_authorship::pallet::Call),
                Indices(__runtime_types::pallet_indices::pallet::Call),
                Balances(__runtime_types::pallet_balances::pallet::Call),
                ElectionProviderMultiPhase(
                    __runtime_types::pallet_election_provider_multi_phase::pallet::Call,
                ),
                Staking(__runtime_types::pallet_staking::pallet::pallet::Call),
                Session(__runtime_types::pallet_session::Call),
                Democracy(__runtime_types::pallet_democracy::pallet::Call),
                Council(__runtime_types::pallet_collective::Call),
                TechnicalCommittee(__runtime_types::pallet_collective::Call),
                Elections(__runtime_types::pallet_elections_phragmen::pallet::Call),
                TechnicalMembership(__runtime_types::pallet_membership::Call),
                Grandpa(__runtime_types::pallet_grandpa::pallet::Call),
                Treasury(__runtime_types::pallet_treasury::pallet::Call),
                Contracts(__runtime_types::pallet_contracts::pallet::Call),
                Sudo(__runtime_types::pallet_sudo::pallet::Call),
                ImOnline(__runtime_types::pallet_im_online::pallet::Call),
                Identity(__runtime_types::pallet_identity::pallet::Call),
                Society(__runtime_types::pallet_society::Call),
                Recovery(__runtime_types::pallet_recovery::pallet::Call),
                Vesting(__runtime_types::pallet_vesting::pallet::Call),
                Scheduler(__runtime_types::pallet_scheduler::pallet::Call),
                Proxy(__runtime_types::pallet_proxy::pallet::Call),
                Multisig(__runtime_types::pallet_multisig::pallet::Call),
                Bounties(__runtime_types::pallet_bounties::Call),
                Tips(__runtime_types::pallet_tips::Call),
                Assets(__runtime_types::pallet_assets::pallet::Call),
                Lottery(__runtime_types::pallet_lottery::pallet::Call),
                Gilt(__runtime_types::pallet_gilt::pallet::Call),
                Uniques(__runtime_types::pallet_uniques::pallet::Call),
                TransactionStorage(__runtime_types::pallet_transaction_storage::pallet::Call),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Event {
                System(__runtime_types::frame_system::pallet::Event),
                Utility(__runtime_types::pallet_utility::pallet::Event),
                Indices(__runtime_types::pallet_indices::pallet::Event),
                Balances(__runtime_types::pallet_balances::pallet::Event),
                ElectionProviderMultiPhase(
                    __runtime_types::pallet_election_provider_multi_phase::pallet::Event,
                ),
                Staking(__runtime_types::pallet_staking::pallet::pallet::Event),
                Session(__runtime_types::pallet_session::Event),
                Democracy(__runtime_types::pallet_democracy::pallet::Event),
                Council(
                    __runtime_types::pallet_collective::RawEvent<
                        __runtime_types::primitive_types::H256,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_collective::Instance1,
                    >,
                ),
                TechnicalCommittee(
                    __runtime_types::pallet_collective::RawEvent<
                        __runtime_types::primitive_types::H256,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_collective::Instance2,
                    >,
                ),
                Elections(__runtime_types::pallet_elections_phragmen::pallet::Event),
                TechnicalMembership(
                    __runtime_types::pallet_membership::RawEvent<
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::node_runtime::Event,
                        __runtime_types::pallet_membership::Instance1,
                    >,
                ),
                Grandpa(__runtime_types::pallet_grandpa::pallet::Event),
                Treasury(__runtime_types::pallet_treasury::pallet::Event),
                Contracts(__runtime_types::pallet_contracts::pallet::Event),
                Sudo(__runtime_types::pallet_sudo::pallet::Event),
                ImOnline(__runtime_types::pallet_im_online::pallet::Event),
                Offences(__runtime_types::pallet_offences::pallet::Event),
                Identity(__runtime_types::pallet_identity::pallet::Event),
                Society(
                    __runtime_types::pallet_society::RawEvent<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                        __runtime_types::pallet_society::DefaultInstance,
                    >,
                ),
                Recovery(__runtime_types::pallet_recovery::pallet::Event),
                Vesting(__runtime_types::pallet_vesting::pallet::Event),
                Scheduler(__runtime_types::pallet_scheduler::pallet::Event),
                Proxy(__runtime_types::pallet_proxy::pallet::Event),
                Multisig(__runtime_types::pallet_multisig::pallet::Event),
                Bounties(
                    __runtime_types::pallet_bounties::RawEvent<
                        u128,
                        __runtime_types::sp_core::crypto::AccountId32,
                    >,
                ),
                Tips(
                    __runtime_types::pallet_tips::RawEvent<
                        u128,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                    >,
                ),
                Assets(__runtime_types::pallet_assets::pallet::Event),
                Lottery(__runtime_types::pallet_lottery::pallet::Event),
                Gilt(__runtime_types::pallet_gilt::pallet::Event),
                Uniques(__runtime_types::pallet_uniques::pallet::Event),
                TransactionStorage(__runtime_types::pallet_transaction_storage::pallet::Event),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct NposSolution16 {
                votes1: Vec<(u32, u16)>,
                votes2: Vec<(
                    u32,
                    (u16, __runtime_types::sp_arithmetic::per_things::PerU16),
                    u16,
                )>,
                votes3: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 2usize],
                    u16,
                )>,
                votes4: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 3usize],
                    u16,
                )>,
                votes5: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 4usize],
                    u16,
                )>,
                votes6: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 5usize],
                    u16,
                )>,
                votes7: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 6usize],
                    u16,
                )>,
                votes8: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 7usize],
                    u16,
                )>,
                votes9: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 8usize],
                    u16,
                )>,
                votes10: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 9usize],
                    u16,
                )>,
                votes11: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 10usize],
                    u16,
                )>,
                votes12: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 11usize],
                    u16,
                )>,
                votes13: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 12usize],
                    u16,
                )>,
                votes14: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 13usize],
                    u16,
                )>,
                votes15: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 14usize],
                    u16,
                )>,
                votes16: Vec<(
                    u32,
                    [(u16, __runtime_types::sp_arithmetic::per_things::PerU16); 15usize],
                    u16,
                )>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum OriginCaller {
                system(
                    __runtime_types::frame_system::RawOrigin<
                        __runtime_types::sp_core::crypto::AccountId32,
                    >,
                ),
                Council(
                    __runtime_types::pallet_collective::RawOrigin<
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_collective::Instance1,
                    >,
                ),
                TechnicalCommittee(
                    __runtime_types::pallet_collective::RawOrigin<
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_collective::Instance2,
                    >,
                ),
                Void(__runtime_types::sp_core::Void),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum ProxyType {
                Any,
                NonTransfer,
                Governance,
                Staking,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SessionKeys {
                pub grandpa: __runtime_types::sp_finality_grandpa::app::Public,
                pub babe: __runtime_types::sp_consensus_babe::app::Public,
                pub im_online: __runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                pub authority_discovery: __runtime_types::sp_authority_discovery::app::Public,
            }
        }
        pub mod pallet_assets {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    create {
                        id: u32,
                        admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        min_balance: u64,
                    },
                    force_create {
                        id: u32,
                        owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        is_sufficient: bool,
                        min_balance: u64,
                    },
                    destroy {
                        id: u32,
                        witness: __runtime_types::pallet_assets::types::DestroyWitness,
                    },
                    mint {
                        id: u32,
                        beneficiary: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        amount: u64,
                    },
                    burn {
                        id: u32,
                        who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        amount: u64,
                    },
                    transfer {
                        id: u32,
                        target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        amount: u64,
                    },
                    transfer_keep_alive {
                        id: u32,
                        target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        amount: u64,
                    },
                    force_transfer {
                        id: u32,
                        source: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        amount: u64,
                    },
                    freeze {
                        id: u32,
                        who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    thaw {
                        id: u32,
                        who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    freeze_asset {
                        id: u32,
                    },
                    thaw_asset {
                        id: u32,
                    },
                    transfer_ownership {
                        id: u32,
                        owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    set_team {
                        id: u32,
                        issuer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        freezer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    set_metadata {
                        id: u32,
                        name: Vec<u8>,
                        symbol: Vec<u8>,
                        decimals: u8,
                    },
                    clear_metadata {
                        id: u32,
                    },
                    force_set_metadata {
                        id: u32,
                        name: Vec<u8>,
                        symbol: Vec<u8>,
                        decimals: u8,
                        is_frozen: bool,
                    },
                    force_clear_metadata {
                        id: u32,
                    },
                    force_asset_status {
                        id: u32,
                        owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        issuer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        freezer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        min_balance: u64,
                        is_sufficient: bool,
                        is_frozen: bool,
                    },
                    approve_transfer {
                        id: u32,
                        delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        amount: u64,
                    },
                    cancel_approval {
                        id: u32,
                        delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    force_cancel_approval {
                        id: u32,
                        owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    transfer_approved {
                        id: u32,
                        owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        destination: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        amount: u64,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    BalanceLow,
                    BalanceZero,
                    NoPermission,
                    Unknown,
                    Frozen,
                    InUse,
                    BadWitness,
                    MinBalanceZero,
                    NoProvider,
                    BadMetadata,
                    Unapproved,
                    WouldDie,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Created(
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    Issued(u32, __runtime_types::sp_core::crypto::AccountId32, u64),
                    Transferred(
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u64,
                    ),
                    Burned(u32, __runtime_types::sp_core::crypto::AccountId32, u64),
                    TeamChanged(
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    OwnerChanged(u32, __runtime_types::sp_core::crypto::AccountId32),
                    Frozen(u32, __runtime_types::sp_core::crypto::AccountId32),
                    Thawed(u32, __runtime_types::sp_core::crypto::AccountId32),
                    AssetFrozen(u32),
                    AssetThawed(u32),
                    Destroyed(u32),
                    ForceCreated(u32, __runtime_types::sp_core::crypto::AccountId32),
                    MetadataSet(u32, Vec<u8>, Vec<u8>, u8, bool),
                    MetadataCleared(u32),
                    ApprovedTransfer(
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u64,
                    ),
                    ApprovalCancelled(
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    TransferredApproved(
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u64,
                    ),
                    AssetStatusChanged(u32),
                }
            }
            pub mod types {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _1,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct AssetBalance<_0, _1> {
                    pub balance: _0,
                    pub is_frozen: bool,
                    pub sufficient: bool,
                    pub extra: _1,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _2,
                    pub min_balance: _0,
                    pub is_sufficient: bool,
                    pub accounts: u32,
                    pub sufficients: u32,
                    pub approvals: u32,
                    pub is_frozen: bool,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: u8,
                    pub is_frozen: bool,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct DestroyWitness {
                    pub accounts: u32,
                    pub sufficients: u32,
                    pub approvals: u32,
                }
            }
        }
        pub mod pallet_authorship {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    set_uncles {
                        new_uncles: Vec<
                            __runtime_types::sp_runtime::generic::header::Header<
                                u32,
                                __runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    InvalidUncleParent,
                    UnclesAlreadySet,
                    TooManyUncles,
                    GenesisUncle,
                    TooHighUncle,
                    UncleAlreadyIncluded,
                    OldUncle,
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum UncleEntryItem<_0, _1, _2> {
                InclusionHeight(_0),
                Uncle(_1, Option<_2>),
            }
        }
        pub mod pallet_babe {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    report_equivocation {
                        equivocation_proof: std::boxed::Box<
                            __runtime_types::sp_consensus_slots::EquivocationProof<
                                __runtime_types::sp_runtime::generic::header::Header<
                                    u32,
                                    __runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                __runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: __runtime_types::sp_session::MembershipProof,
                    },
                    report_equivocation_unsigned {
                        equivocation_proof: std::boxed::Box<
                            __runtime_types::sp_consensus_slots::EquivocationProof<
                                __runtime_types::sp_runtime::generic::header::Header<
                                    u32,
                                    __runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                __runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: __runtime_types::sp_session::MembershipProof,
                    },
                    plan_config_change {
                        config: __runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    InvalidEquivocationProof,
                    InvalidKeyOwnershipProof,
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_balances {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    transfer {
                        dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        value: u128,
                    },
                    set_balance {
                        who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        new_free: u128,
                        new_reserved: u128,
                    },
                    force_transfer {
                        source: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        value: u128,
                    },
                    transfer_keep_alive {
                        dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        value: u128,
                    },
                    transfer_all {
                        dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        keep_alive: bool,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    VestingBalance,
                    LiquidityRestrictions,
                    InsufficientBalance,
                    ExistentialDeposit,
                    KeepAlive,
                    ExistingVestingSchedule,
                    DeadAccount,
                    TooManyReserves,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Endowed(__runtime_types::sp_core::crypto::AccountId32, u128),
                    DustLost(__runtime_types::sp_core::crypto::AccountId32, u128),
                    Transfer(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    BalanceSet(__runtime_types::sp_core::crypto::AccountId32, u128, u128),
                    Deposit(__runtime_types::sp_core::crypto::AccountId32, u128),
                    Reserved(__runtime_types::sp_core::crypto::AccountId32, u128),
                    Unreserved(__runtime_types::sp_core::crypto::AccountId32, u128),
                    ReserveRepatriated(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                        __runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    ),
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct BalanceLock<_0> {
                pub id: [u8; 8usize],
                pub amount: _0,
                pub reasons: __runtime_types::pallet_balances::Reasons,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Reasons {
                Fee,
                Misc,
                All,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V1_0_0,
                V2_0_0,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_bounties {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Bounty<_0, _1, _2> {
                pub proposer: _0,
                pub value: _1,
                pub fee: _1,
                pub curator_deposit: _1,
                pub bond: _1,
                pub status: __runtime_types::pallet_bounties::BountyStatus<_0, _2>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum BountyStatus<_0, _1> {
                Proposed,
                Approved,
                Funded,
                CuratorProposed {
                    curator: _0,
                },
                Active {
                    curator: _0,
                    update_due: _1,
                },
                PendingPayout {
                    curator: _0,
                    beneficiary: _0,
                    unlock_at: _1,
                },
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Call {
                propose_bounty {
                    value: u128,
                    description: Vec<u8>,
                },
                approve_bounty {
                    bounty_id: u32,
                },
                propose_curator {
                    bounty_id: u32,
                    curator: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    fee: u128,
                },
                unassign_curator {
                    bounty_id: u32,
                },
                accept_curator {
                    bounty_id: u32,
                },
                award_bounty {
                    bounty_id: u32,
                    beneficiary: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                },
                claim_bounty {
                    bounty_id: u32,
                },
                close_bounty {
                    bounty_id: u32,
                },
                extend_bounty_expiry {
                    bounty_id: u32,
                    _remark: Vec<u8>,
                },
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Error {
                InsufficientProposersBalance,
                InvalidIndex,
                ReasonTooBig,
                UnexpectedStatus,
                RequireCurator,
                InvalidValue,
                InvalidFee,
                PendingPayout,
                Premature,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum RawEvent<_0, _1> {
                BountyProposed(u32),
                BountyRejected(u32, _0),
                BountyBecameActive(u32),
                BountyAwarded(u32, _1),
                BountyClaimed(u32, _0, _1),
                BountyCanceled(u32),
                BountyExtended(u32),
            }
        }
        pub mod pallet_collective {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Call {
                set_members {
                    new_members: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                    prime: Option<__runtime_types::sp_core::crypto::AccountId32>,
                    old_count: u32,
                },
                execute {
                    proposal: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    length_bound: u32,
                },
                propose {
                    threshold: u32,
                    proposal: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    length_bound: u32,
                },
                vote {
                    proposal: __runtime_types::primitive_types::H256,
                    index: u32,
                    approve: bool,
                },
                close {
                    proposal_hash: __runtime_types::primitive_types::H256,
                    index: u32,
                    proposal_weight_bound: u64,
                    length_bound: u32,
                },
                disapprove_proposal {
                    proposal_hash: __runtime_types::primitive_types::H256,
                },
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Error {
                NotMember,
                DuplicateProposal,
                ProposalMissing,
                WrongIndex,
                DuplicateVote,
                AlreadyInitialized,
                TooEarly,
                TooManyProposals,
                WrongProposalWeight,
                WrongProposalLength,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Instance1 {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Instance2 {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum RawEvent<_0, _1, _2> {
                Proposed(_1, u32, _0, u32),
                Voted(_1, _0, bool, u32, u32),
                Approved(_0),
                Disapproved(_0),
                Executed(_0, Result<(), __runtime_types::sp_runtime::DispatchError>),
                MemberExecuted(_0, Result<(), __runtime_types::sp_runtime::DispatchError>),
                Closed(_0, u32, u32),
                __Ignore(core::marker::PhantomData<(_2,)>),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum RawOrigin<_0, _1> {
                Members(u32, u32),
                Member(_0),
                _Phantom,
                __Ignore(core::marker::PhantomData<(_1,)>),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Votes<_0, _1> {
                pub index: _1,
                pub threshold: _1,
                pub ayes: Vec<_0>,
                pub nays: Vec<_0>,
                pub end: _1,
            }
        }
        pub mod pallet_contracts {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    call {
                        dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        value: u128,
                        gas_limit: u64,
                        data: Vec<u8>,
                    },
                    instantiate_with_code {
                        endowment: u128,
                        gas_limit: u64,
                        code: Vec<u8>,
                        data: Vec<u8>,
                        salt: Vec<u8>,
                    },
                    instantiate {
                        endowment: u128,
                        gas_limit: u64,
                        code_hash: __runtime_types::primitive_types::H256,
                        data: Vec<u8>,
                        salt: Vec<u8>,
                    },
                    claim_surcharge {
                        dest: __runtime_types::sp_core::crypto::AccountId32,
                        aux_sender: Option<__runtime_types::sp_core::crypto::AccountId32>,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    InvalidScheduleVersion,
                    InvalidSurchargeClaim,
                    InvalidSourceContract,
                    InvalidDestinationContract,
                    InvalidTombstone,
                    InvalidContractOrigin,
                    OutOfGas,
                    OutputBufferTooSmall,
                    BelowSubsistenceThreshold,
                    NewContractNotFunded,
                    TransferFailed,
                    MaxCallDepthReached,
                    ContractNotFound,
                    ContractIsTombstone,
                    RentNotPaid,
                    CodeTooLarge,
                    CodeNotFound,
                    OutOfBounds,
                    DecodingFailed,
                    ContractTrapped,
                    ValueTooLarge,
                    TerminatedWhileReentrant,
                    InputForwarded,
                    RandomSubjectTooLong,
                    TooManyTopics,
                    DuplicateTopics,
                    NoChainExtension,
                    DeletionQueueFull,
                    ContractNotEvictable,
                    StorageExhausted,
                    DuplicateContract,
                    TerminatedInConstructor,
                    DebugMessageInvalidUTF8,
                    ReentranceDenied,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Instantiated(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    Evicted(__runtime_types::sp_core::crypto::AccountId32),
                    Terminated(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    Restored(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                        u128,
                    ),
                    CodeStored(__runtime_types::primitive_types::H256),
                    ScheduleUpdated(u32),
                    ContractEmitted(__runtime_types::sp_core::crypto::AccountId32, Vec<u8>),
                    CodeRemoved(__runtime_types::primitive_types::H256),
                }
            }
            pub mod schedule {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct HostFnWeights {
                    pub caller: u64,
                    pub address: u64,
                    pub gas_left: u64,
                    pub balance: u64,
                    pub value_transferred: u64,
                    pub minimum_balance: u64,
                    pub tombstone_deposit: u64,
                    pub rent_allowance: u64,
                    pub block_number: u64,
                    pub now: u64,
                    pub weight_to_fee: u64,
                    pub gas: u64,
                    pub input: u64,
                    pub input_per_byte: u64,
                    pub r#return: u64,
                    pub return_per_byte: u64,
                    pub terminate: u64,
                    pub restore_to: u64,
                    pub restore_to_per_delta: u64,
                    pub random: u64,
                    pub deposit_event: u64,
                    pub deposit_event_per_topic: u64,
                    pub deposit_event_per_byte: u64,
                    pub debug_message: u64,
                    pub set_rent_allowance: u64,
                    pub set_storage: u64,
                    pub set_storage_per_byte: u64,
                    pub clear_storage: u64,
                    pub get_storage: u64,
                    pub get_storage_per_byte: u64,
                    pub transfer: u64,
                    pub call: u64,
                    pub call_transfer_surcharge: u64,
                    pub call_per_input_byte: u64,
                    pub call_per_output_byte: u64,
                    pub instantiate: u64,
                    pub instantiate_per_input_byte: u64,
                    pub instantiate_per_output_byte: u64,
                    pub instantiate_per_salt_byte: u64,
                    pub hash_sha2_256: u64,
                    pub hash_sha2_256_per_byte: u64,
                    pub hash_keccak_256: u64,
                    pub hash_keccak_256_per_byte: u64,
                    pub hash_blake2_256: u64,
                    pub hash_blake2_256_per_byte: u64,
                    pub hash_blake2_128: u64,
                    pub hash_blake2_128_per_byte: u64,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct InstructionWeights {
                    pub version: u32,
                    pub i64const: u32,
                    pub i64load: u32,
                    pub i64store: u32,
                    pub select: u32,
                    pub r#if: u32,
                    pub br: u32,
                    pub br_if: u32,
                    pub br_table: u32,
                    pub br_table_per_entry: u32,
                    pub call: u32,
                    pub call_indirect: u32,
                    pub call_indirect_per_param: u32,
                    pub local_get: u32,
                    pub local_set: u32,
                    pub local_tee: u32,
                    pub global_get: u32,
                    pub global_set: u32,
                    pub memory_current: u32,
                    pub memory_grow: u32,
                    pub i64clz: u32,
                    pub i64ctz: u32,
                    pub i64popcnt: u32,
                    pub i64eqz: u32,
                    pub i64extendsi32: u32,
                    pub i64extendui32: u32,
                    pub i32wrapi64: u32,
                    pub i64eq: u32,
                    pub i64ne: u32,
                    pub i64lts: u32,
                    pub i64ltu: u32,
                    pub i64gts: u32,
                    pub i64gtu: u32,
                    pub i64les: u32,
                    pub i64leu: u32,
                    pub i64ges: u32,
                    pub i64geu: u32,
                    pub i64add: u32,
                    pub i64sub: u32,
                    pub i64mul: u32,
                    pub i64divs: u32,
                    pub i64divu: u32,
                    pub i64rems: u32,
                    pub i64remu: u32,
                    pub i64and: u32,
                    pub i64or: u32,
                    pub i64xor: u32,
                    pub i64shl: u32,
                    pub i64shrs: u32,
                    pub i64shru: u32,
                    pub i64rotl: u32,
                    pub i64rotr: u32,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Limits {
                    pub event_topics: u32,
                    pub stack_height: u32,
                    pub globals: u32,
                    pub parameters: u32,
                    pub memory_pages: u32,
                    pub table_size: u32,
                    pub br_table_size: u32,
                    pub subject_len: u32,
                    pub call_depth: u32,
                    pub payload_len: u32,
                    pub code_len: u32,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Schedule {
                    pub limits: __runtime_types::pallet_contracts::schedule::Limits,
                    pub instruction_weights:
                        __runtime_types::pallet_contracts::schedule::InstructionWeights,
                    pub host_fn_weights: __runtime_types::pallet_contracts::schedule::HostFnWeights,
                }
            }
            pub mod storage {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum ContractInfo {
                    Alive(
                        __runtime_types::pallet_contracts::storage::RawAliveContractInfo<
                            __runtime_types::primitive_types::H256,
                            u128,
                            u32,
                        >,
                    ),
                    Tombstone(
                        __runtime_types::pallet_contracts::storage::RawTombstoneContractInfo<
                            __runtime_types::primitive_types::H256,
                            __runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    ),
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct DeletedContract {
                    pub pair_count: u32,
                    pub trie_id: Vec<u8>,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct RawAliveContractInfo<_0, _1, _2> {
                    pub trie_id: Vec<u8>,
                    pub storage_size: _2,
                    pub pair_count: _2,
                    pub code_hash: _0,
                    pub rent_allowance: _1,
                    pub rent_paid: _1,
                    pub deduct_block: _2,
                    pub last_write: Option<_2>,
                    pub _reserved: Option<()>,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct RawTombstoneContractInfo<_0, _1>(
                    pub _0,
                    pub core::marker::PhantomData<(_1)>,
                );
            }
            pub mod wasm {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct PrefabWasmModule {
                    pub instruction_weights_version: u32,
                    pub initial: u32,
                    pub maximum: u32,
                    pub refcount: u64,
                    pub _reserved: Option<()>,
                    pub code: Vec<u8>,
                    pub original_code_len: u32,
                }
            }
        }
        pub mod pallet_democracy {
            use super::__runtime_types;
            pub mod conviction {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Conviction {
                    None,
                    Locked1x,
                    Locked2x,
                    Locked3x,
                    Locked4x,
                    Locked5x,
                    Locked6x,
                }
            }
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    propose {
                        proposal_hash: __runtime_types::primitive_types::H256,
                        value: u128,
                    },
                    second {
                        proposal: u32,
                        seconds_upper_bound: u32,
                    },
                    vote {
                        ref_index: u32,
                        vote: __runtime_types::pallet_democracy::vote::AccountVote<u128>,
                    },
                    emergency_cancel {
                        ref_index: u32,
                    },
                    external_propose {
                        proposal_hash: __runtime_types::primitive_types::H256,
                    },
                    external_propose_majority {
                        proposal_hash: __runtime_types::primitive_types::H256,
                    },
                    external_propose_default {
                        proposal_hash: __runtime_types::primitive_types::H256,
                    },
                    fast_track {
                        proposal_hash: __runtime_types::primitive_types::H256,
                        voting_period: u32,
                        delay: u32,
                    },
                    veto_external {
                        proposal_hash: __runtime_types::primitive_types::H256,
                    },
                    cancel_referendum {
                        ref_index: u32,
                    },
                    cancel_queued {
                        which: u32,
                    },
                    delegate {
                        to: __runtime_types::sp_core::crypto::AccountId32,
                        conviction: __runtime_types::pallet_democracy::conviction::Conviction,
                        balance: u128,
                    },
                    undelegate,
                    clear_public_proposals,
                    note_preimage {
                        encoded_proposal: Vec<u8>,
                    },
                    note_preimage_operational {
                        encoded_proposal: Vec<u8>,
                    },
                    note_imminent_preimage {
                        encoded_proposal: Vec<u8>,
                    },
                    note_imminent_preimage_operational {
                        encoded_proposal: Vec<u8>,
                    },
                    reap_preimage {
                        proposal_hash: __runtime_types::primitive_types::H256,
                        proposal_len_upper_bound: u32,
                    },
                    unlock {
                        target: __runtime_types::sp_core::crypto::AccountId32,
                    },
                    remove_vote {
                        index: u32,
                    },
                    remove_other_vote {
                        target: __runtime_types::sp_core::crypto::AccountId32,
                        index: u32,
                    },
                    enact_proposal {
                        proposal_hash: __runtime_types::primitive_types::H256,
                        index: u32,
                    },
                    blacklist {
                        proposal_hash: __runtime_types::primitive_types::H256,
                        maybe_ref_index: Option<u32>,
                    },
                    cancel_proposal {
                        prop_index: u32,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    ValueLow,
                    ProposalMissing,
                    AlreadyCanceled,
                    DuplicateProposal,
                    ProposalBlacklisted,
                    NotSimpleMajority,
                    InvalidHash,
                    NoProposal,
                    AlreadyVetoed,
                    DuplicatePreimage,
                    NotImminent,
                    TooEarly,
                    Imminent,
                    PreimageMissing,
                    ReferendumInvalid,
                    PreimageInvalid,
                    NoneWaiting,
                    NotVoter,
                    NoPermission,
                    AlreadyDelegating,
                    InsufficientFunds,
                    NotDelegating,
                    VotesExist,
                    InstantNotAllowed,
                    Nonsense,
                    WrongUpperBound,
                    MaxVotesReached,
                    TooManyProposals,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Proposed(u32, u128),
                    Tabled(
                        u32,
                        u128,
                        Vec<__runtime_types::sp_core::crypto::AccountId32>,
                    ),
                    ExternalTabled,
                    Started(
                        u32,
                        __runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                    ),
                    Passed(u32),
                    NotPassed(u32),
                    Cancelled(u32),
                    Executed(u32, Result<(), __runtime_types::sp_runtime::DispatchError>),
                    Delegated(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    Undelegated(__runtime_types::sp_core::crypto::AccountId32),
                    Vetoed(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                        u32,
                    ),
                    PreimageNoted(
                        __runtime_types::primitive_types::H256,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    PreimageUsed(
                        __runtime_types::primitive_types::H256,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    PreimageInvalid(__runtime_types::primitive_types::H256, u32),
                    PreimageMissing(__runtime_types::primitive_types::H256, u32),
                    PreimageReaped(
                        __runtime_types::primitive_types::H256,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    Blacklisted(__runtime_types::primitive_types::H256),
                }
            }
            pub mod types {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Delegations<_0> {
                    pub votes: _0,
                    pub capital: _0,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum ReferendumInfo<_0, _1, _2> {
                    Ongoing(__runtime_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
                    Finished { approved: bool, end: _0 },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct ReferendumStatus<_0, _1, _2> {
                    pub end: _0,
                    pub proposal_hash: _1,
                    pub threshold: __runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                    pub delay: _0,
                    pub tally: __runtime_types::pallet_democracy::types::Tally<_2>,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Tally<_0> {
                    pub ayes: _0,
                    pub nays: _0,
                    pub turnout: _0,
                }
            }
            pub mod vote {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum AccountVote<_0> {
                    Standard {
                        vote: __runtime_types::pallet_democracy::vote::Vote,
                        balance: _0,
                    },
                    Split {
                        aye: _0,
                        nay: _0,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct PriorLock<_0, _1>(pub _0, pub _1);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Vote(u8);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Voting<_0, _1, _2> {
                    Direct {
                        votes: Vec<(_2, __runtime_types::pallet_democracy::vote::AccountVote<_0>)>,
                        delegations: __runtime_types::pallet_democracy::types::Delegations<_0>,
                        prior: __runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
                    },
                    Delegating {
                        balance: _0,
                        target: _1,
                        conviction: __runtime_types::pallet_democracy::conviction::Conviction,
                        delegations: __runtime_types::pallet_democracy::types::Delegations<_0>,
                        prior: __runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
                    },
                }
            }
            pub mod vote_threshold {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum VoteThreshold {
                    SuperMajorityApprove,
                    SuperMajorityAgainst,
                    SimpleMajority,
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum PreimageStatus<_0, _1, _2> {
                Missing(_2),
                Available {
                    data: Vec<u8>,
                    provider: _0,
                    deposit: _1,
                    since: _2,
                    expiry: Option<_2>,
                },
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V1,
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    submit_unsigned { raw_solution : std :: boxed :: Box < __runtime_types :: pallet_election_provider_multi_phase :: RawSolution < __runtime_types :: node_runtime :: NposSolution16 > > , witness : __runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , set_minimum_untrusted_score { maybe_next_score : Option < [u128 ; 3usize] > , } , set_emergency_election_result { supports : Vec < (__runtime_types :: sp_core :: crypto :: AccountId32 , __runtime_types :: sp_npos_elections :: Support < __runtime_types :: sp_core :: crypto :: AccountId32 > ,) > , } , submit { raw_solution : std :: boxed :: Box < __runtime_types :: pallet_election_provider_multi_phase :: RawSolution < __runtime_types :: node_runtime :: NposSolution16 > > , num_signed_submissions : u32 , } , }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    PreDispatchEarlySubmission,
                    PreDispatchWrongWinnerCount,
                    PreDispatchWeakSubmission,
                    SignedQueueFull,
                    SignedCannotPayDeposit,
                    SignedInvalidWitness,
                    SignedTooMuchWeight,
                    OcwCallWrongEra,
                    MissingSnapshotMetadata,
                    InvalidSubmissionIndex,
                    CallNotAllowed,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    SolutionStored(
                        __runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                        bool,
                    ),
                    ElectionFinalized(
                        Option<
                            __runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                        >,
                    ),
                    Rewarded(__runtime_types::sp_core::crypto::AccountId32, u128),
                    Slashed(__runtime_types::sp_core::crypto::AccountId32, u128),
                    SignedPhaseStarted(u32),
                    UnsignedPhaseStarted(u32),
                }
            }
            pub mod signed {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct SignedSubmission<_0, _1, _2> {
                    pub who: _0,
                    pub deposit: _1,
                    pub raw_solution:
                        __runtime_types::pallet_election_provider_multi_phase::RawSolution<_2>,
                    pub reward: _1,
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum ElectionCompute {
                OnChain,
                Signed,
                Unsigned,
                Emergency,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Phase<_0> {
                Off,
                Signed,
                Unsigned((bool, _0)),
                Emergency,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RawSolution<_0> {
                pub solution: _0,
                pub score: [u128; 3usize],
                pub round: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ReadySolution<_0> {
                pub supports: Vec<(_0, __runtime_types::sp_npos_elections::Support<_0>)>,
                pub score: [u128; 3usize],
                pub compute: __runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RoundSnapshot<_0> {
                pub voters: Vec<(_0, u64, Vec<_0>)>,
                pub targets: Vec<_0>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SolutionOrSnapshotSize {
                pub voters: u32,
                pub targets: u32,
            }
        }
        pub mod pallet_elections_phragmen {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    vote {
                        votes: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                        value: u128,
                    },
                    remove_voter,
                    submit_candidacy {
                        candidate_count: u32,
                    },
                    renounce_candidacy {
                        renouncing: __runtime_types::pallet_elections_phragmen::Renouncing,
                    },
                    remove_member {
                        who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        has_replacement: bool,
                    },
                    clean_defunct_voters {
                        num_voters: u32,
                        num_defunct: u32,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    UnableToVote,
                    NoVotes,
                    TooManyVotes,
                    MaximumVotesExceeded,
                    LowBalance,
                    UnableToPayBond,
                    MustBeVoter,
                    ReportSelf,
                    DuplicatedCandidate,
                    MemberSubmit,
                    RunnerUpSubmit,
                    InsufficientCandidateFunds,
                    NotMember,
                    InvalidWitnessData,
                    InvalidVoteCount,
                    InvalidRenouncing,
                    InvalidReplacement,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    NewTerm(Vec<(__runtime_types::sp_core::crypto::AccountId32, u128)>),
                    EmptyTerm,
                    ElectionError,
                    MemberKicked(__runtime_types::sp_core::crypto::AccountId32),
                    Renounced(__runtime_types::sp_core::crypto::AccountId32),
                    CandidateSlashed(__runtime_types::sp_core::crypto::AccountId32, u128),
                    SeatHolderSlashed(__runtime_types::sp_core::crypto::AccountId32, u128),
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Renouncing {
                Member,
                RunnerUp,
                Candidate(u32),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct SeatHolder<_0, _1> {
                pub who: _0,
                pub stake: _1,
                pub deposit: _1,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Voter<_0, _1> {
                pub votes: Vec<_0>,
                pub stake: _1,
                pub deposit: _1,
            }
        }
        pub mod pallet_gilt {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct ActiveGilt<_0, _1, _2> {
                    pub proportion: __runtime_types::sp_arithmetic::per_things::Perquintill,
                    pub amount: _0,
                    pub who: _1,
                    pub expiry: _2,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct ActiveGiltsTotal<_0> {
                    pub frozen: _0,
                    pub proportion: __runtime_types::sp_arithmetic::per_things::Perquintill,
                    pub index: u32,
                    pub target: __runtime_types::sp_arithmetic::per_things::Perquintill,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    place_bid {
                        amount: u128,
                        duration: u32,
                    },
                    retract_bid {
                        amount: u128,
                        duration: u32,
                    },
                    set_target {
                        target: __runtime_types::sp_arithmetic::per_things::Perquintill,
                    },
                    thaw {
                        index: u32,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    DurationTooSmall,
                    DurationTooBig,
                    AmountTooSmall,
                    BidTooLow,
                    Unknown,
                    NotOwner,
                    NotExpired,
                    NotFound,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    BidPlaced(__runtime_types::sp_core::crypto::AccountId32, u128, u32),
                    BidRetracted(__runtime_types::sp_core::crypto::AccountId32, u128, u32),
                    GiltIssued(
                        u32,
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    GiltThawed(
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                        u128,
                    ),
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct GiltBid<_0, _1> {
                    pub amount: _0,
                    pub who: _1,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    report_equivocation {
                        equivocation_proof: std::boxed::Box<
                            __runtime_types::sp_finality_grandpa::EquivocationProof<
                                __runtime_types::primitive_types::H256,
                                u32,
                            >,
                        >,
                        key_owner_proof: __runtime_types::sp_session::MembershipProof,
                    },
                    report_equivocation_unsigned {
                        equivocation_proof: std::boxed::Box<
                            __runtime_types::sp_finality_grandpa::EquivocationProof<
                                __runtime_types::primitive_types::H256,
                                u32,
                            >,
                        >,
                        key_owner_proof: __runtime_types::sp_session::MembershipProof,
                    },
                    note_stalled {
                        delay: u32,
                        best_finalized_block_number: u32,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    PauseFailed,
                    ResumeFailed,
                    ChangePending,
                    TooSoon,
                    InvalidKeyOwnershipProof,
                    InvalidEquivocationProof,
                    DuplicateOffenceReport,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    NewAuthorities(Vec<(__runtime_types::sp_finality_grandpa::app::Public, u64)>),
                    Paused,
                    Resumed,
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities: Vec<(__runtime_types::sp_finality_grandpa::app::Public, u64)>,
                pub forced: Option<_0>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum StoredState<_0> {
                Live,
                PendingPause { scheduled_at: _0, delay: _0 },
                Paused,
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_identity {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    add_registrar {
                        account: __runtime_types::sp_core::crypto::AccountId32,
                    },
                    set_identity {
                        info:
                            std::boxed::Box<__runtime_types::pallet_identity::types::IdentityInfo>,
                    },
                    set_subs {
                        subs: Vec<(
                            __runtime_types::sp_core::crypto::AccountId32,
                            __runtime_types::pallet_identity::types::Data,
                        )>,
                    },
                    clear_identity,
                    request_judgement {
                        reg_index: u32,
                        max_fee: u128,
                    },
                    cancel_request {
                        reg_index: u32,
                    },
                    set_fee {
                        index: u32,
                        fee: u128,
                    },
                    set_account_id {
                        index: u32,
                        new: __runtime_types::sp_core::crypto::AccountId32,
                    },
                    set_fields {
                        index: u32,
                        fields: __runtime_types::pallet_identity::types::BitFlags<
                            __runtime_types::pallet_identity::types::IdentityField,
                        >,
                    },
                    provide_judgement {
                        reg_index: u32,
                        target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        judgement: __runtime_types::pallet_identity::types::Judgement<u128>,
                    },
                    kill_identity {
                        target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    add_sub {
                        sub: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        data: __runtime_types::pallet_identity::types::Data,
                    },
                    rename_sub {
                        sub: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        data: __runtime_types::pallet_identity::types::Data,
                    },
                    remove_sub {
                        sub: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    quit_sub,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    TooManySubAccounts,
                    NotFound,
                    NotNamed,
                    EmptyIndex,
                    FeeChanged,
                    NoIdentity,
                    StickyJudgement,
                    JudgementGiven,
                    InvalidJudgement,
                    InvalidIndex,
                    InvalidTarget,
                    TooManyFields,
                    TooManyRegistrars,
                    AlreadyClaimed,
                    NotSub,
                    NotOwned,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    IdentitySet(__runtime_types::sp_core::crypto::AccountId32),
                    IdentityCleared(__runtime_types::sp_core::crypto::AccountId32, u128),
                    IdentityKilled(__runtime_types::sp_core::crypto::AccountId32, u128),
                    JudgementRequested(__runtime_types::sp_core::crypto::AccountId32, u32),
                    JudgementUnrequested(__runtime_types::sp_core::crypto::AccountId32, u32),
                    JudgementGiven(__runtime_types::sp_core::crypto::AccountId32, u32),
                    RegistrarAdded(u32),
                    SubIdentityAdded(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    SubIdentityRemoved(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    ),
                    SubIdentityRevoked(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    ),
                }
            }
            pub mod types {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct BitFlags<_0>(pub u64, pub core::marker::PhantomData<(_0)>);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Data {
                    None,
                    Raw0([u8; 0usize]),
                    Raw1([u8; 1usize]),
                    Raw2([u8; 2usize]),
                    Raw3([u8; 3usize]),
                    Raw4([u8; 4usize]),
                    Raw5([u8; 5usize]),
                    Raw6([u8; 6usize]),
                    Raw7([u8; 7usize]),
                    Raw8([u8; 8usize]),
                    Raw9([u8; 9usize]),
                    Raw10([u8; 10usize]),
                    Raw11([u8; 11usize]),
                    Raw12([u8; 12usize]),
                    Raw13([u8; 13usize]),
                    Raw14([u8; 14usize]),
                    Raw15([u8; 15usize]),
                    Raw16([u8; 16usize]),
                    Raw17([u8; 17usize]),
                    Raw18([u8; 18usize]),
                    Raw19([u8; 19usize]),
                    Raw20([u8; 20usize]),
                    Raw21([u8; 21usize]),
                    Raw22([u8; 22usize]),
                    Raw23([u8; 23usize]),
                    Raw24([u8; 24usize]),
                    Raw25([u8; 25usize]),
                    Raw26([u8; 26usize]),
                    Raw27([u8; 27usize]),
                    Raw28([u8; 28usize]),
                    Raw29([u8; 29usize]),
                    Raw30([u8; 30usize]),
                    Raw31([u8; 31usize]),
                    Raw32([u8; 32usize]),
                    BlakeTwo256([u8; 32usize]),
                    Sha256([u8; 32usize]),
                    Keccak256([u8; 32usize]),
                    ShaThree256([u8; 32usize]),
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum IdentityField {
                    Display,
                    Legal,
                    Web,
                    Riot,
                    Email,
                    PgpFingerprint,
                    Image,
                    Twitter,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct IdentityInfo {
                    pub additional:
                        __runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            __runtime_types::pallet_identity::types::Data,
                            __runtime_types::pallet_identity::types::Data,
                        )>,
                    pub display: __runtime_types::pallet_identity::types::Data,
                    pub legal: __runtime_types::pallet_identity::types::Data,
                    pub web: __runtime_types::pallet_identity::types::Data,
                    pub riot: __runtime_types::pallet_identity::types::Data,
                    pub email: __runtime_types::pallet_identity::types::Data,
                    pub pgp_fingerprint: Option<[u8; 20usize]>,
                    pub image: __runtime_types::pallet_identity::types::Data,
                    pub twitter: __runtime_types::pallet_identity::types::Data,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Judgement<_0> {
                    Unknown,
                    FeePaid(_0),
                    Reasonable,
                    KnownGood,
                    OutOfDate,
                    LowQuality,
                    Erroneous,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct RegistrarInfo<_0, _1> {
                    pub account: _1,
                    pub fee: _0,
                    pub fields: __runtime_types::pallet_identity::types::BitFlags<
                        __runtime_types::pallet_identity::types::IdentityField,
                    >,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Registration<_0> {
                    pub judgements:
                        __runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            u32,
                            __runtime_types::pallet_identity::types::Judgement<_0>,
                        )>,
                    pub deposit: _0,
                    pub info: __runtime_types::pallet_identity::types::IdentityInfo,
                }
            }
        }
        pub mod pallet_im_online {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    heartbeat {
                        heartbeat: __runtime_types::pallet_im_online::Heartbeat<u32>,
                        signature:
                            __runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    InvalidKey,
                    DuplicatedHeartbeat,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    HeartbeatReceived(
                        __runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    ),
                    AllGood,
                    SomeOffline(
                        Vec<(
                            __runtime_types::sp_core::crypto::AccountId32,
                            __runtime_types::pallet_staking::Exposure<
                                __runtime_types::sp_core::crypto::AccountId32,
                                u128,
                            >,
                        )>,
                    ),
                }
            }
            pub mod sr25519 {
                use super::__runtime_types;
                pub mod app_sr25519 {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct Public(pub __runtime_types::sp_core::sr25519::Public);
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct Signature(pub __runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state: __runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_indices {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    claim {
                        index: u32,
                    },
                    transfer {
                        new: __runtime_types::sp_core::crypto::AccountId32,
                        index: u32,
                    },
                    free {
                        index: u32,
                    },
                    force_transfer {
                        new: __runtime_types::sp_core::crypto::AccountId32,
                        index: u32,
                        freeze: bool,
                    },
                    freeze {
                        index: u32,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    NotAssigned,
                    NotOwner,
                    InUse,
                    NotTransfer,
                    Permanent,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    IndexAssigned(__runtime_types::sp_core::crypto::AccountId32, u32),
                    IndexFreed(u32),
                    IndexFrozen(u32, __runtime_types::sp_core::crypto::AccountId32),
                }
            }
        }
        pub mod pallet_lottery {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    buy_ticket {
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                    set_calls {
                        calls: Vec<__runtime_types::node_runtime::Call>,
                    },
                    start_lottery {
                        price: u128,
                        length: u32,
                        delay: u32,
                        repeat: bool,
                    },
                    stop_repeat,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    NotConfigured,
                    InProgress,
                    AlreadyEnded,
                    InvalidCall,
                    AlreadyParticipating,
                    TooManyCalls,
                    EncodingFailed,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    LotteryStarted,
                    CallsUpdated,
                    Winner(__runtime_types::sp_core::crypto::AccountId32, u128),
                    TicketBought(__runtime_types::sp_core::crypto::AccountId32, (u8, u8)),
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct LotteryConfig<_0, _1> {
                pub price: _1,
                pub start: _0,
                pub length: _0,
                pub delay: _0,
                pub repeat: bool,
            }
        }
        pub mod pallet_membership {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Call {
                add_member {
                    who: __runtime_types::sp_core::crypto::AccountId32,
                },
                remove_member {
                    who: __runtime_types::sp_core::crypto::AccountId32,
                },
                swap_member {
                    remove: __runtime_types::sp_core::crypto::AccountId32,
                    add: __runtime_types::sp_core::crypto::AccountId32,
                },
                reset_members {
                    members: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                },
                change_key {
                    new: __runtime_types::sp_core::crypto::AccountId32,
                },
                set_prime {
                    who: __runtime_types::sp_core::crypto::AccountId32,
                },
                clear_prime,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Error {
                AlreadyMember,
                NotMember,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Instance1 {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum RawEvent<_0, _1, _2> {
                MemberAdded,
                MemberRemoved,
                MembersSwapped,
                MembersReset,
                KeyChanged,
                Dummy,
                __Ignore(core::marker::PhantomData<(_0, _1, _2)>),
            }
        }
        pub mod pallet_multisig {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    as_multi_threshold_1 {
                        other_signatories: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                    as_multi {
                        threshold: u16,
                        other_signatories: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                        maybe_timepoint: Option<__runtime_types::pallet_multisig::Timepoint<u32>>,
                        call: Vec<u8>,
                        store_call: bool,
                        max_weight: u64,
                    },
                    approve_as_multi {
                        threshold: u16,
                        other_signatories: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                        maybe_timepoint: Option<__runtime_types::pallet_multisig::Timepoint<u32>>,
                        call_hash: [u8; 32usize],
                        max_weight: u64,
                    },
                    cancel_as_multi {
                        threshold: u16,
                        other_signatories: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                        timepoint: __runtime_types::pallet_multisig::Timepoint<u32>,
                        call_hash: [u8; 32usize],
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    MinimumThreshold,
                    AlreadyApproved,
                    NoApprovalsNeeded,
                    TooFewSignatories,
                    TooManySignatories,
                    SignatoriesOutOfOrder,
                    SenderInSignatories,
                    NotFound,
                    NotOwner,
                    NoTimepoint,
                    WrongTimepoint,
                    UnexpectedTimepoint,
                    MaxWeightTooLow,
                    AlreadyStored,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    NewMultisig(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        [u8; 32usize],
                    ),
                    MultisigApproval(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_multisig::Timepoint<u32>,
                        __runtime_types::sp_core::crypto::AccountId32,
                        [u8; 32usize],
                    ),
                    MultisigExecuted(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_multisig::Timepoint<u32>,
                        __runtime_types::sp_core::crypto::AccountId32,
                        [u8; 32usize],
                        Result<(), __runtime_types::sp_runtime::DispatchError>,
                    ),
                    MultisigCancelled(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_multisig::Timepoint<u32>,
                        __runtime_types::sp_core::crypto::AccountId32,
                        [u8; 32usize],
                    ),
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Multisig<_0, _1, _2> {
                pub when: __runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: Vec<_2>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_offences {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Offence([u8; 16usize], Vec<u8>),
                }
            }
        }
        pub mod pallet_proxy {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    proxy {
                        real: __runtime_types::sp_core::crypto::AccountId32,
                        force_proxy_type: Option<__runtime_types::node_runtime::ProxyType>,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                    add_proxy {
                        delegate: __runtime_types::sp_core::crypto::AccountId32,
                        proxy_type: __runtime_types::node_runtime::ProxyType,
                        delay: u32,
                    },
                    remove_proxy {
                        delegate: __runtime_types::sp_core::crypto::AccountId32,
                        proxy_type: __runtime_types::node_runtime::ProxyType,
                        delay: u32,
                    },
                    remove_proxies,
                    anonymous {
                        proxy_type: __runtime_types::node_runtime::ProxyType,
                        delay: u32,
                        index: u16,
                    },
                    kill_anonymous {
                        spawner: __runtime_types::sp_core::crypto::AccountId32,
                        proxy_type: __runtime_types::node_runtime::ProxyType,
                        index: u16,
                        height: u32,
                        ext_index: u32,
                    },
                    announce {
                        real: __runtime_types::sp_core::crypto::AccountId32,
                        call_hash: __runtime_types::primitive_types::H256,
                    },
                    remove_announcement {
                        real: __runtime_types::sp_core::crypto::AccountId32,
                        call_hash: __runtime_types::primitive_types::H256,
                    },
                    reject_announcement {
                        delegate: __runtime_types::sp_core::crypto::AccountId32,
                        call_hash: __runtime_types::primitive_types::H256,
                    },
                    proxy_announced {
                        delegate: __runtime_types::sp_core::crypto::AccountId32,
                        real: __runtime_types::sp_core::crypto::AccountId32,
                        force_proxy_type: Option<__runtime_types::node_runtime::ProxyType>,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    TooMany,
                    NotFound,
                    NotProxy,
                    Unproxyable,
                    Duplicate,
                    NoPermission,
                    Unannounced,
                    NoSelfProxy,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    ProxyExecuted(Result<(), __runtime_types::sp_runtime::DispatchError>),
                    AnonymousCreated(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::node_runtime::ProxyType,
                        u16,
                    ),
                    Announced(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                    ),
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Announcement<_0, _1, _2> {
                pub real: _0,
                pub call_hash: _1,
                pub height: _2,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ProxyDefinition<_0, _1, _2> {
                pub delegate: _0,
                pub proxy_type: _1,
                pub delay: _2,
            }
        }
        pub mod pallet_recovery {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    as_recovered {
                        account: __runtime_types::sp_core::crypto::AccountId32,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                    set_recovered {
                        lost: __runtime_types::sp_core::crypto::AccountId32,
                        rescuer: __runtime_types::sp_core::crypto::AccountId32,
                    },
                    create_recovery {
                        friends: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                        threshold: u16,
                        delay_period: u32,
                    },
                    initiate_recovery {
                        account: __runtime_types::sp_core::crypto::AccountId32,
                    },
                    vouch_recovery {
                        lost: __runtime_types::sp_core::crypto::AccountId32,
                        rescuer: __runtime_types::sp_core::crypto::AccountId32,
                    },
                    claim_recovery {
                        account: __runtime_types::sp_core::crypto::AccountId32,
                    },
                    close_recovery {
                        rescuer: __runtime_types::sp_core::crypto::AccountId32,
                    },
                    remove_recovery,
                    cancel_recovered {
                        account: __runtime_types::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    NotAllowed,
                    ZeroThreshold,
                    NotEnoughFriends,
                    MaxFriends,
                    NotSorted,
                    NotRecoverable,
                    AlreadyRecoverable,
                    AlreadyStarted,
                    NotStarted,
                    NotFriend,
                    DelayPeriod,
                    AlreadyVouched,
                    Threshold,
                    StillActive,
                    AlreadyProxy,
                    BadState,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    RecoveryCreated(__runtime_types::sp_core::crypto::AccountId32),
                    RecoveryInitiated(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    RecoveryVouched(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    RecoveryClosed(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    AccountRecovered(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    RecoveryRemoved(__runtime_types::sp_core::crypto::AccountId32),
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ActiveRecovery<_0, _1, _2> {
                pub created: _0,
                pub deposit: _1,
                pub friends: Vec<_2>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RecoveryConfig<_0, _1, _2> {
                pub delay_period: _0,
                pub deposit: _1,
                pub friends: Vec<_2>,
                pub threshold: u16,
            }
        }
        pub mod pallet_scheduler {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    schedule {
                        when: u32,
                        maybe_periodic: Option<(u32, u32)>,
                        priority: u8,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                    cancel {
                        when: u32,
                        index: u32,
                    },
                    schedule_named {
                        id: Vec<u8>,
                        when: u32,
                        maybe_periodic: Option<(u32, u32)>,
                        priority: u8,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                    cancel_named {
                        id: Vec<u8>,
                    },
                    schedule_after {
                        after: u32,
                        maybe_periodic: Option<(u32, u32)>,
                        priority: u8,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                    schedule_named_after {
                        id: Vec<u8>,
                        after: u32,
                        maybe_periodic: Option<(u32, u32)>,
                        priority: u8,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    FailedToSchedule,
                    NotFound,
                    TargetBlockNumberInPast,
                    RescheduleNoChange,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Scheduled(u32, u32),
                    Canceled(u32, u32),
                    Dispatched(
                        (u32, u32),
                        Option<Vec<u8>>,
                        Result<(), __runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V1,
                V2,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ScheduledV2<_0, _1, _2, _3> {
                pub maybe_id: Option<Vec<u8>>,
                pub priority: u8,
                pub call: _0,
                pub maybe_periodic: Option<(_1, _1)>,
                pub origin: _2,
                pub __chameleon_unused_type_params: core::marker::PhantomData<(_3,)>,
            }
        }
        pub mod pallet_session {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Call {
                set_keys {
                    keys: __runtime_types::node_runtime::SessionKeys,
                    proof: Vec<u8>,
                },
                purge_keys,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Error {
                InvalidProof,
                NoAssociatedValidatorId,
                DuplicatedKey,
                NoKeys,
                NoAccount,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Event {
                NewSession(u32),
            }
        }
        pub mod pallet_society {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Bid<_0, _1> {
                pub who: _0,
                pub kind: __runtime_types::pallet_society::BidKind<_0, _1>,
                pub value: _1,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum BidKind<_0, _1> {
                Deposit(_1),
                Vouch(_0, _1),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Call {
                bid {
                    value: u128,
                },
                unbid {
                    pos: u32,
                },
                vouch {
                    who: __runtime_types::sp_core::crypto::AccountId32,
                    value: u128,
                    tip: u128,
                },
                unvouch {
                    pos: u32,
                },
                vote {
                    candidate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    approve: bool,
                },
                defender_vote {
                    approve: bool,
                },
                payout,
                found {
                    founder: __runtime_types::sp_core::crypto::AccountId32,
                    max_members: u32,
                    rules: Vec<u8>,
                },
                unfound,
                judge_suspended_member {
                    who: __runtime_types::sp_core::crypto::AccountId32,
                    forgive: bool,
                },
                judge_suspended_candidate {
                    who: __runtime_types::sp_core::crypto::AccountId32,
                    judgement: __runtime_types::pallet_society::Judgement,
                },
                set_max_members {
                    max: u32,
                },
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct DefaultInstance {}
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Error {
                BadPosition,
                NotMember,
                AlreadyMember,
                Suspended,
                NotSuspended,
                NoPayout,
                AlreadyFounded,
                InsufficientPot,
                AlreadyVouching,
                NotVouching,
                Head,
                Founder,
                AlreadyBid,
                AlreadyCandidate,
                NotCandidate,
                MaxMembers,
                NotFounder,
                NotHead,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Judgement {
                Rebid,
                Reject,
                Approve,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum RawEvent<_0, _1, _2> {
                Founded(_0),
                Bid(_0, _1),
                Vouch(_0, _1, _0),
                AutoUnbid(_0),
                Unbid(_0),
                Unvouch(_0),
                Inducted(_0, Vec<_0>),
                SuspendedMemberJudgement(_0, bool),
                CandidateSuspended(_0),
                MemberSuspended(_0),
                Challenged(_0),
                Vote(_0, _0, bool),
                DefenderVote(_0, bool),
                NewMaxMembers(u32),
                Unfounded(_0),
                Deposit(_1),
                __Ignore(core::marker::PhantomData<(_2,)>),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Vote {
                Skeptic,
                Reject,
                Approve,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum VouchingStatus {
                Vouching,
                Banned,
            }
        }
        pub mod pallet_staking {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub mod pallet {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub enum Call {
                        bond {
                            controller: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                                __runtime_types::sp_core::crypto::AccountId32,
                                u32,
                            >,
                            value: u128,
                            payee: __runtime_types::pallet_staking::RewardDestination<
                                __runtime_types::sp_core::crypto::AccountId32,
                            >,
                        },
                        bond_extra {
                            max_additional: u128,
                        },
                        unbond {
                            value: u128,
                        },
                        withdraw_unbonded {
                            num_slashing_spans: u32,
                        },
                        validate {
                            prefs: __runtime_types::pallet_staking::ValidatorPrefs,
                        },
                        nominate {
                            targets: Vec<
                                __runtime_types::sp_runtime::multiaddress::MultiAddress<
                                    __runtime_types::sp_core::crypto::AccountId32,
                                    u32,
                                >,
                            >,
                        },
                        chill,
                        set_payee {
                            payee: __runtime_types::pallet_staking::RewardDestination<
                                __runtime_types::sp_core::crypto::AccountId32,
                            >,
                        },
                        set_controller {
                            controller: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                                __runtime_types::sp_core::crypto::AccountId32,
                                u32,
                            >,
                        },
                        set_validator_count {
                            new: u32,
                        },
                        increase_validator_count {
                            additional: u32,
                        },
                        scale_validator_count {
                            factor: __runtime_types::sp_arithmetic::per_things::Percent,
                        },
                        force_no_eras,
                        force_new_era,
                        set_invulnerables {
                            invulnerables: Vec<__runtime_types::sp_core::crypto::AccountId32>,
                        },
                        force_unstake {
                            stash: __runtime_types::sp_core::crypto::AccountId32,
                            num_slashing_spans: u32,
                        },
                        force_new_era_always,
                        cancel_deferred_slash {
                            era: u32,
                            slash_indices: Vec<u32>,
                        },
                        payout_stakers {
                            validator_stash: __runtime_types::sp_core::crypto::AccountId32,
                            era: u32,
                        },
                        rebond {
                            value: u128,
                        },
                        set_history_depth {
                            new_history_depth: u32,
                            era_items_deleted: u32,
                        },
                        reap_stash {
                            stash: __runtime_types::sp_core::crypto::AccountId32,
                            num_slashing_spans: u32,
                        },
                        kick {
                            who: Vec<
                                __runtime_types::sp_runtime::multiaddress::MultiAddress<
                                    __runtime_types::sp_core::crypto::AccountId32,
                                    u32,
                                >,
                            >,
                        },
                        set_staking_limits {
                            min_nominator_bond: u128,
                            min_validator_bond: u128,
                            max_nominator_count: Option<u32>,
                            max_validator_count: Option<u32>,
                            threshold: Option<__runtime_types::sp_arithmetic::per_things::Percent>,
                        },
                        chill_other {
                            controller: __runtime_types::sp_core::crypto::AccountId32,
                        },
                    }
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub enum Error {
                        NotController,
                        NotStash,
                        AlreadyBonded,
                        AlreadyPaired,
                        EmptyTargets,
                        DuplicateIndex,
                        InvalidSlashIndex,
                        InsufficientBond,
                        NoMoreChunks,
                        NoUnlockChunk,
                        FundedTarget,
                        InvalidEraToReward,
                        InvalidNumberOfNominations,
                        NotSortedAndUnique,
                        AlreadyClaimed,
                        IncorrectHistoryDepth,
                        IncorrectSlashingSpans,
                        BadState,
                        TooManyTargets,
                        BadTarget,
                        CannotChillOther,
                        TooManyNominators,
                        TooManyValidators,
                    }
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub enum Event {
                        EraPaid(u32, u128, u128),
                        Rewarded(__runtime_types::sp_core::crypto::AccountId32, u128),
                        Slashed(__runtime_types::sp_core::crypto::AccountId32, u128),
                        OldSlashingReportDiscarded(u32),
                        StakersElected,
                        Bonded(__runtime_types::sp_core::crypto::AccountId32, u128),
                        Unbonded(__runtime_types::sp_core::crypto::AccountId32, u128),
                        Withdrawn(__runtime_types::sp_core::crypto::AccountId32, u128),
                        Kicked(
                            __runtime_types::sp_core::crypto::AccountId32,
                            __runtime_types::sp_core::crypto::AccountId32,
                        ),
                        StakingElectionFailed,
                        Chilled(__runtime_types::sp_core::crypto::AccountId32),
                        PayoutStarted(u32, __runtime_types::sp_core::crypto::AccountId32),
                    }
                }
            }
            pub mod slashing {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct SlashingSpans {
                    pub span_index: u32,
                    pub last_start: u32,
                    pub last_nonzero_slash: u32,
                    pub prior: Vec<u32>,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct SpanRecord<_0> {
                    pub slashed: _0,
                    pub paid_out: _0,
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ActiveEraInfo {
                pub index: u32,
                pub start: Option<u64>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct EraRewardPoints<_0> {
                pub total: u32,
                pub individual: std::collections::BTreeMap<_0, u32>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Exposure<_0, _1> {
                pub total: _1,
                pub own: _1,
                pub others: Vec<__runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Forcing {
                NotForcing,
                ForceNew,
                ForceNone,
                ForceAlways,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct IndividualExposure<_0, _1> {
                pub who: _0,
                pub value: _1,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Nominations<_0> {
                pub targets: Vec<_0>,
                pub submitted_in: u32,
                pub suppressed: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V1_0_0Ancient,
                V2_0_0,
                V3_0_0,
                V4_0_0,
                V5_0_0,
                V6_0_0,
                V7_0_0,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum RewardDestination<_0> {
                Staked,
                Stash,
                Controller,
                Account(_0),
                None,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct StakingLedger<_0, _1> {
                pub stash: _0,
                pub total: _1,
                pub active: _1,
                pub unlocking: Vec<__runtime_types::pallet_staking::UnlockChunk<_1>>,
                pub claimed_rewards: Vec<u32>,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct UnappliedSlash<_0, _1> {
                pub validator: _0,
                pub own: _1,
                pub others: Vec<(_0, _1)>,
                pub reporters: Vec<_0>,
                pub payout: _1,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct UnlockChunk<_0> {
                pub value: _0,
                pub era: u32,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ValidatorPrefs {
                pub commission: __runtime_types::sp_arithmetic::per_things::Perbill,
                pub blocked: bool,
            }
        }
        pub mod pallet_sudo {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    sudo {
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                    sudo_unchecked_weight {
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                        weight: u64,
                    },
                    set_key {
                        new: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    sudo_as {
                        who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    RequireSudo,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Sudid(Result<(), __runtime_types::sp_runtime::DispatchError>),
                    KeyChanged(__runtime_types::sp_core::crypto::AccountId32),
                    SudoAsDone(Result<(), __runtime_types::sp_runtime::DispatchError>),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    set { now: u64 },
                }
            }
        }
        pub mod pallet_tips {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Call {
                report_awesome {
                    reason: Vec<u8>,
                    who: __runtime_types::sp_core::crypto::AccountId32,
                },
                retract_tip {
                    hash: __runtime_types::primitive_types::H256,
                },
                tip_new {
                    reason: Vec<u8>,
                    who: __runtime_types::sp_core::crypto::AccountId32,
                    tip_value: u128,
                },
                tip {
                    hash: __runtime_types::primitive_types::H256,
                    tip_value: u128,
                },
                close_tip {
                    hash: __runtime_types::primitive_types::H256,
                },
                slash_tip {
                    hash: __runtime_types::primitive_types::H256,
                },
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Error {
                ReasonTooBig,
                AlreadyKnown,
                UnknownTip,
                NotFinder,
                StillOpen,
                Premature,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct OpenTip<_0, _1, _2, _3> {
                pub reason: _3,
                pub who: _0,
                pub finder: _0,
                pub deposit: _1,
                pub closes: Option<_2>,
                pub tips: Vec<(_0, _1)>,
                pub finders_fee: bool,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum RawEvent<_0, _1, _2> {
                NewTip(_2),
                TipClosing(_2),
                TipClosed(_2, _1, _0),
                TipRetracted(_2),
                TipSlashed(_2, _1, _0),
            }
        }
        pub mod pallet_transaction_payment {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct ChargeTransactionPayment(pub u128);
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V1Ancient,
                V2,
            }
        }
        pub mod pallet_transaction_storage {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    store {
                        data: Vec<u8>,
                    },
                    renew {
                        block: u32,
                        index: u32,
                    },
                    check_proof {
                        proof:
                            __runtime_types::sp_transaction_storage_proof::TransactionStorageProof,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    InsufficientFunds,
                    NotConfigured,
                    RenewedNotFound,
                    EmptyTransaction,
                    UnexpectedProof,
                    InvalidProof,
                    MissingProof,
                    MissingStateData,
                    DoubleCheck,
                    ProofNotChecked,
                    TransactionTooLarge,
                    TooManyTransactions,
                    BadContext,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Stored(u32),
                    Renewed(u32),
                    ProofChecked,
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransactionInfo {
                pub chunk_root: __runtime_types::primitive_types::H256,
                pub content_hash: __runtime_types::primitive_types::H256,
                pub size: u32,
                pub block_chunks: u32,
            }
        }
        pub mod pallet_treasury {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    propose_spend {
                        value: u128,
                        beneficiary: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    reject_proposal {
                        proposal_id: u32,
                    },
                    approve_proposal {
                        proposal_id: u32,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    InsufficientProposersBalance,
                    InvalidIndex,
                    TooManyApprovals,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Proposed(u32),
                    Spending(u128),
                    Awarded(u32, u128, __runtime_types::sp_core::crypto::AccountId32),
                    Rejected(u32, u128),
                    Burnt(u128),
                    Rollover(u128),
                    Deposit(u128),
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Proposal<_0, _1> {
                pub proposer: _0,
                pub value: _1,
                pub beneficiary: _0,
                pub bond: _1,
            }
        }
        pub mod pallet_uniques {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    create {
                        class: u32,
                        admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    force_create {
                        class: u32,
                        owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        free_holding: bool,
                    },
                    destroy {
                        class: u32,
                        witness: __runtime_types::pallet_uniques::types::DestroyWitness,
                    },
                    mint {
                        class: u32,
                        instance: u32,
                        owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    burn {
                        class: u32,
                        instance: u32,
                        check_owner: Option<
                            __runtime_types::sp_runtime::multiaddress::MultiAddress<
                                __runtime_types::sp_core::crypto::AccountId32,
                                u32,
                            >,
                        >,
                    },
                    transfer {
                        class: u32,
                        instance: u32,
                        dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    redeposit {
                        class: u32,
                        instances: Vec<u32>,
                    },
                    freeze {
                        class: u32,
                        instance: u32,
                    },
                    thaw {
                        class: u32,
                        instance: u32,
                    },
                    freeze_class {
                        class: u32,
                    },
                    thaw_class {
                        class: u32,
                    },
                    transfer_ownership {
                        class: u32,
                        owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    set_team {
                        class: u32,
                        issuer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        freezer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    approve_transfer {
                        class: u32,
                        instance: u32,
                        delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    cancel_approval {
                        class: u32,
                        instance: u32,
                        maybe_check_delegate: Option<
                            __runtime_types::sp_runtime::multiaddress::MultiAddress<
                                __runtime_types::sp_core::crypto::AccountId32,
                                u32,
                            >,
                        >,
                    },
                    force_asset_status {
                        class: u32,
                        owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        issuer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        freezer: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        free_holding: bool,
                        is_frozen: bool,
                    },
                    set_attribute {
                        class: u32,
                        maybe_instance: Option<u32>,
                        key: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                        value: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                    },
                    clear_attribute {
                        class: u32,
                        maybe_instance: Option<u32>,
                        key: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                    },
                    set_metadata {
                        class: u32,
                        instance: u32,
                        data: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                        is_frozen: bool,
                    },
                    clear_metadata {
                        class: u32,
                        instance: u32,
                    },
                    set_class_metadata {
                        class: u32,
                        data: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                        is_frozen: bool,
                    },
                    clear_class_metadata {
                        class: u32,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    NoPermission,
                    Unknown,
                    AlreadyExists,
                    WrongOwner,
                    BadWitness,
                    InUse,
                    Frozen,
                    WrongDelegate,
                    NoDelegate,
                    Unapproved,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    Created(
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    ForceCreated(u32, __runtime_types::sp_core::crypto::AccountId32),
                    Destroyed(u32),
                    Issued(u32, u32, __runtime_types::sp_core::crypto::AccountId32),
                    Transferred(
                        u32,
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    Burned(u32, u32, __runtime_types::sp_core::crypto::AccountId32),
                    Frozen(u32, u32),
                    Thawed(u32, u32),
                    ClassFrozen(u32),
                    ClassThawed(u32),
                    OwnerChanged(u32, __runtime_types::sp_core::crypto::AccountId32),
                    TeamChanged(
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    ApprovedTransfer(
                        u32,
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    ApprovalCancelled(
                        u32,
                        u32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                    ),
                    AssetStatusChanged(u32),
                    ClassMetadataSet(
                        u32,
                        __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                        bool,
                    ),
                    ClassMetadataCleared(u32),
                    MetadataSet(
                        u32,
                        u32,
                        __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                        bool,
                    ),
                    MetadataCleared(u32, u32),
                    Redeposited(u32, Vec<u32>),
                    AttributeSet(
                        u32,
                        Option<u32>,
                        __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                        __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                    ),
                    AttributeCleared(
                        u32,
                        Option<u32>,
                        __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                    ),
                }
            }
            pub mod types {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct ClassDetails<_0, _1> {
                    pub owner: _0,
                    pub issuer: _0,
                    pub admin: _0,
                    pub freezer: _0,
                    pub total_deposit: _1,
                    pub free_holding: bool,
                    pub instances: u32,
                    pub instance_metadatas: u32,
                    pub attributes: u32,
                    pub is_frozen: bool,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct ClassMetadata<_0> {
                    pub deposit: _0,
                    pub data: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                    pub is_frozen: bool,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct DestroyWitness {
                    pub instances: u32,
                    pub instance_metadatas: u32,
                    pub attributes: u32,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct InstanceDetails<_0, _1> {
                    pub owner: _0,
                    pub approved: Option<_0>,
                    pub is_frozen: bool,
                    pub deposit: _1,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct InstanceMetadata<_0> {
                    pub deposit: _0,
                    pub data: __runtime_types::frame_support::storage::bounded_vec::BoundedVec<u8>,
                    pub is_frozen: bool,
                }
            }
        }
        pub mod pallet_utility {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    batch {
                        calls: Vec<__runtime_types::node_runtime::Call>,
                    },
                    as_derivative {
                        index: u16,
                        call: std::boxed::Box<__runtime_types::node_runtime::Call>,
                    },
                    batch_all {
                        calls: Vec<__runtime_types::node_runtime::Call>,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    TooManyCalls,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    BatchInterrupted(u32, __runtime_types::sp_runtime::DispatchError),
                    BatchCompleted,
                    ItemCompleted,
                }
            }
        }
        pub mod pallet_vesting {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Call {
                    vest,
                    vest_other {
                        target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    },
                    vested_transfer {
                        target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        schedule:
                            __runtime_types::pallet_vesting::vesting_info::VestingInfo<u128, u32>,
                    },
                    force_vested_transfer {
                        source: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        schedule:
                            __runtime_types::pallet_vesting::vesting_info::VestingInfo<u128, u32>,
                    },
                    merge_schedules {
                        schedule1_index: u32,
                        schedule2_index: u32,
                    },
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Error {
                    NotVesting,
                    AtMaxVestingSchedules,
                    AmountLow,
                    ScheduleIndexOutOfBounds,
                    InvalidScheduleParams,
                }
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum Event {
                    VestingUpdated(__runtime_types::sp_core::crypto::AccountId32, u128),
                    VestingCompleted(__runtime_types::sp_core::crypto::AccountId32),
                }
            }
            pub mod vesting_info {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct VestingInfo<_0, _1> {
                    pub locked: _0,
                    pub per_block: _0,
                    pub starting_block: _1,
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Releases {
                V0,
                V1,
            }
        }
        pub mod primitive_types {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct H256(pub [u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::__runtime_types;
            pub mod fixed_point {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct FixedU128(pub u128);
            }
            pub mod per_things {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct PerU16(pub u16);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Perbill(pub u32);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Percent(pub u8);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Permill(pub u32);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Perquintill(pub u64);
            }
        }
        pub mod sp_authority_discovery {
            use super::__runtime_types;
            pub mod app {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Public(pub __runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod sp_consensus_babe {
            use super::__runtime_types;
            pub mod app {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Public(pub __runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum NextConfigDescriptor {
                    V1 {
                        c: (u64, u64),
                        allowed_slots: __runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum AllowedSlots {
                PrimarySlots,
                PrimaryAndSecondaryPlainSlots,
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct BabeEpochConfiguration {
                pub c: (u64, u64),
                pub allowed_slots: __runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: __runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Slot(pub u64);
        }
        pub mod sp_core {
            use super::__runtime_types;
            pub mod changes_trie {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct ChangesTrieConfiguration {
                    pub digest_interval: u32,
                    pub digest_levels: u32,
                }
            }
            pub mod crypto {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct AccountId32(pub [u8; 32usize]);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct KeyTypeId(pub [u8; 4usize]);
            }
            pub mod ecdsa {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Signature(pub [u8; 65usize]);
            }
            pub mod ed25519 {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Public(pub [u8; 32usize]);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Signature(pub [u8; 64usize]);
            }
            pub mod offchain {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct OpaqueMultiaddr(pub Vec<u8>);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct OpaqueNetworkState {
                    pub peer_id: __runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        Vec<__runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Public(pub [u8; 32usize]);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Signature(pub [u8; 64usize]);
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct OpaquePeerId(pub Vec<u8>);
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::__runtime_types;
            pub mod app {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Public(pub __runtime_types::sp_core::ed25519::Public);
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct Signature(pub __runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum Equivocation<_0, _1> {
                Prevote(
                    __runtime_types::finality_grandpa::Equivocation<
                        __runtime_types::sp_finality_grandpa::app::Public,
                        __runtime_types::finality_grandpa::Prevote<_0, _1>,
                        __runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                Precommit(
                    __runtime_types::finality_grandpa::Equivocation<
                        __runtime_types::sp_finality_grandpa::app::Public,
                        __runtime_types::finality_grandpa::Precommit<_0, _1>,
                        __runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: u64,
                pub equivocation: __runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_npos_elections {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct Support<_0> {
                pub total: u128,
                pub voters: Vec<(_0, u128)>,
            }
        }
        pub mod sp_runtime {
            use super::__runtime_types;
            pub mod generic {
                use super::__runtime_types;
                pub mod digest {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub enum ChangesTrieSignal {
                        NewConfiguration(
                            Option<
                                __runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                            >,
                        ),
                    }
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct Digest<_0> {
                        pub logs: Vec<__runtime_types::sp_runtime::generic::digest::DigestItem<_0>>,
                    }
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub enum DigestItem<_0> {
                        ChangesTrieRoot(_0),
                        PreRuntime([u8; 4usize], Vec<u8>),
                        Consensus([u8; 4usize], Vec<u8>),
                        Seal([u8; 4usize], Vec<u8>),
                        ChangesTrieSignal(
                            __runtime_types::sp_runtime::generic::digest::ChangesTrieSignal,
                        ),
                        Other(Vec<u8>),
                    }
                }
                pub mod era {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub enum Era {
                        Immortal,
                        Mortal1(u8),
                        Mortal2(u8),
                        Mortal3(u8),
                        Mortal4(u8),
                        Mortal5(u8),
                        Mortal6(u8),
                        Mortal7(u8),
                        Mortal8(u8),
                        Mortal9(u8),
                        Mortal10(u8),
                        Mortal11(u8),
                        Mortal12(u8),
                        Mortal13(u8),
                        Mortal14(u8),
                        Mortal15(u8),
                        Mortal16(u8),
                        Mortal17(u8),
                        Mortal18(u8),
                        Mortal19(u8),
                        Mortal20(u8),
                        Mortal21(u8),
                        Mortal22(u8),
                        Mortal23(u8),
                        Mortal24(u8),
                        Mortal25(u8),
                        Mortal26(u8),
                        Mortal27(u8),
                        Mortal28(u8),
                        Mortal29(u8),
                        Mortal30(u8),
                        Mortal31(u8),
                        Mortal32(u8),
                        Mortal33(u8),
                        Mortal34(u8),
                        Mortal35(u8),
                        Mortal36(u8),
                        Mortal37(u8),
                        Mortal38(u8),
                        Mortal39(u8),
                        Mortal40(u8),
                        Mortal41(u8),
                        Mortal42(u8),
                        Mortal43(u8),
                        Mortal44(u8),
                        Mortal45(u8),
                        Mortal46(u8),
                        Mortal47(u8),
                        Mortal48(u8),
                        Mortal49(u8),
                        Mortal50(u8),
                        Mortal51(u8),
                        Mortal52(u8),
                        Mortal53(u8),
                        Mortal54(u8),
                        Mortal55(u8),
                        Mortal56(u8),
                        Mortal57(u8),
                        Mortal58(u8),
                        Mortal59(u8),
                        Mortal60(u8),
                        Mortal61(u8),
                        Mortal62(u8),
                        Mortal63(u8),
                        Mortal64(u8),
                        Mortal65(u8),
                        Mortal66(u8),
                        Mortal67(u8),
                        Mortal68(u8),
                        Mortal69(u8),
                        Mortal70(u8),
                        Mortal71(u8),
                        Mortal72(u8),
                        Mortal73(u8),
                        Mortal74(u8),
                        Mortal75(u8),
                        Mortal76(u8),
                        Mortal77(u8),
                        Mortal78(u8),
                        Mortal79(u8),
                        Mortal80(u8),
                        Mortal81(u8),
                        Mortal82(u8),
                        Mortal83(u8),
                        Mortal84(u8),
                        Mortal85(u8),
                        Mortal86(u8),
                        Mortal87(u8),
                        Mortal88(u8),
                        Mortal89(u8),
                        Mortal90(u8),
                        Mortal91(u8),
                        Mortal92(u8),
                        Mortal93(u8),
                        Mortal94(u8),
                        Mortal95(u8),
                        Mortal96(u8),
                        Mortal97(u8),
                        Mortal98(u8),
                        Mortal99(u8),
                        Mortal100(u8),
                        Mortal101(u8),
                        Mortal102(u8),
                        Mortal103(u8),
                        Mortal104(u8),
                        Mortal105(u8),
                        Mortal106(u8),
                        Mortal107(u8),
                        Mortal108(u8),
                        Mortal109(u8),
                        Mortal110(u8),
                        Mortal111(u8),
                        Mortal112(u8),
                        Mortal113(u8),
                        Mortal114(u8),
                        Mortal115(u8),
                        Mortal116(u8),
                        Mortal117(u8),
                        Mortal118(u8),
                        Mortal119(u8),
                        Mortal120(u8),
                        Mortal121(u8),
                        Mortal122(u8),
                        Mortal123(u8),
                        Mortal124(u8),
                        Mortal125(u8),
                        Mortal126(u8),
                        Mortal127(u8),
                        Mortal128(u8),
                        Mortal129(u8),
                        Mortal130(u8),
                        Mortal131(u8),
                        Mortal132(u8),
                        Mortal133(u8),
                        Mortal134(u8),
                        Mortal135(u8),
                        Mortal136(u8),
                        Mortal137(u8),
                        Mortal138(u8),
                        Mortal139(u8),
                        Mortal140(u8),
                        Mortal141(u8),
                        Mortal142(u8),
                        Mortal143(u8),
                        Mortal144(u8),
                        Mortal145(u8),
                        Mortal146(u8),
                        Mortal147(u8),
                        Mortal148(u8),
                        Mortal149(u8),
                        Mortal150(u8),
                        Mortal151(u8),
                        Mortal152(u8),
                        Mortal153(u8),
                        Mortal154(u8),
                        Mortal155(u8),
                        Mortal156(u8),
                        Mortal157(u8),
                        Mortal158(u8),
                        Mortal159(u8),
                        Mortal160(u8),
                        Mortal161(u8),
                        Mortal162(u8),
                        Mortal163(u8),
                        Mortal164(u8),
                        Mortal165(u8),
                        Mortal166(u8),
                        Mortal167(u8),
                        Mortal168(u8),
                        Mortal169(u8),
                        Mortal170(u8),
                        Mortal171(u8),
                        Mortal172(u8),
                        Mortal173(u8),
                        Mortal174(u8),
                        Mortal175(u8),
                        Mortal176(u8),
                        Mortal177(u8),
                        Mortal178(u8),
                        Mortal179(u8),
                        Mortal180(u8),
                        Mortal181(u8),
                        Mortal182(u8),
                        Mortal183(u8),
                        Mortal184(u8),
                        Mortal185(u8),
                        Mortal186(u8),
                        Mortal187(u8),
                        Mortal188(u8),
                        Mortal189(u8),
                        Mortal190(u8),
                        Mortal191(u8),
                        Mortal192(u8),
                        Mortal193(u8),
                        Mortal194(u8),
                        Mortal195(u8),
                        Mortal196(u8),
                        Mortal197(u8),
                        Mortal198(u8),
                        Mortal199(u8),
                        Mortal200(u8),
                        Mortal201(u8),
                        Mortal202(u8),
                        Mortal203(u8),
                        Mortal204(u8),
                        Mortal205(u8),
                        Mortal206(u8),
                        Mortal207(u8),
                        Mortal208(u8),
                        Mortal209(u8),
                        Mortal210(u8),
                        Mortal211(u8),
                        Mortal212(u8),
                        Mortal213(u8),
                        Mortal214(u8),
                        Mortal215(u8),
                        Mortal216(u8),
                        Mortal217(u8),
                        Mortal218(u8),
                        Mortal219(u8),
                        Mortal220(u8),
                        Mortal221(u8),
                        Mortal222(u8),
                        Mortal223(u8),
                        Mortal224(u8),
                        Mortal225(u8),
                        Mortal226(u8),
                        Mortal227(u8),
                        Mortal228(u8),
                        Mortal229(u8),
                        Mortal230(u8),
                        Mortal231(u8),
                        Mortal232(u8),
                        Mortal233(u8),
                        Mortal234(u8),
                        Mortal235(u8),
                        Mortal236(u8),
                        Mortal237(u8),
                        Mortal238(u8),
                        Mortal239(u8),
                        Mortal240(u8),
                        Mortal241(u8),
                        Mortal242(u8),
                        Mortal243(u8),
                        Mortal244(u8),
                        Mortal245(u8),
                        Mortal246(u8),
                        Mortal247(u8),
                        Mortal248(u8),
                        Mortal249(u8),
                        Mortal250(u8),
                        Mortal251(u8),
                        Mortal252(u8),
                        Mortal253(u8),
                        Mortal254(u8),
                        Mortal255(u8),
                    }
                }
                pub mod header {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct Header<_0, _1> {
                        pub parent_hash: __runtime_types::primitive_types::H256,
                        pub number: _0,
                        pub state_root: __runtime_types::primitive_types::H256,
                        pub extrinsics_root: __runtime_types::primitive_types::H256,
                        pub digest: __runtime_types::sp_runtime::generic::digest::Digest<
                            __runtime_types::primitive_types::H256,
                        >,
                        pub __chameleon_unused_type_params: core::marker::PhantomData<(_1,)>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::__runtime_types;
                    #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        Vec<u8>,
                        pub core::marker::PhantomData<(_1, _0, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub enum MultiAddress<_0, _1> {
                    Id(_0),
                    Index(_1),
                    Raw(Vec<u8>),
                    Address32([u8; 32usize]),
                    Address20([u8; 20usize]),
                }
            }
            pub mod traits {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct BlakeTwo256 {}
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum ArithmeticError {
                Underflow,
                Overflow,
                DivisionByZero,
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum DispatchError {
                Other,
                CannotLookup,
                BadOrigin,
                Module { index: u8, error: u8 },
                ConsumerRemaining,
                NoProviders,
                Token(__runtime_types::sp_runtime::TokenError),
                Arithmetic(__runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum MultiSignature {
                Ed25519(__runtime_types::sp_core::ed25519::Signature),
                Sr25519(__runtime_types::sp_core::sr25519::Signature),
                Ecdsa(__runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub enum TokenError {
                NoFunds,
                WouldDie,
                BelowMinimum,
                CannotCreate,
                UnknownAsset,
                Frozen,
                Unsupported,
            }
        }
        pub mod sp_session {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct MembershipProof {
                pub session: u32,
                pub trie_nodes: Vec<Vec<u8>>,
                pub validator_count: u32,
            }
        }
        pub mod sp_staking {
            use super::__runtime_types;
            pub mod offence {
                use super::__runtime_types;
                #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
                pub struct OffenceDetails<_0, _1> {
                    pub offender: _1,
                    pub reporters: Vec<_0>,
                }
            }
        }
        pub mod sp_transaction_storage_proof {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct TransactionStorageProof {
                pub chunk: Vec<u8>,
                pub proof: Vec<Vec<u8>>,
            }
        }
        pub mod sp_version {
            use super::__runtime_types;
            #[derive(Debug, :: codec :: Encode, :: codec :: Decode)]
            pub struct RuntimeVersion {
                pub spec_name: String,
                pub impl_name: String,
                pub authoring_version: u32,
                pub spec_version: u32,
                pub impl_version: u32,
                pub apis: Vec<([u8; 8usize], u32)>,
                pub transaction_version: u32,
            }
        }
    }
}
