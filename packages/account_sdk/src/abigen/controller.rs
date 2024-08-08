#[derive(Debug)]
pub struct Controller<A: starknet::accounts::ConnectedAccount + Sync> {
    pub address: starknet::core::types::Felt,
    pub account: A,
    pub block_id: starknet::core::types::BlockId,
}
impl<A: starknet::accounts::ConnectedAccount + Sync> Controller<A> {
    pub fn new(address: starknet::core::types::Felt, account: A) -> Self {
        Self {
            address,
            account,
            block_id: starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending),
        }
    }
    pub fn set_contract_address(&mut self, address: starknet::core::types::Felt) {
        self.address = address;
    }
    pub fn provider(&self) -> &A::Provider {
        self.account.provider()
    }
    pub fn set_block(&mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive(Debug)]
pub struct ControllerReader<P: starknet::providers::Provider + Sync> {
    pub address: starknet::core::types::Felt,
    pub provider: P,
    pub block_id: starknet::core::types::BlockId,
}
impl<P: starknet::providers::Provider + Sync> ControllerReader<P> {
    pub fn new(address: starknet::core::types::Felt, provider: P) -> Self {
        Self {
            address,
            provider,
            block_id: starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending),
        }
    }
    pub fn set_contract_address(&mut self, address: starknet::core::types::Felt) {
        self.address = address;
    }
    pub fn provider(&self) -> &P {
        &self.provider
    }
    pub fn set_block(&mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct ExternalOwnerRegistered {
    pub address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for ExternalOwnerRegistered {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.address);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.address,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let address = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
        Ok(ExternalOwnerRegistered { address })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct OwnerAdded {
    pub owner: Signer,
}
impl cainome::cairo_serde::CairoSerde for OwnerAdded {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += Signer::cairo_serialized_size(&__rust.owner);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(Signer::cairo_serialize(&__rust.owner));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let owner = Signer::cairo_deserialize(__felts, __offset)?;
        __offset += Signer::cairo_serialized_size(&owner);
        Ok(OwnerAdded { owner })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct TransactionExecuted {
    pub hash: starknet::core::types::Felt,
    pub response: Vec<Vec<starknet::core::types::Felt>>,
}
impl cainome::cairo_serde::CairoSerde for TransactionExecuted {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.hash);
        __size += Vec::<Vec<starknet::core::types::Felt>>::cairo_serialized_size(&__rust.response);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.hash));
        __out.extend(Vec::<Vec<starknet::core::types::Felt>>::cairo_serialize(
            &__rust.response,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&hash);
        let response =
            Vec::<Vec<starknet::core::types::Felt>>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<Vec<starknet::core::types::Felt>>::cairo_serialized_size(&response);
        Ok(TransactionExecuted { hash, response })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct StarknetSignature {
    pub r: starknet::core::types::Felt,
    pub s: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for StarknetSignature {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.r);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.s);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.r));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.s));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let r = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&r);
        let s = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&s);
        Ok(StarknetSignature { r, s })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct OutsideExecution {
    pub caller: cainome::cairo_serde::ContractAddress,
    pub nonce: starknet::core::types::Felt,
    pub execute_after: u64,
    pub execute_before: u64,
    pub calls: Vec<Call>,
}
impl cainome::cairo_serde::CairoSerde for OutsideExecution {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.caller);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.nonce);
        __size += u64::cairo_serialized_size(&__rust.execute_after);
        __size += u64::cairo_serialized_size(&__rust.execute_before);
        __size += Vec::<Call>::cairo_serialized_size(&__rust.calls);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.caller,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.nonce));
        __out.extend(u64::cairo_serialize(&__rust.execute_after));
        __out.extend(u64::cairo_serialize(&__rust.execute_before));
        __out.extend(Vec::<Call>::cairo_serialize(&__rust.calls));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let caller = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&caller);
        let nonce = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&nonce);
        let execute_after = u64::cairo_deserialize(__felts, __offset)?;
        __offset += u64::cairo_serialized_size(&execute_after);
        let execute_before = u64::cairo_deserialize(__felts, __offset)?;
        __offset += u64::cairo_serialized_size(&execute_before);
        let calls = Vec::<Call>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<Call>::cairo_serialized_size(&calls);
        Ok(OutsideExecution {
            caller,
            nonce,
            execute_after,
            execute_before,
            calls,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct ExternalOwnerRemoved {
    pub address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for ExternalOwnerRemoved {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.address);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.address,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let address = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
        Ok(ExternalOwnerRemoved { address })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Eip191Signer {
    pub eth_address: cainome::cairo_serde::EthAddress,
}
impl cainome::cairo_serde::CairoSerde for Eip191Signer {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::EthAddress::cairo_serialized_size(&__rust.eth_address);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::EthAddress::cairo_serialize(
            &__rust.eth_address,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let eth_address = cainome::cairo_serde::EthAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::EthAddress::cairo_serialized_size(&eth_address);
        Ok(Eip191Signer { eth_address })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Secp256k1Signer {
    pub pubkey_hash: cainome::cairo_serde::EthAddress,
}
impl cainome::cairo_serde::CairoSerde for Secp256k1Signer {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::EthAddress::cairo_serialized_size(&__rust.pubkey_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::EthAddress::cairo_serialize(
            &__rust.pubkey_hash,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let pubkey_hash = cainome::cairo_serde::EthAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::EthAddress::cairo_serialized_size(&pubkey_hash);
        Ok(Secp256k1Signer { pubkey_hash })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Upgraded {
    pub class_hash: cainome::cairo_serde::ClassHash,
}
impl cainome::cairo_serde::CairoSerde for Upgraded {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ClassHash::cairo_serialized_size(&__rust.class_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            &__rust.class_hash,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let class_hash = cainome::cairo_serde::ClassHash::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
        Ok(Upgraded { class_hash })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct DelegateAccountChanged {
    pub address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for DelegateAccountChanged {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.address);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.address,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let address = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
        Ok(DelegateAccountChanged { address })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Session {
    pub expires_at: u64,
    pub allowed_methods_root: starknet::core::types::Felt,
    pub metadata_hash: starknet::core::types::Felt,
    pub session_key_guid: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for Session {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += u64::cairo_serialized_size(&__rust.expires_at);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.allowed_methods_root);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.metadata_hash);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.session_key_guid);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(u64::cairo_serialize(&__rust.expires_at));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.allowed_methods_root,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.metadata_hash,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.session_key_guid,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let expires_at = u64::cairo_deserialize(__felts, __offset)?;
        __offset += u64::cairo_serialized_size(&expires_at);
        let allowed_methods_root =
            starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&allowed_methods_root);
        let metadata_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&metadata_hash);
        let session_key_guid = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&session_key_guid);
        Ok(Session {
            expires_at,
            allowed_methods_root,
            metadata_hash,
            session_key_guid,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Signature {
    pub r: cainome::cairo_serde::U256,
    pub s: cainome::cairo_serde::U256,
    pub y_parity: bool,
}
impl cainome::cairo_serde::CairoSerde for Signature {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.r);
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.s);
        __size += bool::cairo_serialized_size(&__rust.y_parity);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.r));
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.s));
        __out.extend(bool::cairo_serialize(&__rust.y_parity));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let r = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&r);
        let s = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&s);
        let y_parity = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&y_parity);
        Ok(Signature { r, s, y_parity })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct OwnerRemoved {
    pub owner: Signer,
}
impl cainome::cairo_serde::CairoSerde for OwnerRemoved {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += Signer::cairo_serialized_size(&__rust.owner);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(Signer::cairo_serialize(&__rust.owner));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let owner = Signer::cairo_deserialize(__felts, __offset)?;
        __offset += Signer::cairo_serialized_size(&owner);
        Ok(OwnerRemoved { owner })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Call {
    pub to: cainome::cairo_serde::ContractAddress,
    pub selector: starknet::core::types::Felt,
    pub calldata: Vec<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for Call {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&__rust.to);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.selector);
        __size += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&__rust.calldata);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            &__rust.to,
        ));
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.selector,
        ));
        __out.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            &__rust.calldata,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let to = cainome::cairo_serde::ContractAddress::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&to);
        let selector = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&selector);
        let calldata = Vec::<starknet::core::types::Felt>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<starknet::core::types::Felt>::cairo_serialized_size(&calldata);
        Ok(Call {
            to,
            selector,
            calldata,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Secp256r1Signer {
    pub pubkey: cainome::cairo_serde::NonZero<cainome::cairo_serde::U256>,
}
impl cainome::cairo_serde::CairoSerde for Secp256r1Signer {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_serialized_size(
                &__rust.pubkey,
            );
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_serialize(
                &__rust.pubkey,
            ),
        );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let pubkey =
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_deserialize(
                __felts, __offset,
            )?;
        __offset +=
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_serialized_size(
                &pubkey,
            );
        Ok(Secp256r1Signer { pubkey })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct SessionRevoked {
    pub session_hash: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for SessionRevoked {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.session_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.session_hash,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let session_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&session_hash);
        Ok(SessionRevoked { session_hash })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct WebauthnSignature {
    pub cross_origin: bool,
    pub client_data_json_outro: Vec<u8>,
    pub flags: u8,
    pub sign_count: u32,
    pub ec_signature: Signature,
    pub sha256_implementation: Sha256Implementation,
}
impl cainome::cairo_serde::CairoSerde for WebauthnSignature {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += bool::cairo_serialized_size(&__rust.cross_origin);
        __size += Vec::<u8>::cairo_serialized_size(&__rust.client_data_json_outro);
        __size += u8::cairo_serialized_size(&__rust.flags);
        __size += u32::cairo_serialized_size(&__rust.sign_count);
        __size += Signature::cairo_serialized_size(&__rust.ec_signature);
        __size += Sha256Implementation::cairo_serialized_size(&__rust.sha256_implementation);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(bool::cairo_serialize(&__rust.cross_origin));
        __out.extend(Vec::<u8>::cairo_serialize(&__rust.client_data_json_outro));
        __out.extend(u8::cairo_serialize(&__rust.flags));
        __out.extend(u32::cairo_serialize(&__rust.sign_count));
        __out.extend(Signature::cairo_serialize(&__rust.ec_signature));
        __out.extend(Sha256Implementation::cairo_serialize(
            &__rust.sha256_implementation,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let cross_origin = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&cross_origin);
        let client_data_json_outro = Vec::<u8>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<u8>::cairo_serialized_size(&client_data_json_outro);
        let flags = u8::cairo_deserialize(__felts, __offset)?;
        __offset += u8::cairo_serialized_size(&flags);
        let sign_count = u32::cairo_deserialize(__felts, __offset)?;
        __offset += u32::cairo_serialized_size(&sign_count);
        let ec_signature = Signature::cairo_deserialize(__felts, __offset)?;
        __offset += Signature::cairo_serialized_size(&ec_signature);
        let sha256_implementation = Sha256Implementation::cairo_deserialize(__felts, __offset)?;
        __offset += Sha256Implementation::cairo_serialized_size(&sha256_implementation);
        Ok(WebauthnSignature {
            cross_origin,
            client_data_json_outro,
            flags,
            sign_count,
            ec_signature,
            sha256_implementation,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct SessionRegistered {
    pub session_hash: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for SessionRegistered {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.session_hash);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(
            &__rust.session_hash,
        ));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let session_hash = starknet::core::types::Felt::cairo_deserialize(__felts, __offset)?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&session_hash);
        Ok(SessionRegistered { session_hash })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct WebauthnSigner {
    pub origin: Vec<u8>,
    pub rp_id_hash: cainome::cairo_serde::NonZero<cainome::cairo_serde::U256>,
    pub pubkey: cainome::cairo_serde::NonZero<cainome::cairo_serde::U256>,
}
impl cainome::cairo_serde::CairoSerde for WebauthnSigner {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += Vec::<u8>::cairo_serialized_size(&__rust.origin);
        __size +=
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_serialized_size(
                &__rust.rp_id_hash,
            );
        __size +=
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_serialized_size(
                &__rust.pubkey,
            );
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(Vec::<u8>::cairo_serialize(&__rust.origin));
        __out.extend(
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_serialize(
                &__rust.rp_id_hash,
            ),
        );
        __out.extend(
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_serialize(
                &__rust.pubkey,
            ),
        );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let origin = Vec::<u8>::cairo_deserialize(__felts, __offset)?;
        __offset += Vec::<u8>::cairo_serialized_size(&origin);
        let rp_id_hash =
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_deserialize(
                __felts, __offset,
            )?;
        __offset +=
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_serialized_size(
                &rp_id_hash,
            );
        let pubkey =
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_deserialize(
                __felts, __offset,
            )?;
        __offset +=
            cainome::cairo_serde::NonZero::<cainome::cairo_serde::U256>::cairo_serialized_size(
                &pubkey,
            );
        Ok(WebauthnSigner {
            origin,
            rp_id_hash,
            pubkey,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct StarknetSigner {
    pub pubkey: cainome::cairo_serde::NonZero<starknet::core::types::Felt>,
}
impl cainome::cairo_serde::CairoSerde for StarknetSigner {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size +=
            cainome::cairo_serde::NonZero::<starknet::core::types::Felt>::cairo_serialized_size(
                &__rust.pubkey,
            );
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(
            cainome::cairo_serde::NonZero::<starknet::core::types::Felt>::cairo_serialize(
                &__rust.pubkey,
            ),
        );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let pubkey =
            cainome::cairo_serde::NonZero::<starknet::core::types::Felt>::cairo_deserialize(
                __felts, __offset,
            )?;
        __offset +=
            cainome::cairo_serde::NonZero::<starknet::core::types::Felt>::cairo_serialized_size(
                &pubkey,
            );
        Ok(StarknetSigner { pubkey })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum MultipleOwnersEvent {
    OwnerAdded(OwnerAdded),
    OwnerRemoved(OwnerRemoved),
}
impl cainome::cairo_serde::CairoSerde for MultipleOwnersEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            MultipleOwnersEvent::OwnerAdded(val) => OwnerAdded::cairo_serialized_size(val) + 1,
            MultipleOwnersEvent::OwnerRemoved(val) => OwnerRemoved::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            MultipleOwnersEvent::OwnerAdded(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(OwnerAdded::cairo_serialize(val));
                temp
            }
            MultipleOwnersEvent::OwnerRemoved(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(OwnerRemoved::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(MultipleOwnersEvent::OwnerAdded(
                OwnerAdded::cairo_deserialize(__felts, __offset + 1)?,
            )),
            1usize => Ok(MultipleOwnersEvent::OwnerRemoved(
                OwnerRemoved::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "MultipleOwnersEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for MultipleOwnersEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnerAdded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnerAdded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let owner = match Signer::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "owner", "OwnerAdded", e
                    ))
                }
            };
            data_offset += Signer::cairo_serialized_size(&owner);
            return Ok(MultipleOwnersEvent::OwnerAdded(OwnerAdded { owner }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnerRemoved")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnerRemoved"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let owner = match Signer::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "owner", "OwnerRemoved", e
                    ))
                }
            };
            data_offset += Signer::cairo_serialized_size(&owner);
            return Ok(MultipleOwnersEvent::OwnerRemoved(OwnerRemoved { owner }));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum SignerSignature {
    Starknet((StarknetSigner, StarknetSignature)),
    Secp256k1((Secp256k1Signer, Signature)),
    Secp256r1((Secp256r1Signer, Signature)),
    Eip191((Eip191Signer, Signature)),
    Webauthn((WebauthnSigner, WebauthnSignature)),
}
impl cainome::cairo_serde::CairoSerde for SignerSignature {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            SignerSignature::Starknet(val) => {
                <(StarknetSigner, StarknetSignature)>::cairo_serialized_size(val) + 1
            }
            SignerSignature::Secp256k1(val) => {
                <(Secp256k1Signer, Signature)>::cairo_serialized_size(val) + 1
            }
            SignerSignature::Secp256r1(val) => {
                <(Secp256r1Signer, Signature)>::cairo_serialized_size(val) + 1
            }
            SignerSignature::Eip191(val) => {
                <(Eip191Signer, Signature)>::cairo_serialized_size(val) + 1
            }
            SignerSignature::Webauthn(val) => {
                <(WebauthnSigner, WebauthnSignature)>::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            SignerSignature::Starknet(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(<(StarknetSigner, StarknetSignature)>::cairo_serialize(val));
                temp
            }
            SignerSignature::Secp256k1(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(<(Secp256k1Signer, Signature)>::cairo_serialize(val));
                temp
            }
            SignerSignature::Secp256r1(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(<(Secp256r1Signer, Signature)>::cairo_serialize(val));
                temp
            }
            SignerSignature::Eip191(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(<(Eip191Signer, Signature)>::cairo_serialize(val));
                temp
            }
            SignerSignature::Webauthn(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&4usize));
                temp.extend(<(WebauthnSigner, WebauthnSignature)>::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(SignerSignature::Starknet(<(
                StarknetSigner,
                StarknetSignature,
            )>::cairo_deserialize(
                __felts, __offset + 1
            )?)),
            1usize => Ok(SignerSignature::Secp256k1(
                <(Secp256k1Signer, Signature)>::cairo_deserialize(__felts, __offset + 1)?,
            )),
            2usize => Ok(SignerSignature::Secp256r1(
                <(Secp256r1Signer, Signature)>::cairo_deserialize(__felts, __offset + 1)?,
            )),
            3usize => Ok(SignerSignature::Eip191(
                <(Eip191Signer, Signature)>::cairo_deserialize(__felts, __offset + 1)?,
            )),
            4usize => Ok(SignerSignature::Webauthn(<(
                WebauthnSigner,
                WebauthnSignature,
            )>::cairo_deserialize(
                __felts, __offset + 1
            )?)),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "SignerSignature"
                )))
            }
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum Event {
    TransactionExecuted(TransactionExecuted),
    MultipleOwnersEvent(MultipleOwnersEvent),
    ReentrancyGuardEvent(ReentrancyGuardEvent),
    SessionEvent(SessionEvent),
    ExternalOwnersEvent(ExternalOwnersEvent),
    ExecuteFromOutsideEvents(OutsideExecutionEvent),
    DelegateAccountEvents(DelegateAccountEvent),
    SRC5Events(Src5ComponentEvent),
    UpgradeableEvent(UpgradeEvent),
}
impl cainome::cairo_serde::CairoSerde for Event {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Event::TransactionExecuted(val) => TransactionExecuted::cairo_serialized_size(val) + 1,
            Event::MultipleOwnersEvent(val) => MultipleOwnersEvent::cairo_serialized_size(val) + 1,
            Event::ReentrancyGuardEvent(val) => {
                ReentrancyGuardEvent::cairo_serialized_size(val) + 1
            }
            Event::SessionEvent(val) => SessionEvent::cairo_serialized_size(val) + 1,
            Event::ExternalOwnersEvent(val) => ExternalOwnersEvent::cairo_serialized_size(val) + 1,
            Event::ExecuteFromOutsideEvents(val) => {
                OutsideExecutionEvent::cairo_serialized_size(val) + 1
            }
            Event::DelegateAccountEvents(val) => {
                DelegateAccountEvent::cairo_serialized_size(val) + 1
            }
            Event::SRC5Events(val) => Src5ComponentEvent::cairo_serialized_size(val) + 1,
            Event::UpgradeableEvent(val) => UpgradeEvent::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            Event::TransactionExecuted(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(TransactionExecuted::cairo_serialize(val));
                temp
            }
            Event::MultipleOwnersEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(MultipleOwnersEvent::cairo_serialize(val));
                temp
            }
            Event::ReentrancyGuardEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(ReentrancyGuardEvent::cairo_serialize(val));
                temp
            }
            Event::SessionEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(SessionEvent::cairo_serialize(val));
                temp
            }
            Event::ExternalOwnersEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&4usize));
                temp.extend(ExternalOwnersEvent::cairo_serialize(val));
                temp
            }
            Event::ExecuteFromOutsideEvents(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&5usize));
                temp.extend(OutsideExecutionEvent::cairo_serialize(val));
                temp
            }
            Event::DelegateAccountEvents(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&6usize));
                temp.extend(DelegateAccountEvent::cairo_serialize(val));
                temp
            }
            Event::SRC5Events(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&7usize));
                temp.extend(Src5ComponentEvent::cairo_serialize(val));
                temp
            }
            Event::UpgradeableEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&8usize));
                temp.extend(UpgradeEvent::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(Event::TransactionExecuted(
                TransactionExecuted::cairo_deserialize(__felts, __offset + 1)?,
            )),
            1usize => Ok(Event::MultipleOwnersEvent(
                MultipleOwnersEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            2usize => Ok(Event::ReentrancyGuardEvent(
                ReentrancyGuardEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            3usize => Ok(Event::SessionEvent(SessionEvent::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            4usize => Ok(Event::ExternalOwnersEvent(
                ExternalOwnersEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            5usize => Ok(Event::ExecuteFromOutsideEvents(
                OutsideExecutionEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            6usize => Ok(Event::DelegateAccountEvents(
                DelegateAccountEvent::cairo_deserialize(__felts, __offset + 1)?,
            )),
            7usize => Ok(Event::SRC5Events(Src5ComponentEvent::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            8usize => Ok(Event::UpgradeableEvent(UpgradeEvent::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "Event"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for Event {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("TransactionExecuted")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "TransactionExecuted"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let hash = match starknet::core::types::Felt::cairo_deserialize(&event.keys, key_offset)
            {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "hash", "TransactionExecuted", e
                    ))
                }
            };
            key_offset += starknet::core::types::Felt::cairo_serialized_size(&hash);
            let response = match Vec::<Vec<starknet::core::types::Felt>>::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "response", "TransactionExecuted", e
                    ))
                }
            };
            data_offset +=
                Vec::<Vec<starknet::core::types::Felt>>::cairo_serialized_size(&response);
            return Ok(Event::TransactionExecuted(TransactionExecuted {
                hash,
                response,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnerAdded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnerAdded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let owner = match Signer::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "owner", "OwnerAdded", e
                    ))
                }
            };
            data_offset += Signer::cairo_serialized_size(&owner);
            return Ok(Event::MultipleOwnersEvent(MultipleOwnersEvent::OwnerAdded(
                OwnerAdded { owner },
            )));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnerRemoved")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "OwnerRemoved"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let owner = match Signer::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "owner", "OwnerRemoved", e
                    ))
                }
            };
            data_offset += Signer::cairo_serialized_size(&owner);
            return Ok(Event::MultipleOwnersEvent(
                MultipleOwnersEvent::OwnerRemoved(OwnerRemoved { owner }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("SessionRevoked")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "SessionRevoked"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let session_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "session_hash", "SessionRevoked", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&session_hash);
            return Ok(Event::SessionEvent(SessionEvent::SessionRevoked(
                SessionRevoked { session_hash },
            )));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("SessionRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "SessionRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let session_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "session_hash", "SessionRegistered", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&session_hash);
            return Ok(Event::SessionEvent(SessionEvent::SessionRegistered(
                SessionRegistered { session_hash },
            )));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ExternalOwnerRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ExternalOwnerRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ExternalOwnerRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(Event::ExternalOwnersEvent(
                ExternalOwnersEvent::ExternalOwnerRegistered(ExternalOwnerRegistered { address }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ExternalOwnerRemoved")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ExternalOwnerRemoved"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ExternalOwnerRemoved", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(Event::ExternalOwnersEvent(
                ExternalOwnersEvent::ExternalOwnerRemoved(ExternalOwnerRemoved { address }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("DelegateAccountChanged")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "DelegateAccountChanged"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "DelegateAccountChanged", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(Event::DelegateAccountEvents(
                DelegateAccountEvent::DelegateAccountChanged(DelegateAccountChanged { address }),
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("Upgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "Upgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "Upgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(Event::UpgradeableEvent(UpgradeEvent::Upgraded(Upgraded {
                class_hash,
            })));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum SessionEvent {
    SessionRevoked(SessionRevoked),
    SessionRegistered(SessionRegistered),
}
impl cainome::cairo_serde::CairoSerde for SessionEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            SessionEvent::SessionRevoked(val) => SessionRevoked::cairo_serialized_size(val) + 1,
            SessionEvent::SessionRegistered(val) => {
                SessionRegistered::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            SessionEvent::SessionRevoked(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(SessionRevoked::cairo_serialize(val));
                temp
            }
            SessionEvent::SessionRegistered(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(SessionRegistered::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(SessionEvent::SessionRevoked(
                SessionRevoked::cairo_deserialize(__felts, __offset + 1)?,
            )),
            1usize => Ok(SessionEvent::SessionRegistered(
                SessionRegistered::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "SessionEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for SessionEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("SessionRevoked")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "SessionRevoked"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let session_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "session_hash", "SessionRevoked", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&session_hash);
            return Ok(SessionEvent::SessionRevoked(SessionRevoked {
                session_hash,
            }));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("SessionRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "SessionRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let session_hash =
                match starknet::core::types::Felt::cairo_deserialize(&event.data, data_offset) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "session_hash", "SessionRegistered", e
                        ))
                    }
                };
            data_offset += starknet::core::types::Felt::cairo_serialized_size(&session_hash);
            return Ok(SessionEvent::SessionRegistered(SessionRegistered {
                session_hash,
            }));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum OutsideExecutionEvent {}
impl cainome::cairo_serde::CairoSerde for OutsideExecutionEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "OutsideExecutionEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for OutsideExecutionEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DelegateAccountEvent {
    DelegateAccountChanged(DelegateAccountChanged),
}
impl cainome::cairo_serde::CairoSerde for DelegateAccountEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            DelegateAccountEvent::DelegateAccountChanged(val) => {
                DelegateAccountChanged::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            DelegateAccountEvent::DelegateAccountChanged(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(DelegateAccountChanged::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(DelegateAccountEvent::DelegateAccountChanged(
                DelegateAccountChanged::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "DelegateAccountEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for DelegateAccountEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("DelegateAccountChanged")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "DelegateAccountChanged"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "DelegateAccountChanged", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(DelegateAccountEvent::DelegateAccountChanged(
                DelegateAccountChanged { address },
            ));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum Signer {
    Starknet(StarknetSigner),
    Secp256k1(Secp256k1Signer),
    Secp256r1(Secp256r1Signer),
    Eip191(Eip191Signer),
    Webauthn(WebauthnSigner),
}
impl cainome::cairo_serde::CairoSerde for Signer {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Signer::Starknet(val) => StarknetSigner::cairo_serialized_size(val) + 1,
            Signer::Secp256k1(val) => Secp256k1Signer::cairo_serialized_size(val) + 1,
            Signer::Secp256r1(val) => Secp256r1Signer::cairo_serialized_size(val) + 1,
            Signer::Eip191(val) => Eip191Signer::cairo_serialized_size(val) + 1,
            Signer::Webauthn(val) => WebauthnSigner::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            Signer::Starknet(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(StarknetSigner::cairo_serialize(val));
                temp
            }
            Signer::Secp256k1(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(Secp256k1Signer::cairo_serialize(val));
                temp
            }
            Signer::Secp256r1(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(Secp256r1Signer::cairo_serialize(val));
                temp
            }
            Signer::Eip191(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(Eip191Signer::cairo_serialize(val));
                temp
            }
            Signer::Webauthn(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&4usize));
                temp.extend(WebauthnSigner::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(Signer::Starknet(StarknetSigner::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            1usize => Ok(Signer::Secp256k1(Secp256k1Signer::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            2usize => Ok(Signer::Secp256r1(Secp256r1Signer::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            3usize => Ok(Signer::Eip191(Eip191Signer::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            4usize => Ok(Signer::Webauthn(WebauthnSigner::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "Signer"
                )))
            }
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ExternalOwnersEvent {
    ExternalOwnerRegistered(ExternalOwnerRegistered),
    ExternalOwnerRemoved(ExternalOwnerRemoved),
}
impl cainome::cairo_serde::CairoSerde for ExternalOwnersEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            ExternalOwnersEvent::ExternalOwnerRegistered(val) => {
                ExternalOwnerRegistered::cairo_serialized_size(val) + 1
            }
            ExternalOwnersEvent::ExternalOwnerRemoved(val) => {
                ExternalOwnerRemoved::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            ExternalOwnersEvent::ExternalOwnerRegistered(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(ExternalOwnerRegistered::cairo_serialize(val));
                temp
            }
            ExternalOwnersEvent::ExternalOwnerRemoved(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(ExternalOwnerRemoved::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(ExternalOwnersEvent::ExternalOwnerRegistered(
                ExternalOwnerRegistered::cairo_deserialize(__felts, __offset + 1)?,
            )),
            1usize => Ok(ExternalOwnersEvent::ExternalOwnerRemoved(
                ExternalOwnerRemoved::cairo_deserialize(__felts, __offset + 1)?,
            )),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "ExternalOwnersEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for ExternalOwnersEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ExternalOwnerRegistered")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ExternalOwnerRegistered"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ExternalOwnerRegistered", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(ExternalOwnersEvent::ExternalOwnerRegistered(
                ExternalOwnerRegistered { address },
            ));
        };
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("ExternalOwnerRemoved")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "ExternalOwnerRemoved"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let address = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "address", "ExternalOwnerRemoved", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&address);
            return Ok(ExternalOwnersEvent::ExternalOwnerRemoved(
                ExternalOwnerRemoved { address },
            ));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum Sha256Implementation {
    Cairo0,
    Cairo1,
}
impl cainome::cairo_serde::CairoSerde for Sha256Implementation {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Sha256Implementation::Cairo0 => 1,
            Sha256Implementation::Cairo1 => 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            Sha256Implementation::Cairo0 => usize::cairo_serialize(&0usize),
            Sha256Implementation::Cairo1 => usize::cairo_serialize(&1usize),
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(Sha256Implementation::Cairo0),
            1usize => Ok(Sha256Implementation::Cairo1),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "Sha256Implementation"
                )))
            }
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ReentrancyGuardEvent {}
impl cainome::cairo_serde::CairoSerde for ReentrancyGuardEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "ReentrancyGuardEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for ReentrancyGuardEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum UpgradeEvent {
    Upgraded(Upgraded),
}
impl cainome::cairo_serde::CairoSerde for UpgradeEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            UpgradeEvent::Upgraded(val) => Upgraded::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            UpgradeEvent::Upgraded(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(Upgraded::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => Ok(UpgradeEvent::Upgraded(Upgraded::cairo_deserialize(
                __felts,
                __offset + 1,
            )?)),
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "UpgradeEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for UpgradeEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("Upgraded")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "Upgraded"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let class_hash = match cainome::cairo_serde::ClassHash::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not deserialize field {} for {}: {:?}",
                        "class_hash", "Upgraded", e
                    ))
                }
            };
            data_offset += cainome::cairo_serde::ClassHash::cairo_serialized_size(&class_hash);
            return Ok(UpgradeEvent::Upgraded(Upgraded { class_hash }));
        };
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum Src5ComponentEvent {}
impl cainome::cairo_serde::CairoSerde for Src5ComponentEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            _ => {
                return Err(cainome::cairo_serde::Error::Deserialize(format!(
                    "Index not handle for enum {}",
                    "Src5ComponentEvent"
                )))
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for Src5ComponentEvent {
    type Error = String;
    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        Err(format!(
            "Could not match any event from keys {:?}",
            event.keys
        ))
    }
}
impl<A: starknet::accounts::ConnectedAccount + Sync> Controller<A> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_owner(
        &self,
        owner_guid: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(owner_guid));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn assert_valid_owner_signature(
        &self,
        signer_signature: &SignerSignature,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, ()> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(SignerSignature::cairo_serialize(signer_signature));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("assert_valid_owner_signature"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_external_owner(
        &self,
        external_owner_address: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            external_owner_address,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_external_owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_session_revoked(
        &self,
        session_hash: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(session_hash));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_session_revoked"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_valid_outside_execution_nonce(
        &self,
        nonce: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(nonce));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_valid_outside_execution_nonce"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_outside_execution_message_hash_rev_0(
        &self,
        outside_execution: &OutsideExecution,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::Felt> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(OutsideExecution::cairo_serialize(outside_execution));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!(
                "get_outside_execution_message_hash_rev_0"
            ),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_outside_execution_message_hash_rev_1(
        &self,
        outside_execution: &OutsideExecution,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::Felt> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(OutsideExecution::cairo_serialize(outside_execution));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!(
                "get_outside_execution_message_hash_rev_1"
            ),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn delegate_account(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, cainome::cairo_serde::ContractAddress> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("delegate_account"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn supports_interface(
        &self,
        interface_id: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(interface_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("supports_interface"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_valid_signature(
        &self,
        hash: &starknet::core::types::Felt,
        signature: &Vec<starknet::core::types::Felt>,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, starknet::core::types::Felt> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(hash));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            signature,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_valid_signature"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn add_owner_getcall(
        &self,
        owner: &Signer,
        signature: &SignerSignature,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Signer::cairo_serialize(owner));
        __calldata.extend(SignerSignature::cairo_serialize(signature));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("add_owner"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn add_owner(
        &self,
        owner: &Signer,
        signature: &SignerSignature,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Signer::cairo_serialize(owner));
        __calldata.extend(SignerSignature::cairo_serialize(signature));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("add_owner"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn remove_owner_getcall(&self, owner: &Signer) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Signer::cairo_serialize(owner));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("remove_owner"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn remove_owner(&self, owner: &Signer) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Signer::cairo_serialize(owner));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("remove_owner"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade_getcall(
        &self,
        new_class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            new_class_hash,
        ));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn upgrade(
        &self,
        new_class_hash: &cainome::cairo_serde::ClassHash,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ClassHash::cairo_serialize(
            new_class_hash,
        ));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("upgrade"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_external_owner_getcall(
        &self,
        external_owner_address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            external_owner_address,
        ));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_external_owner"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_external_owner(
        &self,
        external_owner_address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            external_owner_address,
        ));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_external_owner"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn remove_external_owner_getcall(
        &self,
        external_owner_address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            external_owner_address,
        ));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("remove_external_owner"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn remove_external_owner(
        &self,
        external_owner_address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            external_owner_address,
        ));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("remove_external_owner"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn revoke_session_getcall(
        &self,
        session_hash: &starknet::core::types::Felt,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(session_hash));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("revoke_session"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn revoke_session(
        &self,
        session_hash: &starknet::core::types::Felt,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(session_hash));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("revoke_session"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_session_getcall(
        &self,
        session: &Session,
        guid_or_address: &starknet::core::types::Felt,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Session::cairo_serialize(session));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(
            guid_or_address,
        ));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_session"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn register_session(
        &self,
        session: &Session,
        guid_or_address: &starknet::core::types::Felt,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Session::cairo_serialize(session));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(
            guid_or_address,
        ));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("register_session"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn execute_from_outside_getcall(
        &self,
        outside_execution: &OutsideExecution,
        signature: &Vec<starknet::core::types::Felt>,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(OutsideExecution::cairo_serialize(outside_execution));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            signature,
        ));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("execute_from_outside"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn execute_from_outside(
        &self,
        outside_execution: &OutsideExecution,
        signature: &Vec<starknet::core::types::Felt>,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(OutsideExecution::cairo_serialize(outside_execution));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            signature,
        ));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("execute_from_outside"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn execute_from_outside_v2_getcall(
        &self,
        outside_execution: &OutsideExecution,
        signature: &Vec<starknet::core::types::Felt>,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(OutsideExecution::cairo_serialize(outside_execution));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            signature,
        ));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("execute_from_outside_v2"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn execute_from_outside_v2(
        &self,
        outside_execution: &OutsideExecution,
        signature: &Vec<starknet::core::types::Felt>,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(OutsideExecution::cairo_serialize(outside_execution));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            signature,
        ));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("execute_from_outside_v2"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_delegate_account_getcall(
        &self,
        delegate_address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            delegate_address,
        ));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_delegate_account"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_delegate_account(
        &self,
        delegate_address: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            delegate_address,
        ));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_delegate_account"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate_declare___getcall(
        &self,
        class_hash: &starknet::core::types::Felt,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(class_hash));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("__validate_declare__"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate_declare__(
        &self,
        class_hash: &starknet::core::types::Felt,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(class_hash));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("__validate_declare__"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate_deploy___getcall(
        &self,
        class_hash: &starknet::core::types::Felt,
        contract_address_salt: &starknet::core::types::Felt,
        owner: &Signer,
        guardian: &Option<Signer>,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(class_hash));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(
            contract_address_salt,
        ));
        __calldata.extend(Signer::cairo_serialize(owner));
        __calldata.extend(Option::<Signer>::cairo_serialize(guardian));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("__validate_deploy__"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate_deploy__(
        &self,
        class_hash: &starknet::core::types::Felt,
        contract_address_salt: &starknet::core::types::Felt,
        owner: &Signer,
        guardian: &Option<Signer>,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(class_hash));
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(
            contract_address_salt,
        ));
        __calldata.extend(Signer::cairo_serialize(owner));
        __calldata.extend(Option::<Signer>::cairo_serialize(guardian));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("__validate_deploy__"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate___getcall(&self, calls: &Vec<Call>) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("__validate__"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __validate__(&self, calls: &Vec<Call>) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("__validate__"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __execute___getcall(&self, calls: &Vec<Call>) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("__execute__"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn __execute__(&self, calls: &Vec<Call>) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(Vec::<Call>::cairo_serialize(calls));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("__execute__"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
}
impl<P: starknet::providers::Provider + Sync> ControllerReader<P> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_owner(
        &self,
        owner_guid: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(owner_guid));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn assert_valid_owner_signature(
        &self,
        signer_signature: &SignerSignature,
    ) -> cainome::cairo_serde::call::FCall<P, ()> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(SignerSignature::cairo_serialize(signer_signature));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("assert_valid_owner_signature"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_external_owner(
        &self,
        external_owner_address: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ContractAddress::cairo_serialize(
            external_owner_address,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_external_owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_session_revoked(
        &self,
        session_hash: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(session_hash));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_session_revoked"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_valid_outside_execution_nonce(
        &self,
        nonce: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(nonce));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_valid_outside_execution_nonce"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_outside_execution_message_hash_rev_0(
        &self,
        outside_execution: &OutsideExecution,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::Felt> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(OutsideExecution::cairo_serialize(outside_execution));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!(
                "get_outside_execution_message_hash_rev_0"
            ),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_outside_execution_message_hash_rev_1(
        &self,
        outside_execution: &OutsideExecution,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::Felt> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(OutsideExecution::cairo_serialize(outside_execution));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!(
                "get_outside_execution_message_hash_rev_1"
            ),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn delegate_account(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::ContractAddress> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("delegate_account"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn supports_interface(
        &self,
        interface_id: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(interface_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("supports_interface"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn is_valid_signature(
        &self,
        hash: &starknet::core::types::Felt,
        signature: &Vec<starknet::core::types::Felt>,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::Felt> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(hash));
        __calldata.extend(Vec::<starknet::core::types::Felt>::cairo_serialize(
            signature,
        ));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("is_valid_signature"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
}
