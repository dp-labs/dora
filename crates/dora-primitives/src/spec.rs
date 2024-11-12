use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum SpecName {
    Frontier,
    FrontierToHomesteadAt5,
    Homestead,
    HomesteadToDaoAt5,
    HomesteadToEIP150At5,
    EIP150,
    EIP158, // EIP-161: State trie clearing
    EIP158ToByzantiumAt5,
    Byzantium,
    ByzantiumToConstantinopleAt5, // SKIPPED
    ByzantiumToConstantinopleFixAt5,
    Constantinople, // SKIPPED
    ConstantinopleFix,
    Istanbul,
    Berlin,
    BerlinToLondonAt5,
    London,
    Paris,
    Merge,
    Shanghai,
    Cancun,
    Prague,
    PragueEOF,
    Osaka, // SKIPPED
    #[serde(other)]
    Unknown,
}
