#[allow(dead_code, unused_imports)]
pub mod node_runtime {
    mod types {
        pub struct Perbill(pub u32);
        pub struct ChangesTrieConfiguration {
            pub digest_interval: u32,
            pub digest_levels: u32,
        }
        pub struct DispatchInfo {
            pub weight: u64,
            pub class: DispatchClass,
            pub pays_fee: Pays,
        }
        pub enum DispatchClass {
            Normal,
            Operational,
            Mandatory,
        }
        pub enum Pays {
            Yes,
            No,
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
        }
        pub struct AccountId32(pub [u8; 32usize]);
        pub struct H256(pub [u8; 32usize]);
        pub struct EquivocationProof<_0, _1> {
            pub offender: _1,
            pub slot: Slot,
            pub first_header: _0,
            pub second_header: _0,
        }
        pub struct Header {}
        pub struct Public(pub Public);
        pub struct Public(pub [u8; 32usize]);
        pub struct Slot(pub u64);
        pub struct MembershipProof {
            pub session: u32,
            pub trie_nodes: Vec<Vec<u8>>,
            pub validator_count: u32,
        }
        pub enum NextConfigDescriptor {
            V1 {
                c: (u64, u64),
                allowed_slots: AllowedSlots,
            },
        }
        pub enum AllowedSlots {
            PrimarySlots,
            PrimaryAndSecondaryPlainSlots,
            PrimaryAndSecondaryVRFSlots,
        }
        pub enum MultiAddress<_0, _1> {
            Id(_0),
            Index(_1),
            Raw(Vec<u8>),
            Address32([u8; 32usize]),
            Address20([u8; 20usize]),
        }
        pub enum BalanceStatus {
            Free,
            Reserved,
        }
        pub struct RawSolution<_0> {
            pub compact: _0,
            pub score: [u128; 3usize],
            pub round: u32,
        }
        pub struct CompactAssignments {}
        pub struct SolutionOrSnapshotSize {
            pub voters: u32,
            pub targets: u32,
        }
        pub enum ElectionCompute {
            OnChain,
            Signed,
            Unsigned,
        }
        pub struct Schedule<_0> {
            pub version: u32,
            pub enable_println: bool,
            pub limits: Limits,
            pub instruction_weights: InstructionWeights<_0>,
            pub host_fn_weights: HostFnWeights<_0>,
        }
        pub struct Runtime {}
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
        pub enum ProxyType {
            Any,
            NonTransfer,
            Governance,
            Staking,
        }
        pub enum Call {
            System(Call<Runtime>),
            Utility(Call<Runtime>),
            Babe(Call<Runtime>),
            Timestamp(Call<Runtime>),
            Authorship(Call<Runtime>),
            Indices(Call<Runtime>),
            Balances(Call<Runtime, ()>),
            ElectionProviderMultiPhase(Call<Runtime>),
            Staking(Call<Runtime>),
            Session(Call<Runtime>),
            Democracy(Call<Runtime>),
            Council(Call<Runtime, Instance1>),
            TechnicalCommittee(Call<Runtime, Instance2>),
            Elections(Call<Runtime>),
            TechnicalMembership(Call<Runtime, Instance1>),
            Grandpa(Call<Runtime>),
            Treasury(Call<Runtime, DefaultInstance>),
            Contracts(Call<Runtime>),
            Sudo(Call<Runtime>),
            ImOnline(Call<Runtime>),
            AuthorityDiscovery(Call<Runtime>),
            Offences(Call<Runtime>),
            RandomnessCollectiveFlip(Call<Runtime>),
            Identity(Call<Runtime>),
            Society(Call<Runtime, DefaultInstance>),
            Recovery(Call<Runtime>),
            Vesting(Call<Runtime>),
            Scheduler(Call<Runtime>),
            Proxy(Call<Runtime>),
            Multisig(Call<Runtime>),
            Bounties(Call<Runtime>),
            Tips(Call<Runtime>),
            Assets(Call<Runtime>),
            Lottery(Call<Runtime>),
            Gilt(Call<Runtime>),
        }
        pub enum Call<_0> {
            __Ignore(core::marker::PhantomData<(_0,)>, Never),
            fill_block(Perbill),
            remark(Vec<u8>),
            set_heap_pages(u64),
            set_code(Vec<u8>),
            set_code_without_checks(Vec<u8>),
            set_changes_trie_config(Option<ChangesTrieConfiguration>),
            set_storage(Vec<(Vec<u8>, Vec<u8>)>),
            kill_storage(Vec<Vec<u8>>),
            kill_prefix(Vec<u8>, u32),
            remark_with_event(Vec<u8>),
        }
        pub enum Never {}
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            batch(Vec<Call>),
            as_derivative(u16, Call),
            batch_all(Vec<Call>),
        }
        pub enum Call<_0> {
            __Ignore(core::marker::PhantomData<(_0,)>, Never),
            report_equivocation(EquivocationProof<Header, Public>, MembershipProof),
            report_equivocation_unsigned(EquivocationProof<Header, Public>, MembershipProof),
            plan_config_change(NextConfigDescriptor),
        }
        pub enum Call<_0> {
            __Ignore(core::marker::PhantomData<(_0,)>, Never),
            set(u64),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            set_uncles(Vec<Header>),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            claim(u32),
            transfer(AccountId32, u32),
            free(u32),
            force_transfer(AccountId32, u32, bool),
            freeze(u32),
        }
        pub enum Call<_0, _1> {
            __Ignore(core::marker::PhantomData<(_0, _1)>, Never),
            transfer(MultiAddress<AccountId32, u32>, u128),
            set_balance(MultiAddress<AccountId32, u32>, u128, u128),
            force_transfer(
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
                u128,
            ),
            transfer_keep_alive(MultiAddress<AccountId32, u32>, u128),
        }
        pub enum Call<_0> {
            __Ignore(core::marker::PhantomData<(_0,)>, Never),
            submit_unsigned(RawSolution<CompactAssignments>, SolutionOrSnapshotSize),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            bond(
                MultiAddress<AccountId32, u32>,
                u128,
                RewardDestination<AccountId32>,
            ),
            bond_extra(u128),
            unbond(u128),
            withdraw_unbonded(u32),
            validate(ValidatorPrefs),
            nominate(Vec<MultiAddress<AccountId32, u32>>),
            chill,
            set_payee(RewardDestination<AccountId32>),
            set_controller(MultiAddress<AccountId32, u32>),
            set_validator_count(u32),
            increase_validator_count(u32),
            scale_validator_count(Percent),
            force_no_eras,
            force_new_era,
            set_invulnerables(Vec<AccountId32>),
            force_unstake(AccountId32, u32),
            force_new_era_always,
            cancel_deferred_slash(u32, Vec<u32>),
            payout_stakers(AccountId32, u32),
            rebond(u128),
            set_history_depth(u32, u32),
            reap_stash(AccountId32, u32),
            submit_election_solution(
                Vec<u16>,
                CompactAssignments,
                [u128; 3usize],
                u32,
                ElectionSize,
            ),
            submit_election_solution_unsigned(
                Vec<u16>,
                CompactAssignments,
                [u128; 3usize],
                u32,
                ElectionSize,
            ),
            kick(Vec<MultiAddress<AccountId32, u32>>),
        }
        pub enum RewardDestination<_0> {
            Staked,
            Stash,
            Controller,
            Account(_0),
            None,
        }
        pub struct ValidatorPrefs {
            pub commission: Perbill,
            pub blocked: bool,
        }
        pub struct Percent(pub u8);
        pub struct ElectionSize {
            pub validators: u16,
            pub nominators: u32,
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            set_keys(SessionKeys, Vec<u8>),
            purge_keys,
        }
        pub struct SessionKeys {
            pub grandpa: Public,
            pub babe: Public,
            pub im_online: Public,
            pub authority_discovery: Public,
        }
        pub struct Public(pub Public);
        pub struct Public(pub [u8; 32usize]);
        pub struct Public(pub Public);
        pub struct Public(pub Public);
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            propose(H256, u128),
            second(u32, u32),
            vote(u32, AccountVote<u128>),
            emergency_cancel(u32),
            external_propose(H256),
            external_propose_majority(H256),
            external_propose_default(H256),
            fast_track(H256, u32, u32),
            veto_external(H256),
            cancel_referendum(u32),
            cancel_queued(u32),
            delegate(AccountId32, Conviction, u128),
            undelegate,
            clear_public_proposals,
            note_preimage(Vec<u8>),
            note_preimage_operational(Vec<u8>),
            note_imminent_preimage(Vec<u8>),
            note_imminent_preimage_operational(Vec<u8>),
            reap_preimage(H256, u32),
            unlock(AccountId32),
            remove_vote(u32),
            remove_other_vote(AccountId32, u32),
            enact_proposal(H256, u32),
            blacklist(H256, Option<u32>),
            cancel_proposal(u32),
        }
        pub enum AccountVote<_0> {
            Standard { vote: Vote, balance: _0 },
            Split { aye: _0, nay: _0 },
        }
        pub struct Vote {
            pub aye: bool,
            pub conviction: Conviction,
        }
        pub enum Conviction {
            None,
            Locked1x,
            Locked2x,
            Locked3x,
            Locked4x,
            Locked5x,
            Locked6x,
        }
        pub enum Call<_0, _1> {
            __PhantomItem(core::marker::PhantomData<(_0, _1)>, Never),
            set_members(Vec<AccountId32>, Option<AccountId32>, u32),
            execute(Call, u32),
            propose(u32, Call, u32),
            vote(H256, u32, bool),
            close(H256, u32, u64, u32),
            disapprove_proposal(H256),
        }
        pub struct Instance1 {}
        pub enum Call<_0, _1> {
            __PhantomItem(core::marker::PhantomData<(_0, _1)>, Never),
            set_members(Vec<AccountId32>, Option<AccountId32>, u32),
            execute(Call, u32),
            propose(u32, Call, u32),
            vote(H256, u32, bool),
            close(H256, u32, u64, u32),
            disapprove_proposal(H256),
        }
        pub struct Instance2 {}
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            vote(Vec<AccountId32>, u128),
            remove_voter,
            submit_candidacy(u32),
            renounce_candidacy(Renouncing),
            remove_member(MultiAddress<AccountId32, u32>, bool),
            clean_defunct_voters(u32, u32),
        }
        pub enum Renouncing {
            Member,
            RunnerUp,
            Candidate(u32),
        }
        pub enum Call<_0, _1> {
            __PhantomItem(core::marker::PhantomData<(_0, _1)>, Never),
            add_member(AccountId32),
            remove_member(AccountId32),
            swap_member(AccountId32, AccountId32),
            reset_members(Vec<AccountId32>),
            change_key(AccountId32),
            set_prime(AccountId32),
            clear_prime,
        }
        pub struct Instance1 {}
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            report_equivocation(EquivocationProof<H256, u32>, MembershipProof),
            report_equivocation_unsigned(EquivocationProof<H256, u32>, MembershipProof),
            note_stalled(u32, u32),
        }
        pub struct EquivocationProof<_0, _1> {
            pub set_id: u64,
            pub equivocation: Equivocation<_0, _1>,
        }
        pub enum Equivocation<_0, _1> {
            Prevote(Equivocation<Public, Prevote<_0, _1>, Signature>),
            Precommit(Equivocation<Public, Precommit<_0, _1>, Signature>),
        }
        pub struct Equivocation<_0, _1, _2> {
            pub round_number: u64,
            pub identity: _0,
            pub first: (_1, _2),
            pub second: (_1, _2),
        }
        pub struct Prevote<_0, _1> {
            pub target_hash: _0,
            pub target_number: _1,
        }
        pub struct Signature(pub Signature);
        pub struct Signature(pub [u8; 64usize]);
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
        pub enum Call<_0, _1> {
            __PhantomItem(core::marker::PhantomData<(_0, _1)>, Never),
            propose_spend(u128, MultiAddress<AccountId32, u32>),
            reject_proposal(u32),
            approve_proposal(u32),
        }
        pub struct DefaultInstance {}
        pub enum Call<_0> {
            __Ignore(core::marker::PhantomData<(_0,)>, Never),
            update_schedule(Schedule<_0>),
            call(MultiAddress<AccountId32, u32>, u128, u64, Vec<u8>),
            instantiate_with_code(u128, u64, Vec<u8>, Vec<u8>, Vec<u8>),
            instantiate(u128, u64, H256, Vec<u8>, Vec<u8>),
            claim_surcharge(AccountId32, Option<AccountId32>),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            sudo(Call),
            sudo_unchecked_weight(Call, u64),
            set_key(MultiAddress<AccountId32, u32>),
            sudo_as(MultiAddress<AccountId32, u32>, Call),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            heartbeat(Heartbeat<u32>, Signature),
        }
        pub struct Heartbeat<_0> {
            pub block_number: _0,
            pub network_state: OpaqueNetworkState,
            pub session_index: _0,
            pub authority_index: _0,
            pub validators_len: _0,
        }
        pub struct OpaqueNetworkState {
            pub peer_id: OpaquePeerId,
            pub external_addresses: Vec<OpaqueMultiaddr>,
        }
        pub struct OpaquePeerId(pub Vec<u8>);
        pub struct OpaqueMultiaddr(pub Vec<u8>);
        pub struct Signature(pub Signature);
        pub struct Signature(pub [u8; 64usize]);
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            add_registrar(AccountId32),
            set_identity(IdentityInfo),
            set_subs(Vec<(AccountId32, Data)>),
            clear_identity,
            request_judgement(u32, u128),
            cancel_request(u32),
            set_fee(u32, u128),
            set_account_id(u32, AccountId32),
            set_fields(u32, IdentityFields),
            provide_judgement(u32, MultiAddress<AccountId32, u32>, Judgement<u128>),
            kill_identity(MultiAddress<AccountId32, u32>),
            add_sub(MultiAddress<AccountId32, u32>, Data),
            rename_sub(MultiAddress<AccountId32, u32>, Data),
            remove_sub(MultiAddress<AccountId32, u32>),
            quit_sub,
        }
        pub struct IdentityInfo {
            pub additional: Vec<(Data, Data)>,
            pub display: Data,
            pub legal: Data,
            pub web: Data,
            pub riot: Data,
            pub email: Data,
            pub pgp_fingerprint: Option<[u8; 20usize]>,
            pub image: Data,
            pub twitter: Data,
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
        pub enum Judgement<_0> {
            Unknown,
            FeePaid(_0),
            Reasonable,
            KnownGood,
            OutOfDate,
            LowQuality,
            Erroneous,
        }
        pub enum Call<_0, _1> {
            __PhantomItem(core::marker::PhantomData<(_0, _1)>, Never),
            bid(u128),
            unbid(u32),
            vouch(AccountId32, u128, u128),
            unvouch(u32),
            vote(MultiAddress<AccountId32, u32>, bool),
            defender_vote(bool),
            payout,
            found(AccountId32, u32, Vec<u8>),
            unfound,
            judge_suspended_member(AccountId32, bool),
            judge_suspended_candidate(AccountId32, Judgement),
            set_max_members(u32),
        }
        pub struct DefaultInstance {}
        pub enum Judgement {
            Rebid,
            Reject,
            Approve,
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            as_recovered(AccountId32, Call),
            set_recovered(AccountId32, AccountId32),
            create_recovery(Vec<AccountId32>, u16, u32),
            initiate_recovery(AccountId32),
            vouch_recovery(AccountId32, AccountId32),
            claim_recovery(AccountId32),
            close_recovery(AccountId32),
            remove_recovery,
            cancel_recovered(AccountId32),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            vest,
            vest_other(MultiAddress<AccountId32, u32>),
            vested_transfer(MultiAddress<AccountId32, u32>, VestingInfo<u128, u32>),
            force_vested_transfer(
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
                VestingInfo<u128, u32>,
            ),
        }
        pub struct VestingInfo<_0, _1> {
            pub locked: _0,
            pub per_block: _0,
            pub starting_block: _1,
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            schedule(u32, Option<(u32, u32)>, u8, Call),
            cancel(u32, u32),
            schedule_named(Vec<u8>, u32, Option<(u32, u32)>, u8, Call),
            cancel_named(Vec<u8>),
            schedule_after(u32, Option<(u32, u32)>, u8, Call),
            schedule_named_after(Vec<u8>, u32, Option<(u32, u32)>, u8, Call),
        }
        pub enum Call<_0> {
            __Ignore(core::marker::PhantomData<(_0,)>, Never),
            proxy(AccountId32, Option<ProxyType>, Call),
            add_proxy(AccountId32, ProxyType, u32),
            remove_proxy(AccountId32, ProxyType, u32),
            remove_proxies,
            anonymous(ProxyType, u32, u16),
            kill_anonymous(AccountId32, ProxyType, u16, u32, u32),
            announce(AccountId32, H256),
            remove_announcement(AccountId32, H256),
            reject_announcement(AccountId32, H256),
            proxy_announced(AccountId32, AccountId32, Option<ProxyType>, Call),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            as_multi_threshold_1(Vec<AccountId32>, Call),
            as_multi(
                u16,
                Vec<AccountId32>,
                Option<Timepoint<u32>>,
                Vec<u8>,
                bool,
                u64,
            ),
            approve_as_multi(
                u16,
                Vec<AccountId32>,
                Option<Timepoint<u32>>,
                [u8; 32usize],
                u64,
            ),
            cancel_as_multi(u16, Vec<AccountId32>, Timepoint<u32>, [u8; 32usize]),
        }
        pub struct Timepoint<_0> {
            pub height: _0,
            pub index: _0,
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            propose_bounty(u128, Vec<u8>),
            approve_bounty(u32),
            propose_curator(u32, MultiAddress<AccountId32, u32>, u128),
            unassign_curator(u32),
            accept_curator(u32),
            award_bounty(u32, MultiAddress<AccountId32, u32>),
            claim_bounty(u32),
            close_bounty(u32),
            extend_bounty_expiry(u32, Vec<u8>),
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            report_awesome(Vec<u8>, AccountId32),
            retract_tip(H256),
            tip_new(Vec<u8>, AccountId32, u128),
            tip(H256, u128),
            close_tip(H256),
            slash_tip(H256),
        }
        pub enum Call<_0> {
            __Ignore(core::marker::PhantomData<(_0,)>, Never),
            create(u32, MultiAddress<AccountId32, u32>, u64),
            force_create(u32, MultiAddress<AccountId32, u32>, bool, u64),
            destroy(u32, DestroyWitness),
            mint(u32, MultiAddress<AccountId32, u32>, u64),
            burn(u32, MultiAddress<AccountId32, u32>, u64),
            transfer(u32, MultiAddress<AccountId32, u32>, u64),
            transfer_keep_alive(u32, MultiAddress<AccountId32, u32>, u64),
            force_transfer(
                u32,
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
                u64,
            ),
            freeze(u32, MultiAddress<AccountId32, u32>),
            thaw(u32, MultiAddress<AccountId32, u32>),
            freeze_asset(u32),
            thaw_asset(u32),
            transfer_ownership(u32, MultiAddress<AccountId32, u32>),
            set_team(
                u32,
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
            ),
            set_metadata(u32, Vec<u8>, Vec<u8>, u8),
            clear_metadata(u32),
            force_set_metadata(u32, Vec<u8>, Vec<u8>, u8, bool),
            force_clear_metadata(u32),
            force_asset_status(
                u32,
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
                u64,
                bool,
                bool,
            ),
            approve_transfer(u32, MultiAddress<AccountId32, u32>, u64),
            cancel_approval(u32, MultiAddress<AccountId32, u32>),
            force_cancel_approval(
                u32,
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
            ),
            transfer_approved(
                u32,
                MultiAddress<AccountId32, u32>,
                MultiAddress<AccountId32, u32>,
                u64,
            ),
        }
        pub struct DestroyWitness {
            pub accounts: u32,
            pub sufficients: u32,
            pub approvals: u32,
        }
        pub enum Call<_0> {
            __PhantomItem(core::marker::PhantomData<(_0,)>, Never),
            buy_ticket(Call),
            set_calls(Vec<Call>),
            start_lottery(u128, u32, u32, bool),
            stop_repeat,
        }
        pub enum Call<_0> {
            __Ignore(core::marker::PhantomData<(_0,)>, Never),
            place_bid(u128, u32),
            retract_bid(u128, u32),
            set_target(Perquintill),
            thaw(u32),
        }
        pub struct Perquintill(pub u64);
        pub struct CheckSpecVersion<_0>(pub core::marker::PhantomData<_0>);
        pub struct CheckTxVersion<_0>(pub core::marker::PhantomData<_0>);
        pub struct CheckGenesis<_0>(pub core::marker::PhantomData<_0>);
        pub struct CheckMortality<_0>(pub Era, pub core::marker::PhantomData<_0>);
        pub enum Era {
            Immortal,
            Mortal(u64, u64),
        }
        pub struct CheckNonce<_0>(pub u32);
        pub struct CheckWeight<_0>(pub core::marker::PhantomData<_0>);
        pub struct ChargeTransactionPayment<_0>(pub u128);
    }
    pub mod system {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct FillBlock {
                _ratio: Perbill,
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
                changes_trie_config: Option<ChangesTrieConfiguration>,
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
            use super::*;
            pub struct ExtrinsicSuccess(DispatchInfo);
            pub struct ExtrinsicFailed(DispatchError, DispatchInfo);
            pub struct CodeUpdated();
            pub struct NewAccount(AccountId32);
            pub struct KilledAccount(AccountId32);
            pub struct Remarked(AccountId32, H256);
        }
    }
    pub mod utility {
        use super::types::*;
    }
    pub mod babe {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct ReportEquivocation {
                equivocation_proof: EquivocationProof<Header, Public>,
                key_owner_proof: MembershipProof,
            }
            pub struct ReportEquivocationUnsigned {
                equivocation_proof: EquivocationProof<Header, Public>,
                key_owner_proof: MembershipProof,
            }
            pub struct PlanConfigChange {
                config: NextConfigDescriptor,
            }
        }
    }
    pub mod timestamp {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct Set {
                now: u64,
            }
        }
    }
    pub mod authorship {
        use super::types::*;
    }
    pub mod indices {
        use super::types::*;
    }
    pub mod balances {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct Transfer {
                dest: MultiAddress<AccountId32, u32>,
                value: u128,
            }
            pub struct SetBalance {
                who: MultiAddress<AccountId32, u32>,
                new_free: u128,
                new_reserved: u128,
            }
            pub struct ForceTransfer {
                source: MultiAddress<AccountId32, u32>,
                dest: MultiAddress<AccountId32, u32>,
                value: u128,
            }
            pub struct TransferKeepAlive {
                dest: MultiAddress<AccountId32, u32>,
                value: u128,
            }
        }
        pub mod events {
            use super::*;
            pub struct Endowed(AccountId32, u128);
            pub struct DustLost(AccountId32, u128);
            pub struct Transfer(AccountId32, AccountId32, u128);
            pub struct BalanceSet(AccountId32, u128, u128);
            pub struct Deposit(AccountId32, u128);
            pub struct Reserved(AccountId32, u128);
            pub struct Unreserved(AccountId32, u128);
            pub struct ReserveRepatriated(AccountId32, AccountId32, u128, BalanceStatus);
        }
    }
    pub mod transaction_payment {
        use super::types::*;
    }
    pub mod election_provider_multi_phase {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct SubmitUnsigned {
                solution: RawSolution<CompactAssignments>,
                witness: SolutionOrSnapshotSize,
            }
        }
        pub mod events {
            use super::*;
            pub struct SolutionStored(ElectionCompute);
            pub struct ElectionFinalized(Option<ElectionCompute>);
            pub struct Rewarded(AccountId32);
            pub struct Slashed(AccountId32);
            pub struct SignedPhaseStarted(u32);
            pub struct UnsignedPhaseStarted(u32);
        }
    }
    pub mod staking {
        use super::types::*;
    }
    pub mod session {
        use super::types::*;
    }
    pub mod democracy {
        use super::types::*;
    }
    pub mod council {
        use super::types::*;
    }
    pub mod technical_committee {
        use super::types::*;
    }
    pub mod elections {
        use super::types::*;
    }
    pub mod technical_membership {
        use super::types::*;
    }
    pub mod grandpa {
        use super::types::*;
    }
    pub mod treasury {
        use super::types::*;
    }
    pub mod contracts {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct UpdateSchedule {
                schedule: Schedule<Runtime>,
            }
            pub struct Call {
                dest: MultiAddress<AccountId32, u32>,
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
                code_hash: H256,
                data: Vec<u8>,
                salt: Vec<u8>,
            }
            pub struct ClaimSurcharge {
                dest: AccountId32,
                aux_sender: Option<AccountId32>,
            }
        }
        pub mod events {
            use super::*;
            pub struct Instantiated(AccountId32, AccountId32);
            pub struct Evicted(AccountId32);
            pub struct Terminated(AccountId32, AccountId32);
            pub struct Restored(AccountId32, AccountId32, H256, u128);
            pub struct CodeStored(H256);
            pub struct ScheduleUpdated(u32);
            pub struct ContractEmitted(AccountId32, Vec<u8>);
            pub struct CodeRemoved(H256);
        }
    }
    pub mod sudo {
        use super::types::*;
    }
    pub mod im_online {
        use super::types::*;
    }
    pub mod authority_discovery {
        use super::types::*;
    }
    pub mod offences {
        use super::types::*;
    }
    pub mod historical {
        use super::types::*;
    }
    pub mod randomness_collective_flip {
        use super::types::*;
    }
    pub mod identity {
        use super::types::*;
    }
    pub mod society {
        use super::types::*;
    }
    pub mod recovery {
        use super::types::*;
    }
    pub mod vesting {
        use super::types::*;
    }
    pub mod scheduler {
        use super::types::*;
    }
    pub mod proxy {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct Proxy {
                real: AccountId32,
                force_proxy_type: Option<ProxyType>,
                call: Call,
            }
            pub struct AddProxy {
                delegate: AccountId32,
                proxy_type: ProxyType,
                delay: u32,
            }
            pub struct RemoveProxy {
                delegate: AccountId32,
                proxy_type: ProxyType,
                delay: u32,
            }
            pub struct RemoveProxies {}
            pub struct Anonymous {
                proxy_type: ProxyType,
                delay: u32,
                index: u16,
            }
            pub struct KillAnonymous {
                spawner: AccountId32,
                proxy_type: ProxyType,
                index: u16,
                height: u32,
                ext_index: u32,
            }
            pub struct Announce {
                real: AccountId32,
                call_hash: H256,
            }
            pub struct RemoveAnnouncement {
                real: AccountId32,
                call_hash: H256,
            }
            pub struct RejectAnnouncement {
                delegate: AccountId32,
                call_hash: H256,
            }
            pub struct ProxyAnnounced {
                delegate: AccountId32,
                real: AccountId32,
                force_proxy_type: Option<ProxyType>,
                call: Call,
            }
        }
        pub mod events {
            use super::*;
            pub struct ProxyExecuted(Result<(), DispatchError>);
            pub struct AnonymousCreated(AccountId32, AccountId32, ProxyType, u16);
            pub struct Announced(AccountId32, AccountId32, H256);
        }
    }
    pub mod multisig {
        use super::types::*;
    }
    pub mod bounties {
        use super::types::*;
    }
    pub mod tips {
        use super::types::*;
    }
    pub mod assets {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct Create {
                id: u32,
                admin: MultiAddress<AccountId32, u32>,
                min_balance: u64,
            }
            pub struct ForceCreate {
                id: u32,
                owner: MultiAddress<AccountId32, u32>,
                is_sufficient: bool,
                min_balance: u64,
            }
            pub struct Destroy {
                id: u32,
                witness: DestroyWitness,
            }
            pub struct Mint {
                id: u32,
                beneficiary: MultiAddress<AccountId32, u32>,
                amount: u64,
            }
            pub struct Burn {
                id: u32,
                who: MultiAddress<AccountId32, u32>,
                amount: u64,
            }
            pub struct Transfer {
                id: u32,
                target: MultiAddress<AccountId32, u32>,
                amount: u64,
            }
            pub struct TransferKeepAlive {
                id: u32,
                target: MultiAddress<AccountId32, u32>,
                amount: u64,
            }
            pub struct ForceTransfer {
                id: u32,
                source: MultiAddress<AccountId32, u32>,
                dest: MultiAddress<AccountId32, u32>,
                amount: u64,
            }
            pub struct Freeze {
                id: u32,
                who: MultiAddress<AccountId32, u32>,
            }
            pub struct Thaw {
                id: u32,
                who: MultiAddress<AccountId32, u32>,
            }
            pub struct FreezeAsset {
                id: u32,
            }
            pub struct ThawAsset {
                id: u32,
            }
            pub struct TransferOwnership {
                id: u32,
                owner: MultiAddress<AccountId32, u32>,
            }
            pub struct SetTeam {
                id: u32,
                issuer: MultiAddress<AccountId32, u32>,
                admin: MultiAddress<AccountId32, u32>,
                freezer: MultiAddress<AccountId32, u32>,
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
                owner: MultiAddress<AccountId32, u32>,
                issuer: MultiAddress<AccountId32, u32>,
                admin: MultiAddress<AccountId32, u32>,
                freezer: MultiAddress<AccountId32, u32>,
                min_balance: u64,
                is_sufficient: bool,
                is_frozen: bool,
            }
            pub struct ApproveTransfer {
                id: u32,
                delegate: MultiAddress<AccountId32, u32>,
                amount: u64,
            }
            pub struct CancelApproval {
                id: u32,
                delegate: MultiAddress<AccountId32, u32>,
            }
            pub struct ForceCancelApproval {
                id: u32,
                owner: MultiAddress<AccountId32, u32>,
                delegate: MultiAddress<AccountId32, u32>,
            }
            pub struct TransferApproved {
                id: u32,
                owner: MultiAddress<AccountId32, u32>,
                destination: MultiAddress<AccountId32, u32>,
                amount: u64,
            }
        }
        pub mod events {
            use super::*;
            pub struct Created(u32, AccountId32, AccountId32);
            pub struct Issued(u32, AccountId32, u64);
            pub struct Transferred(u32, AccountId32, AccountId32, u64);
            pub struct Burned(u32, AccountId32, u64);
            pub struct TeamChanged(u32, AccountId32, AccountId32, AccountId32);
            pub struct OwnerChanged(u32, AccountId32);
            pub struct Frozen(u32, AccountId32);
            pub struct Thawed(u32, AccountId32);
            pub struct AssetFrozen(u32);
            pub struct AssetThawed(u32);
            pub struct Destroyed(u32);
            pub struct ForceCreated(u32, AccountId32);
            pub struct MetadataSet(u32, Vec<u8>, Vec<u8>, u8, bool);
            pub struct MetadataCleared(u32);
            pub struct ApprovedTransfer(u32, AccountId32, AccountId32, u64);
            pub struct ApprovalCancelled(u32, AccountId32, AccountId32);
            pub struct TransferredApproved(u32, AccountId32, AccountId32, AccountId32, u64);
            pub struct AssetStatusChanged(u32);
        }
    }
    pub mod mmr {
        use super::types::*;
    }
    pub mod lottery {
        use super::types::*;
    }
    pub mod gilt {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct PlaceBid {
                amount: u128,
                duration: u32,
            }
            pub struct RetractBid {
                amount: u128,
                duration: u32,
            }
            pub struct SetTarget {
                target: Perquintill,
            }
            pub struct Thaw {
                index: u32,
            }
        }
        pub mod events {
            use super::*;
            pub struct BidPlaced(AccountId32, u128, u32);
            pub struct BidRetracted(AccountId32, u128, u32);
            pub struct GiltIssued(u32, u32, AccountId32, u128);
            pub struct GiltThawed(u32, AccountId32, u128, u128);
        }
    }
}
