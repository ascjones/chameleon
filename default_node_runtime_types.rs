#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod node_runtime {
    pub mod __runtime_types {
        use super::__runtime_types;
        pub mod finality_grandpa {
            use super::__runtime_types;
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::__runtime_types;
            pub mod traits {
                use super::__runtime_types;
                pub mod tokens {
                    use super::__runtime_types;
                    pub mod misc {
                        use super::__runtime_types;
                        pub enum BalanceStatus {
                            Free,
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::__runtime_types;
                pub enum DispatchClass {
                    Normal,
                    Operational,
                    Mandatory,
                }
                pub struct DispatchInfo {
                    pub weight: u64,
                    pub class: __runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: __runtime_types::frame_support::weights::Pays,
                }
                pub enum Pays {
                    Yes,
                    No,
                }
            }
            pub enum Never {}
        }
        pub mod frame_system {
            use super::__runtime_types;
            pub mod extensions {
                use super::__runtime_types;
                pub mod check_genesis {
                    use super::__runtime_types;
                    pub struct CheckGenesis<_0>(pub core::marker::PhantomData<_0>);
                }
                pub mod check_mortality {
                    use super::__runtime_types;
                    pub struct CheckMortality<_0>(
                        pub __runtime_types::sp_runtime::generic::era::Era,
                        pub core::marker::PhantomData<_0>,
                    );
                }
                pub mod check_nonce {
                    use super::__runtime_types;
                    pub struct CheckNonce<_0>(pub u32, pub core::marker::PhantomData<(_0,)>);
                }
                pub mod check_spec_version {
                    use super::__runtime_types;
                    pub struct CheckSpecVersion<_0>(pub core::marker::PhantomData<_0>);
                }
                pub mod check_tx_version {
                    use super::__runtime_types;
                    pub struct CheckTxVersion<_0>(pub core::marker::PhantomData<_0>);
                }
                pub mod check_weight {
                    use super::__runtime_types;
                    pub struct CheckWeight<_0>(pub core::marker::PhantomData<_0>);
                }
            }
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    fill_block(__runtime_types::sp_arithmetic::per_things::Perbill),
                    remark(Vec<u8>),
                    set_heap_pages(u64),
                    set_code(Vec<u8>),
                    set_code_without_checks(Vec<u8>),
                    set_changes_trie_config(
                        Option<__runtime_types::sp_core::changes_trie::ChangesTrieConfiguration>,
                    ),
                    set_storage(Vec<(Vec<u8>, Vec<u8>)>),
                    kill_storage(Vec<Vec<u8>>),
                    kill_prefix(Vec<u8>, u32),
                    remark_with_event(Vec<u8>),
                }
                pub enum Event<_0> {
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
                    __Ignore(
                        core::marker::PhantomData<_0>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
        }
        pub mod node_runtime {
            use super::__runtime_types;
            pub enum Call {
                System(
                    __runtime_types::frame_system::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Utility(
                    __runtime_types::pallet_utility::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Babe(
                    __runtime_types::pallet_babe::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Timestamp(
                    __runtime_types::pallet_timestamp::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Authorship(
                    __runtime_types::pallet_authorship::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Indices(
                    __runtime_types::pallet_indices::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Balances(
                    __runtime_types::pallet_balances::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                        (),
                    >,
                ),
                ElectionProviderMultiPhase(
                    __runtime_types::pallet_election_provider_multi_phase::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Staking(
                    __runtime_types::pallet_staking::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Session(
                    __runtime_types::pallet_session::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Democracy(
                    __runtime_types::pallet_democracy::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Council(
                    __runtime_types::pallet_collective::Call<
                        __runtime_types::node_runtime::Runtime,
                        __runtime_types::pallet_collective::Instance1,
                    >,
                ),
                TechnicalCommittee(
                    __runtime_types::pallet_collective::Call<
                        __runtime_types::node_runtime::Runtime,
                        __runtime_types::pallet_collective::Instance2,
                    >,
                ),
                Elections(
                    __runtime_types::pallet_elections_phragmen::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                TechnicalMembership(
                    __runtime_types::pallet_membership::Call<
                        __runtime_types::node_runtime::Runtime,
                        __runtime_types::pallet_membership::Instance1,
                    >,
                ),
                Grandpa(
                    __runtime_types::pallet_grandpa::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Treasury(
                    __runtime_types::pallet_treasury::Call<
                        __runtime_types::node_runtime::Runtime,
                        __runtime_types::pallet_treasury::DefaultInstance,
                    >,
                ),
                Contracts(
                    __runtime_types::pallet_contracts::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Sudo(
                    __runtime_types::pallet_sudo::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                ImOnline(
                    __runtime_types::pallet_im_online::Call<__runtime_types::node_runtime::Runtime>,
                ),
                AuthorityDiscovery(
                    __runtime_types::pallet_authority_discovery::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Offences(
                    __runtime_types::pallet_offences::Call<__runtime_types::node_runtime::Runtime>,
                ),
                RandomnessCollectiveFlip(
                    __runtime_types::pallet_randomness_collective_flip::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Identity(
                    __runtime_types::pallet_identity::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Society(
                    __runtime_types::pallet_society::Call<
                        __runtime_types::node_runtime::Runtime,
                        __runtime_types::pallet_society::DefaultInstance,
                    >,
                ),
                Recovery(
                    __runtime_types::pallet_recovery::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Vesting(
                    __runtime_types::pallet_vesting::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Scheduler(
                    __runtime_types::pallet_scheduler::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Proxy(
                    __runtime_types::pallet_proxy::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Multisig(
                    __runtime_types::pallet_multisig::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Bounties(
                    __runtime_types::pallet_bounties::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Tips(__runtime_types::pallet_tips::Call<__runtime_types::node_runtime::Runtime>),
                Assets(
                    __runtime_types::pallet_assets::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                Lottery(
                    __runtime_types::pallet_lottery::Call<__runtime_types::node_runtime::Runtime>,
                ),
                Gilt(
                    __runtime_types::pallet_gilt::pallet::Call<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
            }
            pub enum Event {
                frame_system(
                    __runtime_types::frame_system::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                pallet_utility(__runtime_types::pallet_utility::pallet::Event),
                pallet_indices(
                    __runtime_types::pallet_indices::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                pallet_balances(
                    __runtime_types::pallet_balances::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                        (),
                    >,
                ),
                pallet_election_provider_multi_phase(
                    __runtime_types::pallet_election_provider_multi_phase::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                pallet_staking(
                    __runtime_types::pallet_staking::RawEvent<
                        u128,
                        __runtime_types::sp_core::crypto::AccountId32,
                    >,
                ),
                pallet_session(__runtime_types::pallet_session::Event),
                pallet_democracy(
                    __runtime_types::pallet_democracy::RawEvent<
                        u128,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                        u32,
                    >,
                ),
                pallet_collective_Instance1(
                    __runtime_types::pallet_collective::RawEvent<
                        __runtime_types::primitive_types::H256,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_collective::Instance1,
                    >,
                ),
                pallet_collective_Instance2(
                    __runtime_types::pallet_collective::RawEvent<
                        __runtime_types::primitive_types::H256,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_collective::Instance2,
                    >,
                ),
                pallet_elections_phragmen(
                    __runtime_types::pallet_elections_phragmen::RawEvent<
                        u128,
                        __runtime_types::sp_core::crypto::AccountId32,
                    >,
                ),
                pallet_membership_Instance1(
                    __runtime_types::pallet_membership::RawEvent<
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::node_runtime::Event,
                        __runtime_types::pallet_membership::Instance1,
                    >,
                ),
                pallet_grandpa(__runtime_types::pallet_grandpa::Event),
                pallet_treasury(
                    __runtime_types::pallet_treasury::RawEvent<
                        u128,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_treasury::DefaultInstance,
                    >,
                ),
                pallet_contracts(
                    __runtime_types::pallet_contracts::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                pallet_sudo(
                    __runtime_types::pallet_sudo::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                pallet_im_online(
                    __runtime_types::pallet_im_online::RawEvent<
                        __runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                        (
                            __runtime_types::sp_core::crypto::AccountId32,
                            __runtime_types::pallet_staking::Exposure<
                                __runtime_types::sp_core::crypto::AccountId32,
                                u128,
                            >,
                        ),
                    >,
                ),
                pallet_offences(__runtime_types::pallet_offences::Event),
                pallet_identity(
                    __runtime_types::pallet_identity::RawEvent<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    >,
                ),
                pallet_society(
                    __runtime_types::pallet_society::RawEvent<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                        __runtime_types::pallet_society::DefaultInstance,
                    >,
                ),
                pallet_recovery(
                    __runtime_types::pallet_recovery::RawEvent<
                        __runtime_types::sp_core::crypto::AccountId32,
                    >,
                ),
                pallet_vesting(
                    __runtime_types::pallet_vesting::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                pallet_scheduler(__runtime_types::pallet_scheduler::RawEvent<u32>),
                pallet_proxy(
                    __runtime_types::pallet_proxy::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                pallet_multisig(
                    __runtime_types::pallet_multisig::RawEvent<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                        [u8; 32usize],
                    >,
                ),
                pallet_bounties(
                    __runtime_types::pallet_bounties::RawEvent<
                        u128,
                        __runtime_types::sp_core::crypto::AccountId32,
                    >,
                ),
                pallet_tips(
                    __runtime_types::pallet_tips::RawEvent<
                        u128,
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                    >,
                ),
                pallet_assets(
                    __runtime_types::pallet_assets::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
                pallet_lottery(
                    __runtime_types::pallet_lottery::RawEvent<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    >,
                ),
                pallet_gilt(
                    __runtime_types::pallet_gilt::pallet::Event<
                        __runtime_types::node_runtime::Runtime,
                    >,
                ),
            }
            pub enum ProxyType {
                Any,
                NonTransfer,
                Governance,
                Staking,
            }
            pub struct Runtime {}
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
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    create(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u64,
                    ),
                    force_create(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        bool,
                        u64,
                    ),
                    destroy(u32, __runtime_types::pallet_assets::types::DestroyWitness),
                    mint(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u64,
                    ),
                    burn(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u64,
                    ),
                    transfer(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u64,
                    ),
                    transfer_keep_alive(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u64,
                    ),
                    force_transfer(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u64,
                    ),
                    freeze(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    ),
                    thaw(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    ),
                    freeze_asset(u32),
                    thaw_asset(u32),
                    transfer_ownership(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    ),
                    set_team(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    ),
                    set_metadata(u32, Vec<u8>, Vec<u8>, u8),
                    clear_metadata(u32),
                    force_set_metadata(u32, Vec<u8>, Vec<u8>, u8, bool),
                    force_clear_metadata(u32),
                    force_asset_status(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u64,
                        bool,
                        bool,
                    ),
                    approve_transfer(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u64,
                    ),
                    cancel_approval(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    ),
                    force_cancel_approval(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    ),
                    transfer_approved(
                        u32,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u64,
                    ),
                }
                pub enum Event<_0> {
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
                    __Ignore(
                        core::marker::PhantomData<_0>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
            pub mod types {
                use super::__runtime_types;
                pub struct DestroyWitness {
                    pub accounts: u32,
                    pub sufficients: u32,
                    pub approvals: u32,
                }
            }
        }
        pub mod pallet_authority_discovery {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
            }
        }
        pub mod pallet_authorship {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                set_uncles(Vec<__runtime_types::sp_runtime::generic::header::Header>),
            }
        }
        pub mod pallet_babe {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    report_equivocation(
                        __runtime_types::sp_consensus_slots::EquivocationProof<
                            __runtime_types::sp_runtime::generic::header::Header,
                            __runtime_types::sp_consensus_babe::app::Public,
                        >,
                        __runtime_types::sp_session::MembershipProof,
                    ),
                    report_equivocation_unsigned(
                        __runtime_types::sp_consensus_slots::EquivocationProof<
                            __runtime_types::sp_runtime::generic::header::Header,
                            __runtime_types::sp_consensus_babe::app::Public,
                        >,
                        __runtime_types::sp_session::MembershipProof,
                    ),
                    plan_config_change(
                        __runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    ),
                }
            }
        }
        pub mod pallet_balances {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0, _1> {
                    __Ignore(
                        core::marker::PhantomData<(_0, _1)>,
                        __runtime_types::frame_support::Never,
                    ),
                    transfer(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u128,
                    ),
                    set_balance(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u128,
                        u128,
                    ),
                    force_transfer(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u128,
                    ),
                    transfer_keep_alive(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u128,
                    ),
                }
                pub enum Event<_0, _1> {
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
                    __Ignore(
                        core::marker::PhantomData<(_0, _1)>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
        }
        pub mod pallet_bounties {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                propose_bounty(u128, Vec<u8>),
                approve_bounty(u32),
                propose_curator(
                    u32,
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    u128,
                ),
                unassign_curator(u32),
                accept_curator(u32),
                award_bounty(
                    u32,
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                ),
                claim_bounty(u32),
                close_bounty(u32),
                extend_bounty_expiry(u32, Vec<u8>),
            }
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
            pub enum Call<_0, _1> {
                __PhantomItem(
                    core::marker::PhantomData<(_0, _1)>,
                    __runtime_types::frame_support::Never,
                ),
                set_members(
                    Vec<__runtime_types::sp_core::crypto::AccountId32>,
                    Option<__runtime_types::sp_core::crypto::AccountId32>,
                    u32,
                ),
                execute(__runtime_types::node_runtime::Call, u32),
                propose(u32, __runtime_types::node_runtime::Call, u32),
                vote(__runtime_types::primitive_types::H256, u32, bool),
                close(__runtime_types::primitive_types::H256, u32, u64, u32),
                disapprove_proposal(__runtime_types::primitive_types::H256),
            }
            pub struct Instance1 {}
            pub struct Instance2 {}
            pub enum RawEvent<_0, _1, _2> {
                Proposed(_1, u32, _0, u32),
                Voted(_1, _0, bool, u32, u32),
                Approved(_0),
                Disapproved(_0),
                Executed(_0, Result<(), __runtime_types::sp_runtime::DispatchError>),
                MemberExecuted(_0, Result<(), __runtime_types::sp_runtime::DispatchError>),
                Closed(_0, u32, u32),
                PhantomData(core::marker::PhantomData<_2>),
            }
        }
        pub mod pallet_contracts {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    update_schedule(__runtime_types::pallet_contracts::schedule::Schedule<_0>),
                    call(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        u128,
                        u64,
                        Vec<u8>,
                    ),
                    instantiate_with_code(u128, u64, Vec<u8>, Vec<u8>, Vec<u8>),
                    instantiate(
                        u128,
                        u64,
                        __runtime_types::primitive_types::H256,
                        Vec<u8>,
                        Vec<u8>,
                    ),
                    claim_surcharge(
                        __runtime_types::sp_core::crypto::AccountId32,
                        Option<__runtime_types::sp_core::crypto::AccountId32>,
                    ),
                }
                pub enum Event<_0> {
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
                    __Ignore(
                        core::marker::PhantomData<_0>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
            pub mod schedule {
                use super::__runtime_types;
                pub struct HostFnWeights<_0> {
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
                    pub terminate_per_code_byte: u64,
                    pub restore_to: u64,
                    pub restore_to_per_caller_code_byte: u64,
                    pub restore_to_per_tombstone_code_byte: u64,
                    pub restore_to_per_delta: u64,
                    pub random: u64,
                    pub deposit_event: u64,
                    pub deposit_event_per_topic: u64,
                    pub deposit_event_per_byte: u64,
                    pub set_rent_allowance: u64,
                    pub set_storage: u64,
                    pub set_storage_per_byte: u64,
                    pub clear_storage: u64,
                    pub get_storage: u64,
                    pub get_storage_per_byte: u64,
                    pub transfer: u64,
                    pub call: u64,
                    pub call_per_code_byte: u64,
                    pub call_transfer_surcharge: u64,
                    pub call_per_input_byte: u64,
                    pub call_per_output_byte: u64,
                    pub instantiate: u64,
                    pub instantiate_per_code_byte: u64,
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
                    pub rent_params: u64,
                    pub _phantom: core::marker::PhantomData<_0>,
                }
                pub struct InstructionWeights<_0> {
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
                    pub _phantom: core::marker::PhantomData<_0>,
                }
                pub struct Limits {
                    pub event_topics: u32,
                    pub stack_height: u32,
                    pub globals: u32,
                    pub parameters: u32,
                    pub memory_pages: u32,
                    pub table_size: u32,
                    pub br_table_size: u32,
                    pub subject_len: u32,
                }
                pub struct Schedule<_0> {
                    pub version: u32,
                    pub enable_println: bool,
                    pub limits: __runtime_types::pallet_contracts::schedule::Limits,
                    pub instruction_weights:
                        __runtime_types::pallet_contracts::schedule::InstructionWeights<_0>,
                    pub host_fn_weights:
                        __runtime_types::pallet_contracts::schedule::HostFnWeights<_0>,
                }
            }
        }
        pub mod pallet_democracy {
            use super::__runtime_types;
            pub mod conviction {
                use super::__runtime_types;
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
            pub mod vote {
                use super::__runtime_types;
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
                pub struct Vote {
                    pub aye: bool,
                    pub conviction: __runtime_types::pallet_democracy::conviction::Conviction,
                }
            }
            pub mod vote_threshold {
                use super::__runtime_types;
                pub enum VoteThreshold {
                    SuperMajorityApprove,
                    SuperMajorityAgainst,
                    SimpleMajority,
                }
            }
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                propose(__runtime_types::primitive_types::H256, u128),
                second(u32, u32),
                vote(
                    u32,
                    __runtime_types::pallet_democracy::vote::AccountVote<u128>,
                ),
                emergency_cancel(u32),
                external_propose(__runtime_types::primitive_types::H256),
                external_propose_majority(__runtime_types::primitive_types::H256),
                external_propose_default(__runtime_types::primitive_types::H256),
                fast_track(__runtime_types::primitive_types::H256, u32, u32),
                veto_external(__runtime_types::primitive_types::H256),
                cancel_referendum(u32),
                cancel_queued(u32),
                delegate(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::pallet_democracy::conviction::Conviction,
                    u128,
                ),
                undelegate,
                clear_public_proposals,
                note_preimage(Vec<u8>),
                note_preimage_operational(Vec<u8>),
                note_imminent_preimage(Vec<u8>),
                note_imminent_preimage_operational(Vec<u8>),
                reap_preimage(__runtime_types::primitive_types::H256, u32),
                unlock(__runtime_types::sp_core::crypto::AccountId32),
                remove_vote(u32),
                remove_other_vote(__runtime_types::sp_core::crypto::AccountId32, u32),
                enact_proposal(__runtime_types::primitive_types::H256, u32),
                blacklist(__runtime_types::primitive_types::H256, Option<u32>),
                cancel_proposal(u32),
            }
            pub enum RawEvent<_0, _1, _2, _3> {
                Proposed(_3, _0),
                Tabled(_3, _0, Vec<_1>),
                ExternalTabled,
                Started(
                    _3,
                    __runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                ),
                Passed(_3),
                NotPassed(_3),
                Cancelled(_3),
                Executed(_3, bool),
                Delegated(_1, _1),
                Undelegated(_1),
                Vetoed(_1, _2, _3),
                PreimageNoted(_2, _1, _0),
                PreimageUsed(_2, _1, _0),
                PreimageInvalid(_2, _3),
                PreimageMissing(_2, _3),
                PreimageReaped(_2, _1, _0, _1),
                Unlocked(_1),
                Blacklisted(_2),
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore (core :: marker :: PhantomData < (_0 ,) > , __runtime_types :: frame_support :: Never ,) , submit_unsigned (__runtime_types :: pallet_election_provider_multi_phase :: RawSolution < () > , __runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize ,) , }
                pub enum Event<_0> {
                    SolutionStored(
                        __runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                    ),
                    ElectionFinalized(
                        Option<
                            __runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                        >,
                    ),
                    Rewarded(__runtime_types::sp_core::crypto::AccountId32),
                    Slashed(__runtime_types::sp_core::crypto::AccountId32),
                    SignedPhaseStarted(u32),
                    UnsignedPhaseStarted(u32),
                    __Ignore(
                        core::marker::PhantomData<_0>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
            pub enum ElectionCompute {
                OnChain,
                Signed,
                Unsigned,
            }
            pub struct RawSolution<_0> {
                pub compact: _0,
                pub score: [u128; 3usize],
                pub round: u32,
            }
            pub struct SolutionOrSnapshotSize {
                pub voters: u32,
                pub targets: u32,
            }
        }
        pub mod pallet_elections_phragmen {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                vote(Vec<__runtime_types::sp_core::crypto::AccountId32>, u128),
                remove_voter,
                submit_candidacy(u32),
                renounce_candidacy(__runtime_types::pallet_elections_phragmen::Renouncing),
                remove_member(
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    bool,
                ),
                clean_defunct_voters(u32, u32),
            }
            pub enum RawEvent<_0, _1> {
                NewTerm(Vec<(_1, _0)>),
                EmptyTerm,
                ElectionError,
                MemberKicked(_1),
                Renounced(_1),
                CandidateSlashed(_1, _0),
                SeatHolderSlashed(_1, _0),
            }
            pub enum Renouncing {
                Member,
                RunnerUp,
                Candidate(u32),
            }
        }
        pub mod pallet_gilt {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    place_bid(u128, u32),
                    retract_bid(u128, u32),
                    set_target(__runtime_types::sp_arithmetic::per_things::Perquintill),
                    thaw(u32),
                }
                pub enum Event<_0> {
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
                    __Ignore(
                        core::marker::PhantomData<_0>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
        }
        pub mod pallet_grandpa {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                report_equivocation(
                    __runtime_types::sp_finality_grandpa::EquivocationProof<
                        __runtime_types::primitive_types::H256,
                        u32,
                    >,
                    __runtime_types::sp_session::MembershipProof,
                ),
                report_equivocation_unsigned(
                    __runtime_types::sp_finality_grandpa::EquivocationProof<
                        __runtime_types::primitive_types::H256,
                        u32,
                    >,
                    __runtime_types::sp_session::MembershipProof,
                ),
                note_stalled(u32, u32),
            }
            pub enum Event {
                NewAuthorities(Vec<(__runtime_types::sp_finality_grandpa::app::Public, u64)>),
                Paused,
                Resumed,
            }
        }
        pub mod pallet_identity {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                add_registrar(__runtime_types::sp_core::crypto::AccountId32),
                set_identity(__runtime_types::pallet_identity::IdentityInfo),
                set_subs(
                    Vec<(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::pallet_identity::Data,
                    )>,
                ),
                clear_identity,
                request_judgement(u32, u128),
                cancel_request(u32),
                set_fee(u32, u128),
                set_account_id(u32, __runtime_types::sp_core::crypto::AccountId32),
                set_fields(u32, __runtime_types::pallet_identity::IdentityFields),
                provide_judgement(
                    u32,
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    __runtime_types::pallet_identity::Judgement<u128>,
                ),
                kill_identity(
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                ),
                add_sub(
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    __runtime_types::pallet_identity::Data,
                ),
                rename_sub(
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    __runtime_types::pallet_identity::Data,
                ),
                remove_sub(
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                ),
                quit_sub,
            }
            pub enum Data {
                None,
                Raw(Vec<u8>),
                BlakeTwo256([u8; 32usize]),
                Sha256([u8; 32usize]),
                Keccak256([u8; 32usize]),
                ShaThree256([u8; 32usize]),
            }
            pub struct IdentityFields(pub u64);
            pub struct IdentityInfo {
                pub additional: Vec<(
                    __runtime_types::pallet_identity::Data,
                    __runtime_types::pallet_identity::Data,
                )>,
                pub display: __runtime_types::pallet_identity::Data,
                pub legal: __runtime_types::pallet_identity::Data,
                pub web: __runtime_types::pallet_identity::Data,
                pub riot: __runtime_types::pallet_identity::Data,
                pub email: __runtime_types::pallet_identity::Data,
                pub pgp_fingerprint: Option<[u8; 20usize]>,
                pub image: __runtime_types::pallet_identity::Data,
                pub twitter: __runtime_types::pallet_identity::Data,
            }
            pub enum Judgement<_0> {
                Unknown,
                FeePaid(_0),
                Reasonable,
                KnownGood,
                OutOfDate,
                LowQuality,
                Erroneous,
            }
            pub enum RawEvent<_0, _1> {
                IdentitySet(_0),
                IdentityCleared(_0, _1),
                IdentityKilled(_0, _1),
                JudgementRequested(_0, u32),
                JudgementUnrequested(_0, u32),
                JudgementGiven(_0, u32),
                RegistrarAdded(u32),
                SubIdentityAdded(_0, _0, _1),
                SubIdentityRemoved(_0, _0, _1),
                SubIdentityRevoked(_0, _0, _1),
            }
        }
        pub mod pallet_im_online {
            use super::__runtime_types;
            pub mod sr25519 {
                use super::__runtime_types;
                pub mod app_sr25519 {
                    use super::__runtime_types;
                    pub struct Public(pub __runtime_types::sp_core::sr25519::Public);
                    pub struct Signature(pub __runtime_types::sp_core::sr25519::Signature);
                }
            }
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                heartbeat(
                    __runtime_types::pallet_im_online::Heartbeat<u32>,
                    __runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                ),
            }
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state: __runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
            pub enum RawEvent<_0, _1> {
                HeartbeatReceived(_0),
                AllGood,
                SomeOffline(Vec<_1>),
            }
        }
        pub mod pallet_indices {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    claim(u32),
                    transfer(__runtime_types::sp_core::crypto::AccountId32, u32),
                    free(u32),
                    force_transfer(__runtime_types::sp_core::crypto::AccountId32, u32, bool),
                    freeze(u32),
                }
                pub enum Event<_0> {
                    IndexAssigned(__runtime_types::sp_core::crypto::AccountId32, u32),
                    IndexFreed(u32),
                    IndexFrozen(u32, __runtime_types::sp_core::crypto::AccountId32),
                    __Ignore(
                        core::marker::PhantomData<_0>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
        }
        pub mod pallet_lottery {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                buy_ticket(__runtime_types::node_runtime::Call),
                set_calls(Vec<__runtime_types::node_runtime::Call>),
                start_lottery(u128, u32, u32, bool),
                stop_repeat,
            }
            pub enum RawEvent<_0, _1> {
                LotteryStarted,
                CallsUpdated,
                Winner(_0, _1),
                TicketBought(_0, (u8, u8)),
            }
        }
        pub mod pallet_membership {
            use super::__runtime_types;
            pub enum Call<_0, _1> {
                __PhantomItem(
                    core::marker::PhantomData<(_0, _1)>,
                    __runtime_types::frame_support::Never,
                ),
                add_member(__runtime_types::sp_core::crypto::AccountId32),
                remove_member(__runtime_types::sp_core::crypto::AccountId32),
                swap_member(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::sp_core::crypto::AccountId32,
                ),
                reset_members(Vec<__runtime_types::sp_core::crypto::AccountId32>),
                change_key(__runtime_types::sp_core::crypto::AccountId32),
                set_prime(__runtime_types::sp_core::crypto::AccountId32),
                clear_prime,
            }
            pub struct Instance1 {}
            pub enum RawEvent<_0, _1, _2> {
                MemberAdded,
                MemberRemoved,
                MembersSwapped,
                MembersReset,
                KeyChanged,
                Dummy(core::marker::PhantomData<(_0, _1)>),
                PhantomData(core::marker::PhantomData<_2>),
            }
        }
        pub mod pallet_multisig {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                as_multi_threshold_1(
                    Vec<__runtime_types::sp_core::crypto::AccountId32>,
                    __runtime_types::node_runtime::Call,
                ),
                as_multi(
                    u16,
                    Vec<__runtime_types::sp_core::crypto::AccountId32>,
                    Option<__runtime_types::pallet_multisig::Timepoint<u32>>,
                    Vec<u8>,
                    bool,
                    u64,
                ),
                approve_as_multi(
                    u16,
                    Vec<__runtime_types::sp_core::crypto::AccountId32>,
                    Option<__runtime_types::pallet_multisig::Timepoint<u32>>,
                    [u8; 32usize],
                    u64,
                ),
                cancel_as_multi(
                    u16,
                    Vec<__runtime_types::sp_core::crypto::AccountId32>,
                    __runtime_types::pallet_multisig::Timepoint<u32>,
                    [u8; 32usize],
                ),
            }
            pub enum RawEvent<_0, _1, _2> {
                NewMultisig(_0, _0, _2),
                MultisigApproval(_0, __runtime_types::pallet_multisig::Timepoint<_1>, _0, _2),
                MultisigExecuted(
                    _0,
                    __runtime_types::pallet_multisig::Timepoint<_1>,
                    _0,
                    _2,
                    Result<(), __runtime_types::sp_runtime::DispatchError>,
                ),
                MultisigCancelled(_0, __runtime_types::pallet_multisig::Timepoint<_1>, _0, _2),
            }
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_offences {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
            }
            pub enum Event {
                Offence([u8; 16usize], Vec<u8>, bool),
            }
        }
        pub mod pallet_proxy {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    proxy(
                        __runtime_types::sp_core::crypto::AccountId32,
                        Option<__runtime_types::node_runtime::ProxyType>,
                        __runtime_types::node_runtime::Call,
                    ),
                    add_proxy(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::node_runtime::ProxyType,
                        u32,
                    ),
                    remove_proxy(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::node_runtime::ProxyType,
                        u32,
                    ),
                    remove_proxies,
                    anonymous(__runtime_types::node_runtime::ProxyType, u32, u16),
                    kill_anonymous(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::node_runtime::ProxyType,
                        u16,
                        u32,
                        u32,
                    ),
                    announce(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                    ),
                    remove_announcement(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                    ),
                    reject_announcement(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::primitive_types::H256,
                    ),
                    proxy_announced(
                        __runtime_types::sp_core::crypto::AccountId32,
                        __runtime_types::sp_core::crypto::AccountId32,
                        Option<__runtime_types::node_runtime::ProxyType>,
                        __runtime_types::node_runtime::Call,
                    ),
                }
                pub enum Event<_0> {
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
                    __Ignore(
                        core::marker::PhantomData<_0>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
        }
        pub mod pallet_randomness_collective_flip {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
            }
        }
        pub mod pallet_recovery {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                as_recovered(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::node_runtime::Call,
                ),
                set_recovered(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::sp_core::crypto::AccountId32,
                ),
                create_recovery(Vec<__runtime_types::sp_core::crypto::AccountId32>, u16, u32),
                initiate_recovery(__runtime_types::sp_core::crypto::AccountId32),
                vouch_recovery(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::sp_core::crypto::AccountId32,
                ),
                claim_recovery(__runtime_types::sp_core::crypto::AccountId32),
                close_recovery(__runtime_types::sp_core::crypto::AccountId32),
                remove_recovery,
                cancel_recovered(__runtime_types::sp_core::crypto::AccountId32),
            }
            pub enum RawEvent<_0> {
                RecoveryCreated(_0),
                RecoveryInitiated(_0, _0),
                RecoveryVouched(_0, _0, _0),
                RecoveryClosed(_0, _0),
                AccountRecovered(_0, _0),
                RecoveryRemoved(_0),
            }
        }
        pub mod pallet_scheduler {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                schedule(
                    u32,
                    Option<(u32, u32)>,
                    u8,
                    __runtime_types::node_runtime::Call,
                ),
                cancel(u32, u32),
                schedule_named(
                    Vec<u8>,
                    u32,
                    Option<(u32, u32)>,
                    u8,
                    __runtime_types::node_runtime::Call,
                ),
                cancel_named(Vec<u8>),
                schedule_after(
                    u32,
                    Option<(u32, u32)>,
                    u8,
                    __runtime_types::node_runtime::Call,
                ),
                schedule_named_after(
                    Vec<u8>,
                    u32,
                    Option<(u32, u32)>,
                    u8,
                    __runtime_types::node_runtime::Call,
                ),
            }
            pub enum RawEvent<_0> {
                Scheduled(_0, _0),
                Canceled(_0, _0),
                Dispatched(
                    (_0, _0),
                    Option<Vec<u8>>,
                    Result<(), __runtime_types::sp_runtime::DispatchError>,
                ),
            }
        }
        pub mod pallet_session {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                set_keys(__runtime_types::node_runtime::SessionKeys, Vec<u8>),
                purge_keys,
            }
            pub enum Event {
                NewSession(u32),
            }
        }
        pub mod pallet_society {
            use super::__runtime_types;
            pub enum Call<_0, _1> {
                __PhantomItem(
                    core::marker::PhantomData<(_0, _1)>,
                    __runtime_types::frame_support::Never,
                ),
                bid(u128),
                unbid(u32),
                vouch(__runtime_types::sp_core::crypto::AccountId32, u128, u128),
                unvouch(u32),
                vote(
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    bool,
                ),
                defender_vote(bool),
                payout,
                found(__runtime_types::sp_core::crypto::AccountId32, u32, Vec<u8>),
                unfound,
                judge_suspended_member(__runtime_types::sp_core::crypto::AccountId32, bool),
                judge_suspended_candidate(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::pallet_society::Judgement,
                ),
                set_max_members(u32),
            }
            pub struct DefaultInstance {}
            pub enum Judgement {
                Rebid,
                Reject,
                Approve,
            }
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
                PhantomData(core::marker::PhantomData<_2>),
            }
        }
        pub mod pallet_staking {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                bond(
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                    u128,
                    __runtime_types::pallet_staking::RewardDestination<
                        __runtime_types::sp_core::crypto::AccountId32,
                    >,
                ),
                bond_extra(u128),
                unbond(u128),
                withdraw_unbonded(u32),
                validate(__runtime_types::pallet_staking::ValidatorPrefs),
                nominate(
                    Vec<
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    >,
                ),
                chill,
                set_payee(
                    __runtime_types::pallet_staking::RewardDestination<
                        __runtime_types::sp_core::crypto::AccountId32,
                    >,
                ),
                set_controller(
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                ),
                set_validator_count(u32),
                increase_validator_count(u32),
                scale_validator_count(__runtime_types::sp_arithmetic::per_things::Percent),
                force_no_eras,
                force_new_era,
                set_invulnerables(Vec<__runtime_types::sp_core::crypto::AccountId32>),
                force_unstake(__runtime_types::sp_core::crypto::AccountId32, u32),
                force_new_era_always,
                cancel_deferred_slash(u32, Vec<u32>),
                payout_stakers(__runtime_types::sp_core::crypto::AccountId32, u32),
                rebond(u128),
                set_history_depth(u32, u32),
                reap_stash(__runtime_types::sp_core::crypto::AccountId32, u32),
                kick(
                    Vec<
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    >,
                ),
            }
            pub struct Exposure<_0, _1> {
                pub total: _1,
                pub own: _1,
                pub others: Vec<__runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
            }
            pub struct IndividualExposure<_0, _1> {
                pub who: _0,
                pub value: _1,
            }
            pub enum RawEvent<_0, _1> {
                EraPayout(u32, _0, _0),
                Reward(_1, _0),
                Slash(_1, _0),
                OldSlashingReportDiscarded(u32),
                StakingElection,
                Bonded(_1, _0),
                Unbonded(_1, _0),
                Withdrawn(_1, _0),
                Kicked(_1, _1),
            }
            pub enum RewardDestination<_0> {
                Staked,
                Stash,
                Controller,
                Account(_0),
                None,
            }
            pub struct ValidatorPrefs {
                pub commission: __runtime_types::sp_arithmetic::per_things::Perbill,
                pub blocked: bool,
            }
        }
        pub mod pallet_sudo {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    sudo(__runtime_types::node_runtime::Call),
                    sudo_unchecked_weight(__runtime_types::node_runtime::Call, u64),
                    set_key(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    ),
                    sudo_as(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::node_runtime::Call,
                    ),
                }
                pub enum Event<_0> {
                    Sudid(Result<(), __runtime_types::sp_runtime::DispatchError>),
                    KeyChanged(__runtime_types::sp_core::crypto::AccountId32),
                    SudoAsDone(Result<(), __runtime_types::sp_runtime::DispatchError>),
                    __Ignore(
                        core::marker::PhantomData<_0>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    set(u64),
                }
            }
        }
        pub mod pallet_tips {
            use super::__runtime_types;
            pub enum Call<_0> {
                __PhantomItem(
                    core::marker::PhantomData<(_0,)>,
                    __runtime_types::frame_support::Never,
                ),
                report_awesome(Vec<u8>, __runtime_types::sp_core::crypto::AccountId32),
                retract_tip(__runtime_types::primitive_types::H256),
                tip_new(Vec<u8>, __runtime_types::sp_core::crypto::AccountId32, u128),
                tip(__runtime_types::primitive_types::H256, u128),
                close_tip(__runtime_types::primitive_types::H256),
                slash_tip(__runtime_types::primitive_types::H256),
            }
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
            pub struct ChargeTransactionPayment<_0>(pub u128, pub core::marker::PhantomData<(_0,)>);
        }
        pub mod pallet_treasury {
            use super::__runtime_types;
            pub enum Call<_0, _1> {
                __PhantomItem(
                    core::marker::PhantomData<(_0, _1)>,
                    __runtime_types::frame_support::Never,
                ),
                propose_spend(
                    u128,
                    __runtime_types::sp_runtime::multiaddress::MultiAddress<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u32,
                    >,
                ),
                reject_proposal(u32),
                approve_proposal(u32),
            }
            pub struct DefaultInstance {}
            pub enum RawEvent<_0, _1, _2> {
                Proposed(u32),
                Spending(_0),
                Awarded(u32, _0, _1),
                Rejected(u32, _0),
                Burnt(_0),
                Rollover(_0),
                Deposit(_0),
                PhantomData(core::marker::PhantomData<_2>),
            }
        }
        pub mod pallet_utility {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    batch(Vec<__runtime_types::node_runtime::Call>),
                    as_derivative(u16, __runtime_types::node_runtime::Call),
                    batch_all(Vec<__runtime_types::node_runtime::Call>),
                }
                pub enum Event {
                    BatchInterrupted(u32, __runtime_types::sp_runtime::DispatchError),
                    BatchCompleted,
                }
            }
        }
        pub mod pallet_vesting {
            use super::__runtime_types;
            pub mod pallet {
                use super::__runtime_types;
                pub enum Call<_0> {
                    __Ignore(
                        core::marker::PhantomData<(_0,)>,
                        __runtime_types::frame_support::Never,
                    ),
                    vest,
                    vest_other(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                    ),
                    vested_transfer(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::pallet_vesting::VestingInfo<u128, u32>,
                    ),
                    force_vested_transfer(
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::sp_runtime::multiaddress::MultiAddress<
                            __runtime_types::sp_core::crypto::AccountId32,
                            u32,
                        >,
                        __runtime_types::pallet_vesting::VestingInfo<u128, u32>,
                    ),
                }
                pub enum Event<_0> {
                    VestingUpdated(__runtime_types::sp_core::crypto::AccountId32, u128),
                    VestingCompleted(__runtime_types::sp_core::crypto::AccountId32),
                    __Ignore(
                        core::marker::PhantomData<_0>,
                        __runtime_types::frame_support::Never,
                    ),
                }
            }
            pub struct VestingInfo<_0, _1> {
                pub locked: _0,
                pub per_block: _0,
                pub starting_block: _1,
            }
        }
        pub mod primitive_types {
            use super::__runtime_types;
            pub struct H256(pub [u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::__runtime_types;
            pub mod per_things {
                use super::__runtime_types;
                pub struct Perbill(pub u32);
                pub struct Percent(pub u8);
                pub struct Perquintill(pub u64);
            }
        }
        pub mod sp_authority_discovery {
            use super::__runtime_types;
            pub mod app {
                use super::__runtime_types;
                pub struct Public(pub __runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod sp_consensus_babe {
            use super::__runtime_types;
            pub mod app {
                use super::__runtime_types;
                pub struct Public(pub __runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::__runtime_types;
                pub enum NextConfigDescriptor {
                    V1 {
                        c: (u64, u64),
                        allowed_slots: __runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            pub enum AllowedSlots {
                PrimarySlots,
                PrimaryAndSecondaryPlainSlots,
                PrimaryAndSecondaryVRFSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::__runtime_types;
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: __runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            pub struct Slot(pub u64);
        }
        pub mod sp_core {
            use super::__runtime_types;
            pub mod changes_trie {
                use super::__runtime_types;
                pub struct ChangesTrieConfiguration {
                    pub digest_interval: u32,
                    pub digest_levels: u32,
                }
            }
            pub mod crypto {
                use super::__runtime_types;
                pub struct AccountId32(pub [u8; 32usize]);
            }
            pub mod ed25519 {
                use super::__runtime_types;
                pub struct Public(pub [u8; 32usize]);
                pub struct Signature(pub [u8; 64usize]);
            }
            pub mod offchain {
                use super::__runtime_types;
                pub struct OpaqueMultiaddr(pub Vec<u8>);
                pub struct OpaqueNetworkState {
                    pub peer_id: __runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        Vec<__runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::__runtime_types;
                pub struct Public(pub [u8; 32usize]);
                pub struct Signature(pub [u8; 64usize]);
            }
            pub struct OpaquePeerId(pub Vec<u8>);
        }
        pub mod sp_finality_grandpa {
            use super::__runtime_types;
            pub mod app {
                use super::__runtime_types;
                pub struct Public(pub __runtime_types::sp_core::ed25519::Public);
                pub struct Signature(pub __runtime_types::sp_core::ed25519::Signature);
            }
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
            pub struct EquivocationProof<_0, _1> {
                pub set_id: u64,
                pub equivocation: __runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_runtime {
            use super::__runtime_types;
            pub mod generic {
                use super::__runtime_types;
                pub mod era {
                    use super::__runtime_types;
                    pub enum Era {
                        Immortal,
                        Mortal(u64, u64),
                    }
                }
                pub mod header {
                    use super::__runtime_types;
                    pub struct Header {}
                }
            }
            pub mod multiaddress {
                use super::__runtime_types;
                pub enum MultiAddress<_0, _1> {
                    Id(_0),
                    Index(_1),
                    Raw(Vec<u8>),
                    Address32([u8; 32usize]),
                    Address20([u8; 20usize]),
                }
            }
            pub enum DispatchError {
                Other(String),
                CannotLookup,
                BadOrigin,
                Module {
                    index: u8,
                    error: u8,
                    message: Option<String>,
                },
                ConsumerRemaining,
                NoProviders,
                Token(__runtime_types::sp_runtime::TokenError),
            }
            pub enum TokenError {
                NoFunds,
                WouldDie,
                BelowMinimum,
                CannotCreate,
                UnknownAsset,
                Frozen,
                Underflow,
                Overflow,
            }
        }
        pub mod sp_session {
            use super::__runtime_types;
            pub struct MembershipProof {
                pub session: u32,
                pub trie_nodes: Vec<Vec<u8>>,
                pub validator_count: u32,
            }
        }
    }
    pub mod system {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct FillBlock {
                _ratio: __runtime_types::sp_arithmetic::per_things::Perbill,
            }
            pub struct Remark {
                _remark: Vec<u8>,
            }
            pub struct SetHeapPages {
                pages: u64,
            }
            pub struct SetCode {
                code: Vec<u8>,
            }
            pub struct SetCodeWithoutChecks {
                code: Vec<u8>,
            }
            pub struct SetChangesTrieConfig {
                changes_trie_config:
                    Option<__runtime_types::sp_core::changes_trie::ChangesTrieConfiguration>,
            }
            pub struct SetStorage {
                items: Vec<(Vec<u8>, Vec<u8>)>,
            }
            pub struct KillStorage {
                keys: Vec<Vec<u8>>,
            }
            pub struct KillPrefix {
                prefix: Vec<u8>,
                _subkeys: u32,
            }
            pub struct RemarkWithEvent {
                remark: Vec<u8>,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct ExtrinsicSuccess(__runtime_types::frame_support::weights::DispatchInfo);
            pub struct ExtrinsicFailed(
                __runtime_types::sp_runtime::DispatchError,
                __runtime_types::frame_support::weights::DispatchInfo,
            );
            pub struct CodeUpdated();
            pub struct NewAccount(__runtime_types::sp_core::crypto::AccountId32);
            pub struct KilledAccount(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Remarked(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::primitive_types::H256,
            );
        }
    }
    pub mod utility {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct Batch {
                calls: Vec<__runtime_types::node_runtime::Call>,
            }
            pub struct AsDerivative {
                index: u16,
                call: __runtime_types::node_runtime::Call,
            }
            pub struct BatchAll {
                calls: Vec<__runtime_types::node_runtime::Call>,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct BatchInterrupted(u32, __runtime_types::sp_runtime::DispatchError);
            pub struct BatchCompleted();
        }
    }
    pub mod babe {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct ReportEquivocation {
                equivocation_proof: __runtime_types::sp_consensus_slots::EquivocationProof<
                    __runtime_types::sp_runtime::generic::header::Header,
                    __runtime_types::sp_consensus_babe::app::Public,
                >,
                key_owner_proof: __runtime_types::sp_session::MembershipProof,
            }
            pub struct ReportEquivocationUnsigned {
                equivocation_proof: __runtime_types::sp_consensus_slots::EquivocationProof<
                    __runtime_types::sp_runtime::generic::header::Header,
                    __runtime_types::sp_consensus_babe::app::Public,
                >,
                key_owner_proof: __runtime_types::sp_session::MembershipProof,
            }
            pub struct PlanConfigChange {
                config: __runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
        }
    }
    pub mod timestamp {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct Set {
                now: u64,
            }
        }
    }
    pub mod authorship {
        use super::__runtime_types;
    }
    pub mod indices {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct Claim {
                index: u32,
            }
            pub struct Transfer {
                new: __runtime_types::sp_core::crypto::AccountId32,
                index: u32,
            }
            pub struct Free {
                index: u32,
            }
            pub struct ForceTransfer {
                new: __runtime_types::sp_core::crypto::AccountId32,
                index: u32,
                freeze: bool,
            }
            pub struct Freeze {
                index: u32,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct IndexAssigned(__runtime_types::sp_core::crypto::AccountId32, u32);
            pub struct IndexFreed(u32);
            pub struct IndexFrozen(u32, __runtime_types::sp_core::crypto::AccountId32);
        }
    }
    pub mod balances {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct Transfer {
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                value: u128,
            }
            pub struct SetBalance {
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                new_free: u128,
                new_reserved: u128,
            }
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
            pub struct TransferKeepAlive {
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                value: u128,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct Endowed(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct DustLost(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct Transfer(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
            );
            pub struct BalanceSet(__runtime_types::sp_core::crypto::AccountId32, u128, u128);
            pub struct Deposit(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct Reserved(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct Unreserved(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct ReserveRepatriated(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
                __runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            );
        }
    }
    pub mod transaction_payment {
        use super::__runtime_types;
    }
    pub mod election_provider_multi_phase {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct SubmitUnsigned {
                solution: __runtime_types::pallet_election_provider_multi_phase::RawSolution<()>,
                witness:
                    __runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct SolutionStored(
                __runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
            );
            pub struct ElectionFinalized(
                Option<__runtime_types::pallet_election_provider_multi_phase::ElectionCompute>,
            );
            pub struct Rewarded(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Slashed(__runtime_types::sp_core::crypto::AccountId32);
            pub struct SignedPhaseStarted(u32);
            pub struct UnsignedPhaseStarted(u32);
        }
    }
    pub mod staking {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct EraPayout(u32, u128, u128);
            pub struct Reward(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct Slash(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct OldSlashingReportDiscarded(u32);
            pub struct StakingElection();
            pub struct Bonded(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct Unbonded(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct Withdrawn(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct Kicked(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
        }
    }
    pub mod session {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct NewSession(u32);
        }
    }
    pub mod democracy {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct Proposed(u32, u128);
            pub struct Tabled(
                u32,
                u128,
                Vec<__runtime_types::sp_core::crypto::AccountId32>,
            );
            pub struct ExternalTabled();
            pub struct Started(
                u32,
                __runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
            );
            pub struct Passed(u32);
            pub struct NotPassed(u32);
            pub struct Cancelled(u32);
            pub struct Executed(u32, bool);
            pub struct Delegated(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct Undelegated(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Vetoed(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::primitive_types::H256,
                u32,
            );
            pub struct PreimageNoted(
                __runtime_types::primitive_types::H256,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
            );
            pub struct PreimageUsed(
                __runtime_types::primitive_types::H256,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
            );
            pub struct PreimageInvalid(__runtime_types::primitive_types::H256, u32);
            pub struct PreimageMissing(__runtime_types::primitive_types::H256, u32);
            pub struct PreimageReaped(
                __runtime_types::primitive_types::H256,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct Unlocked(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Blacklisted(__runtime_types::primitive_types::H256);
        }
    }
    pub mod council {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct Proposed(
                __runtime_types::sp_core::crypto::AccountId32,
                u32,
                __runtime_types::primitive_types::H256,
                u32,
            );
            pub struct Voted(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::primitive_types::H256,
                bool,
                u32,
                u32,
            );
            pub struct Approved(__runtime_types::primitive_types::H256);
            pub struct Disapproved(__runtime_types::primitive_types::H256);
            pub struct Executed(
                __runtime_types::primitive_types::H256,
                Result<(), __runtime_types::sp_runtime::DispatchError>,
            );
            pub struct MemberExecuted(
                __runtime_types::primitive_types::H256,
                Result<(), __runtime_types::sp_runtime::DispatchError>,
            );
            pub struct Closed(__runtime_types::primitive_types::H256, u32, u32);
        }
    }
    pub mod technical_committee {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct Proposed(
                __runtime_types::sp_core::crypto::AccountId32,
                u32,
                __runtime_types::primitive_types::H256,
                u32,
            );
            pub struct Voted(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::primitive_types::H256,
                bool,
                u32,
                u32,
            );
            pub struct Approved(__runtime_types::primitive_types::H256);
            pub struct Disapproved(__runtime_types::primitive_types::H256);
            pub struct Executed(
                __runtime_types::primitive_types::H256,
                Result<(), __runtime_types::sp_runtime::DispatchError>,
            );
            pub struct MemberExecuted(
                __runtime_types::primitive_types::H256,
                Result<(), __runtime_types::sp_runtime::DispatchError>,
            );
            pub struct Closed(__runtime_types::primitive_types::H256, u32, u32);
        }
    }
    pub mod elections {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct NewTerm(Vec<(__runtime_types::sp_core::crypto::AccountId32, u128)>);
            pub struct EmptyTerm();
            pub struct ElectionError();
            pub struct MemberKicked(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Renounced(__runtime_types::sp_core::crypto::AccountId32);
            pub struct CandidateSlashed(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct SeatHolderSlashed(__runtime_types::sp_core::crypto::AccountId32, u128);
        }
    }
    pub mod technical_membership {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct MemberAdded();
            pub struct MemberRemoved();
            pub struct MembersSwapped();
            pub struct MembersReset();
            pub struct KeyChanged();
            pub struct Dummy(
                core::marker::PhantomData<(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::node_runtime::Event,
                )>,
            );
        }
    }
    pub mod grandpa {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct NewAuthorities(
                Vec<(__runtime_types::sp_finality_grandpa::app::Public, u64)>,
            );
            pub struct Paused();
            pub struct Resumed();
        }
    }
    pub mod treasury {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct Proposed(u32);
            pub struct Spending(u128);
            pub struct Awarded(u32, u128, __runtime_types::sp_core::crypto::AccountId32);
            pub struct Rejected(u32, u128);
            pub struct Burnt(u128);
            pub struct Rollover(u128);
            pub struct Deposit(u128);
        }
    }
    pub mod contracts {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct UpdateSchedule {
                schedule: __runtime_types::pallet_contracts::schedule::Schedule<
                    __runtime_types::node_runtime::Runtime,
                >,
            }
            pub struct Call {
                dest: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                value: u128,
                gas_limit: u64,
                data: Vec<u8>,
            }
            pub struct InstantiateWithCode {
                endowment: u128,
                gas_limit: u64,
                code: Vec<u8>,
                data: Vec<u8>,
                salt: Vec<u8>,
            }
            pub struct Instantiate {
                endowment: u128,
                gas_limit: u64,
                code_hash: __runtime_types::primitive_types::H256,
                data: Vec<u8>,
                salt: Vec<u8>,
            }
            pub struct ClaimSurcharge {
                dest: __runtime_types::sp_core::crypto::AccountId32,
                aux_sender: Option<__runtime_types::sp_core::crypto::AccountId32>,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct Instantiated(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct Evicted(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Terminated(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct Restored(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::primitive_types::H256,
                u128,
            );
            pub struct CodeStored(__runtime_types::primitive_types::H256);
            pub struct ScheduleUpdated(u32);
            pub struct ContractEmitted(__runtime_types::sp_core::crypto::AccountId32, Vec<u8>);
            pub struct CodeRemoved(__runtime_types::primitive_types::H256);
        }
    }
    pub mod sudo {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct Sudo {
                call: __runtime_types::node_runtime::Call,
            }
            pub struct SudoUncheckedWeight {
                call: __runtime_types::node_runtime::Call,
                _weight: u64,
            }
            pub struct SetKey {
                new: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            pub struct SudoAs {
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                call: __runtime_types::node_runtime::Call,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct Sudid(Result<(), __runtime_types::sp_runtime::DispatchError>);
            pub struct KeyChanged(__runtime_types::sp_core::crypto::AccountId32);
            pub struct SudoAsDone(Result<(), __runtime_types::sp_runtime::DispatchError>);
        }
    }
    pub mod im_online {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct HeartbeatReceived(
                __runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            );
            pub struct AllGood();
            pub struct SomeOffline(
                Vec<(
                    __runtime_types::sp_core::crypto::AccountId32,
                    __runtime_types::pallet_staking::Exposure<
                        __runtime_types::sp_core::crypto::AccountId32,
                        u128,
                    >,
                )>,
            );
        }
    }
    pub mod authority_discovery {
        use super::__runtime_types;
    }
    pub mod offences {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct Offence([u8; 16usize], Vec<u8>, bool);
        }
    }
    pub mod historical {
        use super::__runtime_types;
    }
    pub mod randomness_collective_flip {
        use super::__runtime_types;
    }
    pub mod identity {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct IdentitySet(__runtime_types::sp_core::crypto::AccountId32);
            pub struct IdentityCleared(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct IdentityKilled(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct JudgementRequested(__runtime_types::sp_core::crypto::AccountId32, u32);
            pub struct JudgementUnrequested(__runtime_types::sp_core::crypto::AccountId32, u32);
            pub struct JudgementGiven(__runtime_types::sp_core::crypto::AccountId32, u32);
            pub struct RegistrarAdded(u32);
            pub struct SubIdentityAdded(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
            );
            pub struct SubIdentityRemoved(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
            );
            pub struct SubIdentityRevoked(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
            );
        }
    }
    pub mod society {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct Founded(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Bid(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct Vouch(
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct AutoUnbid(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Unbid(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Unvouch(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Inducted(
                __runtime_types::sp_core::crypto::AccountId32,
                Vec<__runtime_types::sp_core::crypto::AccountId32>,
            );
            pub struct SuspendedMemberJudgement(
                __runtime_types::sp_core::crypto::AccountId32,
                bool,
            );
            pub struct CandidateSuspended(__runtime_types::sp_core::crypto::AccountId32);
            pub struct MemberSuspended(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Challenged(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Vote(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                bool,
            );
            pub struct DefenderVote(__runtime_types::sp_core::crypto::AccountId32, bool);
            pub struct NewMaxMembers(u32);
            pub struct Unfounded(__runtime_types::sp_core::crypto::AccountId32);
            pub struct Deposit(u128);
        }
    }
    pub mod recovery {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct RecoveryCreated(__runtime_types::sp_core::crypto::AccountId32);
            pub struct RecoveryInitiated(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct RecoveryVouched(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct RecoveryClosed(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct AccountRecovered(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct RecoveryRemoved(__runtime_types::sp_core::crypto::AccountId32);
        }
    }
    pub mod vesting {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct Vest {}
            pub struct VestOther {
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            pub struct VestedTransfer {
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                schedule: __runtime_types::pallet_vesting::VestingInfo<u128, u32>,
            }
            pub struct ForceVestedTransfer {
                source: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                schedule: __runtime_types::pallet_vesting::VestingInfo<u128, u32>,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct VestingUpdated(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct VestingCompleted(__runtime_types::sp_core::crypto::AccountId32);
        }
    }
    pub mod scheduler {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct Scheduled(u32, u32);
            pub struct Canceled(u32, u32);
            pub struct Dispatched(
                (u32, u32),
                Option<Vec<u8>>,
                Result<(), __runtime_types::sp_runtime::DispatchError>,
            );
        }
    }
    pub mod proxy {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct Proxy {
                real: __runtime_types::sp_core::crypto::AccountId32,
                force_proxy_type: Option<__runtime_types::node_runtime::ProxyType>,
                call: __runtime_types::node_runtime::Call,
            }
            pub struct AddProxy {
                delegate: __runtime_types::sp_core::crypto::AccountId32,
                proxy_type: __runtime_types::node_runtime::ProxyType,
                delay: u32,
            }
            pub struct RemoveProxy {
                delegate: __runtime_types::sp_core::crypto::AccountId32,
                proxy_type: __runtime_types::node_runtime::ProxyType,
                delay: u32,
            }
            pub struct RemoveProxies {}
            pub struct Anonymous {
                proxy_type: __runtime_types::node_runtime::ProxyType,
                delay: u32,
                index: u16,
            }
            pub struct KillAnonymous {
                spawner: __runtime_types::sp_core::crypto::AccountId32,
                proxy_type: __runtime_types::node_runtime::ProxyType,
                index: u16,
                height: u32,
                ext_index: u32,
            }
            pub struct Announce {
                real: __runtime_types::sp_core::crypto::AccountId32,
                call_hash: __runtime_types::primitive_types::H256,
            }
            pub struct RemoveAnnouncement {
                real: __runtime_types::sp_core::crypto::AccountId32,
                call_hash: __runtime_types::primitive_types::H256,
            }
            pub struct RejectAnnouncement {
                delegate: __runtime_types::sp_core::crypto::AccountId32,
                call_hash: __runtime_types::primitive_types::H256,
            }
            pub struct ProxyAnnounced {
                delegate: __runtime_types::sp_core::crypto::AccountId32,
                real: __runtime_types::sp_core::crypto::AccountId32,
                force_proxy_type: Option<__runtime_types::node_runtime::ProxyType>,
                call: __runtime_types::node_runtime::Call,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct ProxyExecuted(Result<(), __runtime_types::sp_runtime::DispatchError>);
            pub struct AnonymousCreated(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::node_runtime::ProxyType,
                u16,
            );
            pub struct Announced(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::primitive_types::H256,
            );
        }
    }
    pub mod multisig {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct NewMultisig(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                [u8; 32usize],
            );
            pub struct MultisigApproval(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::pallet_multisig::Timepoint<u32>,
                __runtime_types::sp_core::crypto::AccountId32,
                [u8; 32usize],
            );
            pub struct MultisigExecuted(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::pallet_multisig::Timepoint<u32>,
                __runtime_types::sp_core::crypto::AccountId32,
                [u8; 32usize],
                Result<(), __runtime_types::sp_runtime::DispatchError>,
            );
            pub struct MultisigCancelled(
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::pallet_multisig::Timepoint<u32>,
                __runtime_types::sp_core::crypto::AccountId32,
                [u8; 32usize],
            );
        }
    }
    pub mod bounties {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct BountyProposed(u32);
            pub struct BountyRejected(u32, u128);
            pub struct BountyBecameActive(u32);
            pub struct BountyAwarded(u32, __runtime_types::sp_core::crypto::AccountId32);
            pub struct BountyClaimed(u32, u128, __runtime_types::sp_core::crypto::AccountId32);
            pub struct BountyCanceled(u32);
            pub struct BountyExtended(u32);
        }
    }
    pub mod tips {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct NewTip(__runtime_types::primitive_types::H256);
            pub struct TipClosing(__runtime_types::primitive_types::H256);
            pub struct TipClosed(
                __runtime_types::primitive_types::H256,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
            );
            pub struct TipRetracted(__runtime_types::primitive_types::H256);
            pub struct TipSlashed(
                __runtime_types::primitive_types::H256,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
            );
        }
    }
    pub mod assets {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct Create {
                id: u32,
                admin: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                min_balance: u64,
            }
            pub struct ForceCreate {
                id: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                is_sufficient: bool,
                min_balance: u64,
            }
            pub struct Destroy {
                id: u32,
                witness: __runtime_types::pallet_assets::types::DestroyWitness,
            }
            pub struct Mint {
                id: u32,
                beneficiary: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            pub struct Burn {
                id: u32,
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            pub struct Transfer {
                id: u32,
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            pub struct TransferKeepAlive {
                id: u32,
                target: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
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
            pub struct Freeze {
                id: u32,
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            pub struct Thaw {
                id: u32,
                who: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
            pub struct FreezeAsset {
                id: u32,
            }
            pub struct ThawAsset {
                id: u32,
            }
            pub struct TransferOwnership {
                id: u32,
                owner: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
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
            pub struct SetMetadata {
                id: u32,
                name: Vec<u8>,
                symbol: Vec<u8>,
                decimals: u8,
            }
            pub struct ClearMetadata {
                id: u32,
            }
            pub struct ForceSetMetadata {
                id: u32,
                name: Vec<u8>,
                symbol: Vec<u8>,
                decimals: u8,
                is_frozen: bool,
            }
            pub struct ForceClearMetadata {
                id: u32,
            }
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
            pub struct ApproveTransfer {
                id: u32,
                delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
                amount: u64,
            }
            pub struct CancelApproval {
                id: u32,
                delegate: __runtime_types::sp_runtime::multiaddress::MultiAddress<
                    __runtime_types::sp_core::crypto::AccountId32,
                    u32,
                >,
            }
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
        pub mod events {
            use super::__runtime_types;
            pub struct Created(
                u32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct Issued(u32, __runtime_types::sp_core::crypto::AccountId32, u64);
            pub struct Transferred(
                u32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                u64,
            );
            pub struct Burned(u32, __runtime_types::sp_core::crypto::AccountId32, u64);
            pub struct TeamChanged(
                u32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct OwnerChanged(u32, __runtime_types::sp_core::crypto::AccountId32);
            pub struct Frozen(u32, __runtime_types::sp_core::crypto::AccountId32);
            pub struct Thawed(u32, __runtime_types::sp_core::crypto::AccountId32);
            pub struct AssetFrozen(u32);
            pub struct AssetThawed(u32);
            pub struct Destroyed(u32);
            pub struct ForceCreated(u32, __runtime_types::sp_core::crypto::AccountId32);
            pub struct MetadataSet(u32, Vec<u8>, Vec<u8>, u8, bool);
            pub struct MetadataCleared(u32);
            pub struct ApprovedTransfer(
                u32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                u64,
            );
            pub struct ApprovalCancelled(
                u32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
            );
            pub struct TransferredApproved(
                u32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                __runtime_types::sp_core::crypto::AccountId32,
                u64,
            );
            pub struct AssetStatusChanged(u32);
        }
    }
    pub mod mmr {
        use super::__runtime_types;
    }
    pub mod lottery {
        use super::__runtime_types;
        pub mod events {
            use super::__runtime_types;
            pub struct LotteryStarted();
            pub struct CallsUpdated();
            pub struct Winner(__runtime_types::sp_core::crypto::AccountId32, u128);
            pub struct TicketBought(__runtime_types::sp_core::crypto::AccountId32, (u8, u8));
        }
    }
    pub mod gilt {
        use super::__runtime_types;
        mod calls {
            use super::__runtime_types;
            pub struct PlaceBid {
                amount: u128,
                duration: u32,
            }
            pub struct RetractBid {
                amount: u128,
                duration: u32,
            }
            pub struct SetTarget {
                target: __runtime_types::sp_arithmetic::per_things::Perquintill,
            }
            pub struct Thaw {
                index: u32,
            }
        }
        pub mod events {
            use super::__runtime_types;
            pub struct BidPlaced(__runtime_types::sp_core::crypto::AccountId32, u128, u32);
            pub struct BidRetracted(__runtime_types::sp_core::crypto::AccountId32, u128, u32);
            pub struct GiltIssued(
                u32,
                u32,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
            );
            pub struct GiltThawed(
                u32,
                __runtime_types::sp_core::crypto::AccountId32,
                u128,
                u128,
            );
        }
    }
}
