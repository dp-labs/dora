use crate::dora;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Operation {
    Stop = 0x00,
    Add = 0x01,
    Sub = 0x03,
    Mul = 0x02,
    Div = 0x04,
    SDiv = 0x05,
    Mod = 0x06,
    SMod = 0x07,
    AddMod = 0x08,
    MulMod = 0x09,
    Exp = 0x0A,
    SignExtend = 0x0B,
    Lt = 0x10,
    Gt = 0x11,
    Slt = 0x12,
    Sgt = 0x13,
    Eq = 0x14,
    IsZero = 0x15,
    And = 0x16,
    Or = 0x17,
    Xor = 0x18,
    Not = 0x19,
    Byte = 0x1A,
    Shl = 0x1B,
    Shr = 0x1C,
    Sar = 0x1D,
    Keccak256 = 0x20,
    Address = 0x30,
    Balance = 0x31,
    Origin = 0x32,
    Caller = 0x33,
    CallValue = 0x34,
    CalldataLoad = 0x35,
    CalldataSize = 0x36,
    CalldataCopy = 0x37,
    CodeSize = 0x38,
    CodeCopy = 0x39,
    GasPrice = 0x3A,
    ExtCodeSize = 0x3B,
    ExtCodeCopy = 0x3C,
    ReturndataSize = 0x3D,
    ReturndataCopy = 0x3E,
    ExtCodeHash = 0x3F,
    BlockHash = 0x40,
    Coinbase = 0x41,
    Timestamp = 0x42,
    Number = 0x43,
    PrevRandao = 0x44,
    GasLimit = 0x45,
    ChainId = 0x46,
    SelfBalance = 0x47,
    BaseFee = 0x48,
    BlobHash = 0x49,
    BlobBaseFee = 0x4A,
    MLoad = 0x51,
    MStore = 0x52,
    MStore8 = 0x53,
    SLoad = 0x54,
    SStore = 0x55,
    MSize = 0x59,
    Gas = 0x5A,
    TLoad = 0x5C,
    TStore = 0x5D,
    MCopy = 0x5E,
    Log0 = 0xA0,
    Log1 = 0xA1,
    Log2 = 0xA2,
    Log3 = 0xA3,
    Log4 = 0xA4,
    DataLoad = 0xD0,
    DataLoadN = 0xD1,
    DataSize = 0xD2,
    DataCopy = 0xD3,
    CallF = 0xE3,
    RetF = 0xE4,
    EofCreate = 0xEC,
    ReturnContract = 0xEE,
    Create = 0xF0,
    Call = 0xF1,
    Callcode = 0xF2,
    Return = 0xF3,
    Delegatecall = 0xF4,
    Create2 = 0xF5,
    ReturndataLoad = 0xF7,
    ExtCall = 0xF8,
    ExtDelegatecall = 0xF9,
    Staticcall = 0xFA,
    ExtStaticcall = 0xFB,
    Revert = 0xFD,
    Invalid = 0xFE,
    SelfDestruct = 0xFF,
}

