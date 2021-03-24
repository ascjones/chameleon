# chameleon

## Health warning: this is a WIP prototype to prove the concept.

SCALE compatible type generation for substrate runtimes. No dependencies on substrate crates required!

Accepts a SCALE encoded metadata file via a CLI or a macro, and generates a Rust module with all types required for interacting with pallets for the given substrate runtime.

## Current status

- Generated code not compiling yet, need to handle generics
- Only System and Balances module converted to the new metadata in https://github.com/paritytech/substrate/compare/gui-macro-attribute...aj-metadata-vnext

Using the command: `cargo run -p chameleon-cli | rustfmt --edition=2018 --emit=stdout`, generates the following:

<details>

```
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
        }
        pub struct AccountId32(pub [u8; 32usize]);
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
    }
    pub mod System {
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
            pub struct Suicide {}
        }
        pub mod events {
            use super::*;
            pub struct ExtrinsicSuccess(DispatchInfo);
            pub struct ExtrinsicFailed(DispatchError, DispatchInfo);
            pub struct CodeUpdated();
            pub struct NewAccount(AccountId32);
            pub struct KilledAccount(AccountId32);
        }
    }
    pub mod Utility {
        use super::types::*;
    }
    pub mod Babe {
        use super::types::*;
    }
    pub mod Timestamp {
        use super::types::*;
        mod calls {
            use super::*;
            pub struct Set {
                now: u64,
            }
        }
    }
    pub mod Authorship {
        use super::types::*;
    }
    pub mod Indices {
        use super::types::*;
    }
    pub mod Balances {
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
    pub mod TransactionPayment {
        use super::types::*;
    }
    pub mod Staking {
        use super::types::*;
    }
    pub mod Session {
        use super::types::*;
    }
    pub mod Democracy {
        use super::types::*;
    }
    pub mod Council {
        use super::types::*;
    }
    pub mod TechnicalCommittee {
        use super::types::*;
    }
    pub mod Elections {
        use super::types::*;
    }
    pub mod TechnicalMembership {
        use super::types::*;
    }
    pub mod Grandpa {
        use super::types::*;
    }
    pub mod Treasury {
        use super::types::*;
    }
    pub mod Contracts {
        use super::types::*;
    }
    pub mod Sudo {
        use super::types::*;
    }
    pub mod ImOnline {
        use super::types::*;
    }
    pub mod AuthorityDiscovery {
        use super::types::*;
    }
    pub mod Offences {
        use super::types::*;
    }
    pub mod Historical {
        use super::types::*;
    }
    pub mod RandomnessCollectiveFlip {
        use super::types::*;
    }
    pub mod Identity {
        use super::types::*;
    }
    pub mod Society {
        use super::types::*;
    }
    pub mod Recovery {
        use super::types::*;
    }
    pub mod Vesting {
        use super::types::*;
    }
    pub mod Scheduler {
        use super::types::*;
    }
    pub mod Proxy {
        use super::types::*;
    }
    pub mod Multisig {
        use super::types::*;
    }
}

```

</details>

