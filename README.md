# chameleon

## Health warning: this is a WIP prototype to prove the concept.

SCALE compatible type generation for substrate runtimes. No dependencies on substrate crates required!

Accepts a SCALE encoded metadata file via a CLI or a macro, and generates a Rust module with all types required for interacting with pallets for the given substrate runtime.

## Current status

- Generated code not compiling yet, need to handle generics
- Only System and Balances module converted to the new metadata in https://github.com/paritytech/substrate/compare/gui-macro-attribute...aj-metadata-vnext

Using the command: `cargo run -p chameleon-cli | rustfmt --edition=2018 --emit=stdout`, generates the following:

```
mod node_runtime {
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
        }
        pub struct AccountId32(pub [u8; 32usize]);
        pub enum MultiAddress {
            Id(AccountId32),
            Index(u32),
            Raw(Vec<u8>),
            Address32([u8; 32usize]),
            Address20([u8; 20usize]),
        }
        pub enum BalanceStatus {
            Free,
            Reserved,
        }
    }
    mod System {
        mod calls {
            use super::super::types::*;
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
            pub struct Suicide {}
        }
        mod events {
            use super::super::types::*;
            pub struct ExtrinsicSuccess(DispatchInfo);
            pub struct ExtrinsicFailed(DispatchError, DispatchInfo);
            pub struct CodeUpdated();
            pub struct NewAccount(AccountId32);
            pub struct KilledAccount(AccountId32);
        }
    }
    mod Utility {}
    mod Babe {}
    mod Timestamp {
        mod calls {
            use super::super::types::*;
            pub struct Set {
                now: u64,
            }
        }
        mod events {
            use super::super::types::*;
        }
    }
    mod Authorship {}
    mod Indices {}
    mod Balances {
        mod calls {
            use super::super::types::*;
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
        mod events {
            use super::super::types::*;
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
    mod TransactionPayment {}
    mod Staking {}
    mod Session {}
    mod Democracy {}
    mod Council {}
    mod TechnicalCommittee {}
    mod Elections {}
    mod TechnicalMembership {}
    mod Grandpa {}
    mod Treasury {}
    mod Contracts {}
    mod Sudo {}
    mod ImOnline {}
    mod AuthorityDiscovery {}
    mod Offences {}
    mod Historical {}
    mod RandomnessCollectiveFlip {}
    mod Identity {}
    mod Society {}
    mod Recovery {}
    mod Vesting {}
    mod Scheduler {}
    mod Proxy {}
    mod Multisig {}
}

```