impl Operation {
    pub fn name(&self) -> &'static str {
        match self {
            Operation::Stop => dora::StopOperation::name(),
            Operation::Add => dora::AddOperation::name(),
            Operation::Sub => dora::SubOperation::name(),
            Operation::Mul => dora::MulOperation::name(),
            Operation::Div => dora::DivOperation::name(),
            Operation::SDiv => dora::SDivOperation::name(),
            Operation::Mod => dora::ModOperation::name(),
            Operation::SMod => dora::SModOperation::name(),
            Operation::AddMod => dora::AddModOperation::name(),
            Operation::MulMod => dora::MulModOperation::name(),
            Operation::Exp => dora::ExpOperation::name(),
            Operation::SignExtend => dora::SignExtendOperation::name(),
            Operation::Lt => dora::LtOperation::name(),
            Operation::Gt => dora::GtOperation::name(),
            Operation::Slt => dora::SltOperation::name(),
            Operation::Sgt => dora::SgtOperation::name(),
            Operation::Eq => dora::EqOperation::name(),
            Operation::IsZero => dora::IsZeroOperation::name(),
            Operation::And => dora::AndOperation::name(),
            Operation::Or => dora::OrOperation::name(),
            Operation::Xor => dora::XorOperation::name(),
            Operation::Not => dora::NotOperation::name(),
            Operation::Byte => dora::ByteOperation::name(),
            Operation::Shl => dora::ShlOperation::name(),
            Operation::Shr => dora::ShrOperation::name(),
            Operation::Sar => dora::SarOperation::name(),
            Operation::Keccak256 => dora::Keccak256Operation::name(),
            Operation::Address => dora::AddressOperation::name(),
            Operation::Balance => dora::BalanceOperation::name(),
            Operation::Origin => dora::OriginOperation::name(),
            Operation::Caller => dora::CallerOperation::name(),
            Operation::CallValue => dora::CallValueOperation::name(),
            Operation::CalldataLoad => dora::CalldataLoadOperation::name(),
            Operation::CalldataSize => dora::CalldataSizeOperation::name(),
            Operation::CalldataCopy => dora::CalldataCopyOperation::name(),
            Operation::CodeSize => dora::CodeSizeOperation::name(),
            Operation::CodeCopy => dora::CodeCopyOperation::name(),
            Operation::GasPrice => dora::GasPriceOperation::name(),
            Operation::ExtCodeSize => dora::ExtCodeSizeOperation::name(),
            Operation::ExtCodeCopy => dora::ExtCodeCopyOperation::name(),
            Operation::ReturndataSize => dora::ReturndataSizeOperation::name(),
            Operation::ReturndataCopy => dora::ReturndataCopyOperation::name(),
            Operation::ExtCodeHash => dora::ExtCodeHashOperation::name(),
            Operation::BlockHash => dora::BlockHashOperation::name(),
            Operation::Coinbase => dora::CoinbaseOperation::name(),
            Operation::Timestamp => dora::TimestampOperation::name(),
            Operation::Number => dora::NumberOperation::name(),
            Operation::PrevRandao => dora::PrevRandaoOperation::name(),
            Operation::GasLimit => dora::GasLimitOperation::name(),
            Operation::ChainId => dora::ChainIdOperation::name(),
            Operation::SelfBalance => dora::SelfBalanceOperation::name(),
            Operation::BaseFee => dora::BaseFeeOperation::name(),
            Operation::BlobHash => dora::BlobHashOperation::name(),
            Operation::BlobBaseFee => dora::BlobBaseFeeOperation::name(),
            Operation::MLoad => dora::MLoadOperation::name(),
            Operation::MStore => dora::MStoreOperation::name(),
            Operation::MStore8 => dora::MStore8Operation::name(),
            Operation::SLoad => dora::SLoadOperation::name(),
            Operation::SStore => dora::SStoreOperation::name(),
            Operation::MSize => dora::MSizeOperation::name(),
            Operation::Gas => dora::GasOperation::name(),
            Operation::TLoad => dora::TLoadOperation::name(),
            Operation::TStore => dora::TStoreOperation::name(),
            Operation::MCopy => dora::MCopyOperation::name(),
            Operation::Log0 => dora::Log0Operation::name(),
            Operation::Log1 => dora::Log1Operation::name(),
            Operation::Log2 => dora::Log2Operation::name(),
            Operation::Log3 => dora::Log3Operation::name(),
            Operation::Log4 => dora::Log4Operation::name(),
            Operation::DataLoad => dora::DataLoadOperation::name(),
            Operation::DataLoadN => dora::DataLoadNOperation::name(),
            Operation::DataSize => dora::DataSizeOperation::name(),
            Operation::DataCopy => dora::DataCopyOperation::name(),
            Operation::CallF => dora::CallFOperation::name(),
            Operation::RetF => dora::RetFOperation::name(),
            Operation::EofCreate => dora::EofCreateOperation::name(),
            Operation::ReturnContract => dora::ReturnContractOperation::name(),
            Operation::Create => dora::CreateOperation::name(),
            Operation::Call => dora::CallOperation::name(),
            Operation::Callcode => dora::CallcodeOperation::name(),
            Operation::Return => dora::ReturnOperation::name(),
            Operation::Delegatecall => dora::DelegatecallOperation::name(),
            Operation::Create2 => dora::Create2Operation::name(),
            Operation::ReturndataLoad => dora::ReturndataLoadOperation::name(),
            Operation::ExtCall => dora::ExtCallOperation::name(),
            Operation::ExtDelegatecall => dora::ExtDelegatecallOperation::name(),
            Operation::Staticcall => dora::StaticcallOperation::name(),
            Operation::ExtStaticcall => dora::ExtStaticcallOperation::name(),
            Operation::Revert => dora::RevertOperation::name(),
            Operation::Invalid => dora::InvalidOperation::name(),
            Operation::SelfDestruct => dora::SelfDestructOperation::name(),
        }
    }

    #[inline]
    pub fn is_storage_op(&self) -> bool {
        matches!(
            self,
            Operation::SLoad | Operation::SStore | Operation::TLoad | Operation::TStore
        )
    }
}

