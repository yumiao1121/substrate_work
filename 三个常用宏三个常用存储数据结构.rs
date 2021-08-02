substrate常见的宏:
1.pallet::config
pub trait Config: 'static + Eq + Clone {
    type BaseCallFilter: Filter<Self::Call>;
    type BlockWeights: Get<BlockWeights>;
    type BlockLength: Get<BlockLength>;
    type Origin: Into<Result<RawOrigin<Self::AccountId>, Self::Origin>> + From<RawOrigin<Self::AccountId>> + Clone + OriginTrait<Call = Self::Call>;
    type Call: Dispatchable + Debug;
    type Index: Parameter + Member + MaybeSerializeDeserialize + Debug + Default + MaybeDisplay + AtLeast32Bit + Copy;
    type BlockNumber: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + AtLeast32BitUnsigned + Default + Bounded + Copy + Hash + FromStr + MaybeMallocSizeOf + MaxEncodedLen;
    type Hash: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + SimpleBitOps + Ord + Default + Copy + CheckEqual + Hash + AsRef<[u8]> + AsMut<[u8]> + MaybeMallocSizeOf + MaxEncodedLen;
    type Hashing: Hash<Output = Self::Hash>;
    type AccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + Ord + Default + MaxEncodedLen;
    type Lookup: StaticLookup<Target = Self::AccountId>;
    type Header: Parameter + Header<Number = Self::BlockNumber, Hash = Self::Hash>;
    type Event: Parameter + Member + From<Event<Self>> + Debug + IsType<Self::Event>;
    type BlockHashCount: Get<Self::BlockNumber>;
    type DbWeight: Get<RuntimeDbWeight>;
    type Version: Get<RuntimeVersion>;
    type PalletInfo: PalletInfo;
    type AccountData: Member + FullCodec + Clone + Default;
    type OnNewAccount: OnNewAccount<Self::AccountId>;
    type OnKilledAccount: OnKilledAccount<Self::AccountId>;
    type SystemWeightInfo: WeightInfo;
    type SS58Prefix: Get<u16>;
    type OnSetCode: SetCode;
}

2.pallet::event

pub enum Event<T: Config> {
    ExtrinsicSuccess(DispatchInfo),
    ExtrinsicFailed(DispatchError, DispatchInfo),
    CodeUpdated,
    NewAccount(T::AccountId),
    KilledAccount(T::AccountId),
    Remarked(T::AccountId, T::Hash),
    // some variants omitted
}

3.pallet::error

pub enum Error<T> {
    InvalidSpecName,
    SpecVersionNeedsToIncrease,
    FailedToExtractRuntimeVersion,
    NonDefaultComposite,
    NonZeroRefCount,
    // some variants omitted
}
3个常用的存储数据结构：
1.单值 StorageValue
pub struct StorageValue<Prefix, Value, QueryKind = OptionQuery, OnEmpty = GetDefault>(_);
2.映射 StorageMap
pub struct StorageMap<Prefix, Hasher, Key, Value, QueryKind = OptionQuery, OnEmpty = GetDefault, MaxValues = GetDefault>(_);
3.双键映射 StorageDoubleMap
pub struct StorageDoubleMap<Prefix, Hasher1, Key1, Hasher2, Key2, Value, QueryKind = OptionQuery, OnEmpty = GetDefault, MaxValues = GetDefault>(_);