impl TryFrom<&str> for Operation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            x if x == dora::StopOperation::name() => Ok(Operation::Stop),
            x if x == dora::AddOperation::name() => Ok(Operation::Add),
            x if x == dora::SubOperation::name() => Ok(Operation::Sub),
            x if x == dora::MulOperation::name() => Ok(Operation::Mul),
            x if x == dora::DivOperation::name() => Ok(Operation::Div),
            x if x == dora::SDivOperation::name() => Ok(Operation::SDiv),
            x if x == dora::ModOperation::name() => Ok(Operation::Mod),
            x if x == dora::SModOperation::name() => Ok(Operation::SMod),
            x if x == dora::AddModOperation::name() => Ok(Operation::AddMod),
            x if x == dora::MulModOperation::name() => Ok(Operation::MulMod),
            x if x == dora::ExpOperation::name() => Ok(Operation::Exp),
            x if x == dora::SignExtendOperation::name() => Ok(Operation::SignExtend),
            x if x == dora::LtOperation::name() => Ok(Operation::Lt),
            x if x == dora::GtOperation::name() => Ok(Operation::Gt),
            x if x == dora::SltOperation::name() => Ok(Operation::Slt),
            x if x == dora::SgtOperation::name() => Ok(Operation::Sgt),
            x if x == dora::EqOperation::name() => Ok(Operation::Eq),
            x if x == dora::IsZeroOperation::name() => Ok(Operation::IsZero),
            x if x == dora::AndOperation::name() => Ok(Operation::And),
            x if x == dora::OrOperation::name() => Ok(Operation::Or),
            x if x == dora::XorOperation::name() => Ok(Operation::Xor),
            x if x == dora::NotOperation::name() => Ok(Operation::Not),
            x if x == dora::ByteOperation::name() => Ok(Operation::Byte),
            x if x == dora::ShlOperation::name() => Ok(Operation::Shl),
            x if x == dora::ShrOperation::name() => Ok(Operation::Shr),
            x if x == dora::SarOperation::name() => Ok(Operation::Sar),
            x if x == dora::Keccak256Operation::name() => Ok(Operation::Keccak256),
            x if x == dora::AddressOperation::name() => Ok(Operation::Address),
            x if x == dora::BalanceOperation::name() => Ok(Operation::Balance),
            x if x == dora::OriginOperation::name() => Ok(Operation::Origin),
            x if x == dora::CallerOperation::name() => Ok(Operation::Caller),
            x if x == dora::CallValueOperation::name() => Ok(Operation::CallValue),
            x if x == dora::CalldataLoadOperation::name() => Ok(Operation::CalldataLoad),
            x if x == dora::CalldataSizeOperation::name() => Ok(Operation::CalldataSize),
            x if x == dora::CalldataCopyOperation::name() => Ok(Operation::CalldataCopy),
            x if x == dora::CodeSizeOperation::name() => Ok(Operation::CodeSize),
            x if x == dora::CodeCopyOperation::name() => Ok(Operation::CodeCopy),
            x if x == dora::GasPriceOperation::name() => Ok(Operation::GasPrice),
            x if x == dora::ExtCodeSizeOperation::name() => Ok(Operation::ExtCodeSize),
            x if x == dora::ExtCodeCopyOperation::name() => Ok(Operation::ExtCodeCopy),
            x if x == dora::ReturndataSizeOperation::name() => Ok(Operation::ReturndataSize),
            x if x == dora::ReturndataCopyOperation::name() => Ok(Operation::ReturndataCopy),
            x if x == dora::ExtCodeHashOperation::name() => Ok(Operation::ExtCodeHash),
            x if x == dora::BlockHashOperation::name() => Ok(Operation::BlockHash),
            x if x == dora::CoinbaseOperation::name() => Ok(Operation::Coinbase),
            x if x == dora::TimestampOperation::name() => Ok(Operation::Timestamp),
            x if x == dora::NumberOperation::name() => Ok(Operation::Number),
            x if x == dora::PrevRandaoOperation::name() => Ok(Operation::PrevRandao),
            x if x == dora::GasLimitOperation::name() => Ok(Operation::GasLimit),
            x if x == dora::ChainIdOperation::name() => Ok(Operation::ChainId),
            x if x == dora::SelfBalanceOperation::name() => Ok(Operation::SelfBalance),
            x if x == dora::BaseFeeOperation::name() => Ok(Operation::BaseFee),
            x if x == dora::BlobHashOperation::name() => Ok(Operation::BlobHash),
            x if x == dora::BlobBaseFeeOperation::name() => Ok(Operation::BlobBaseFee),
            x if x == dora::MLoadOperation::name() => Ok(Operation::MLoad),
            x if x == dora::MStoreOperation::name() => Ok(Operation::MStore),
            x if x == dora::MStore8Operation::name() => Ok(Operation::MStore8),
            x if x == dora::SLoadOperation::name() => Ok(Operation::SLoad),
            x if x == dora::SStoreOperation::name() => Ok(Operation::SStore),
            x if x == dora::MSizeOperation::name() => Ok(Operation::MSize),
            x if x == dora::GasOperation::name() => Ok(Operation::Gas),
            x if x == dora::TLoadOperation::name() => Ok(Operation::TLoad),
            x if x == dora::TStoreOperation::name() => Ok(Operation::TStore),
            x if x == dora::MCopyOperation::name() => Ok(Operation::MCopy),
            x if x == dora::Log0Operation::name() => Ok(Operation::Log0),
            x if x == dora::Log1Operation::name() => Ok(Operation::Log1),
            x if x == dora::Log2Operation::name() => Ok(Operation::Log2),
            x if x == dora::Log3Operation::name() => Ok(Operation::Log3),
            x if x == dora::Log4Operation::name() => Ok(Operation::Log4),
            x if x == dora::DataLoadOperation::name() => Ok(Operation::DataLoad),
            x if x == dora::DataLoadNOperation::name() => Ok(Operation::DataLoadN),
            x if x == dora::DataSizeOperation::name() => Ok(Operation::DataSize),
            x if x == dora::DataCopyOperation::name() => Ok(Operation::DataCopy),
            x if x == dora::CallFOperation::name() => Ok(Operation::CallF),
            x if x == dora::RetFOperation::name() => Ok(Operation::RetF),
            x if x == dora::EofCreateOperation::name() => Ok(Operation::EofCreate),
            x if x == dora::ReturnContractOperation::name() => Ok(Operation::ReturnContract),
            x if x == dora::CreateOperation::name() => Ok(Operation::Create),
            x if x == dora::CallOperation::name() => Ok(Operation::Call),
            x if x == dora::CallcodeOperation::name() => Ok(Operation::Callcode),
            x if x == dora::ReturnOperation::name() => Ok(Operation::Return),
            x if x == dora::DelegatecallOperation::name() => Ok(Operation::Delegatecall),
            x if x == dora::Create2Operation::name() => Ok(Operation::Create2),
            x if x == dora::ReturndataLoadOperation::name() => Ok(Operation::ReturndataLoad),
            x if x == dora::ExtCallOperation::name() => Ok(Operation::ExtCall),
            x if x == dora::ExtDelegatecallOperation::name() => Ok(Operation::ExtDelegatecall),
            x if x == dora::StaticcallOperation::name() => Ok(Operation::Staticcall),
            x if x == dora::ExtStaticcallOperation::name() => Ok(Operation::ExtStaticcall),
            x if x == dora::RevertOperation::name() => Ok(Operation::Revert),
            x if x == dora::InvalidOperation::name() => Ok(Operation::Invalid),
            x if x == dora::SelfDestructOperation::name() => Ok(Operation::SelfDestruct),
            _ => Err(()),
        }
    }
}
