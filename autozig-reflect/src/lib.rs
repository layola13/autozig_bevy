use autozig::include_zig;
use std::any::{Any, TypeId};
use std::fmt;

// ============================================================================
// Core Reflection Types - 基础反射类型 (10个)
// ============================================================================

/// Type information for reflected types
#[derive(Debug, Clone, PartialEq)]
pub enum TypeInfo {
    Array(ArrayInfo),
    List(ListInfo),
    Map(MapInfo),
    Set(SetInfo),
    Struct(StructInfo),
    Tuple(TupleInfo),
    TupleStruct(TupleStructInfo),
    Enum(EnumInfo),
    Opaque(OpaqueInfo),
    Remote(RemoteInfo),
}

/// Type path information for reflecting type names
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypePath {
    pub path: *const u8,
    pub len: usize,
}

unsafe impl Send for TypePath {}
unsafe impl Sync for TypePath {}

/// Registration of a type in the type registry
#[repr(C)]
pub struct TypeRegistration {
    type_id: TypeId,
    type_info: *const TypeInfo,
    type_path: TypePath,
}

unsafe impl Send for TypeRegistration {}
unsafe impl Sync for TypeRegistration {}

/// Global type registry for reflected types
pub struct TypeRegistry {
    registrations: std::collections::HashMap<TypeId, TypeRegistration>,
}

/// Thread-safe reference to TypeRegistry
pub struct TypeRegistryArc {
    inner: std::sync::Arc<std::sync::RwLock<TypeRegistry>>,
}

/// Reference to a reflected value
#[derive(Debug)]
pub enum ReflectRef<'a> {
    Struct(&'a dyn Struct),
    TupleStruct(&'a dyn TupleStruct),
    Tuple(&'a dyn Tuple),
    List(&'a dyn List),
    Array(&'a dyn Array),
    Map(&'a dyn Map),
    Set(&'a dyn Set),
    Enum(&'a dyn Enum),
    Opaque(&'a dyn PartialReflect),
}

/// Mutable reference to a reflected value
#[derive(Debug)]
pub enum ReflectMut<'a> {
    Struct(&'a mut dyn Struct),
    TupleStruct(&'a mut dyn TupleStruct),
    Tuple(&'a mut dyn Tuple),
    List(&'a mut dyn List),
    Array(&'a mut dyn Array),
    Map(&'a mut dyn Map),
    Set(&'a mut dyn Set),
    Enum(&'a mut dyn Enum),
    Opaque(&'a mut dyn PartialReflect),
}

/// Owned reflected value
#[derive(Debug)]
pub enum ReflectOwned {
    Struct(Box<dyn Struct>),
    TupleStruct(Box<dyn TupleStruct>),
    Tuple(Box<dyn Tuple>),
    List(Box<dyn List>),
    Array(Box<dyn Array>),
    Map(Box<dyn Map>),
    Set(Box<dyn Set>),
    Enum(Box<dyn Enum>),
    Opaque(Box<dyn PartialReflect>),
}

/// Kind of reflected type
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReflectKind {
    Struct,
    TupleStruct,
    Tuple,
    List,
    Array,
    Map,
    Set,
    Enum,
    Opaque,
}

// ============================================================================
// Type Information Structures - 类型信息 (11个)
// ============================================================================

/// Array type information
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInfo {
    pub item_type_path: TypePath,
    pub item_type_id: TypeId,
    pub capacity: usize,
}

/// Enum type information
#[derive(Debug, Clone, PartialEq)]
pub struct EnumInfo {
    pub type_path: TypePath,
    pub type_id: TypeId,
    pub variants: Vec<VariantInfo>,
}

/// List type information
#[derive(Debug, Clone, PartialEq)]
pub struct ListInfo {
    pub item_type_path: TypePath,
    pub item_type_id: TypeId,
}

/// Map type information
#[derive(Debug, Clone, PartialEq)]
pub struct MapInfo {
    pub key_type_path: TypePath,
    pub key_type_id: TypeId,
    pub value_type_path: TypePath,
    pub value_type_id: TypeId,
}

/// Opaque type information (no introspection)
#[derive(Debug, Clone, PartialEq)]
pub struct OpaqueInfo {
    pub type_path: TypePath,
    pub type_id: TypeId,
}

/// Set type information
#[derive(Debug, Clone, PartialEq)]
pub struct SetInfo {
    pub item_type_path: TypePath,
    pub item_type_id: TypeId,
}

/// Struct type information
#[derive(Debug, Clone, PartialEq)]
pub struct StructInfo {
    pub type_path: TypePath,
    pub type_id: TypeId,
    pub fields: Vec<NamedField>,
}

/// Tuple type information
#[derive(Debug, Clone, PartialEq)]
pub struct TupleInfo {
    pub type_path: TypePath,
    pub type_id: TypeId,
    pub fields: Vec<UnnamedField>,
}

/// Tuple struct type information
#[derive(Debug, Clone, PartialEq)]
pub struct TupleStructInfo {
    pub type_path: TypePath,
    pub type_id: TypeId,
    pub fields: Vec<UnnamedField>,
}

/// Value information (for enum variants)
#[derive(Debug, Clone, PartialEq)]
pub struct ValueInfo {
    pub name: String,
}

/// Variant information for enums
#[derive(Debug, Clone, PartialEq)]
pub enum VariantInfo {
    Struct(StructVariantInfo),
    Tuple(TupleVariantInfo),
    Unit(UnitVariantInfo),
}

// ============================================================================
// Field and Variant Types - 字段和变体 (6个)
// ============================================================================

/// Named field in a struct
#[derive(Debug, Clone, PartialEq)]
pub struct NamedField {
    pub name: String,
    pub type_path: TypePath,
    pub type_id: TypeId,
}

/// Unnamed field in a tuple
#[derive(Debug, Clone, PartialEq)]
pub struct UnnamedField {
    pub index: usize,
    pub type_path: TypePath,
    pub type_id: TypeId,
}

/// Enum variant info wrapper
#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariantInfo {
    pub name: String,
    pub variant: VariantInfo,
}

/// Struct variant in an enum
#[derive(Debug, Clone, PartialEq)]
pub struct StructVariantInfo {
    pub name: String,
    pub fields: Vec<NamedField>,
}

/// Tuple variant in an enum
#[derive(Debug, Clone, PartialEq)]
pub struct TupleVariantInfo {
    pub name: String,
    pub fields: Vec<UnnamedField>,
}

/// Unit variant in an enum
#[derive(Debug, Clone, PartialEq)]
pub struct UnitVariantInfo {
    pub name: String,
}

/// Variant type discriminant
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariantType {
    Struct,
    Tuple,
    Unit,
}

// ============================================================================
// Path Access Types - 路径访问 (5个)
// ============================================================================

/// Parsed path for accessing nested fields
pub struct ParsedPath {
    segments: Vec<PathSegment>,
}

/// Segment of a reflection path
enum PathSegment {
    Field(String),
    Index(usize),
    Key(Box<dyn PartialReflect>),
}

/// Trait for path-based reflection access
pub trait ReflectPath {
    fn path<'r, 'p>(&'r self, path: &'p str) -> Result<&'r dyn PartialReflect, ReflectPathError>;
    fn path_mut<'r, 'p>(&'r mut self, path: &'p str) -> Result<&'r mut dyn PartialReflect, ReflectPathError>;
}

/// Error accessing a reflection path
#[derive(Debug, Clone, PartialEq)]
pub enum AccessError {
    MissingField(String),
    IndexOutOfBounds { index: usize, len: usize },
    MissingKey(String),
    TypeMismatch,
}

/// Offset-based access helper
#[repr(C)]
pub struct OffsetAccess {
    offset: usize,
    type_id: TypeId,
}

/// Error during path parsing or traversal
#[derive(Debug, Clone, PartialEq)]
pub enum ReflectPathError {
    ParseError(String),
    AccessError(AccessError),
}

// ============================================================================
// Utility Types - 工具类型 (9个)
// ============================================================================

/// Error applying a reflected value
#[derive(Debug, Clone, PartialEq)]
pub enum ApplyError {
    TypeMismatch {
        expected: TypeId,
        found: TypeId,
    },
    MismatchedKinds {
        expected: ReflectKind,
        found: ReflectKind,
    },
}

/// Error converting from a reflected value
#[derive(Debug, Clone, PartialEq)]
pub enum FromReflectError {
    TypeMismatch,
    MissingField(String),
    InvalidVariant(String),
}

/// Trait for getting type registration
pub trait GetTypeRegistration {
    fn get_type_registration() -> TypeRegistration;
}

/// Error registering a type
#[derive(Debug, Clone, PartialEq)]
pub enum RegistrationError {
    AlreadyRegistered(TypeId),
    MissingTypeInfo(TypeId),
}

/// Trait for deserializing reflected types
pub trait ReflectDeserialize {
    fn deserialize(data: &[u8]) -> Result<Box<dyn PartialReflect>, Box<dyn std::error::Error>>;
}

/// Trait for creating reflected values from raw pointers
pub trait ReflectFromPtr {
    unsafe fn as_reflect_ptr<'a>(ptr: *const ()) -> &'a dyn Reflect;
    unsafe fn as_reflect_ptr_mut<'a>(ptr: *mut ()) -> &'a mut dyn Reflect;
}

/// Trait for FromReflect functionality
pub trait ReflectFromReflect {
    fn from_reflect(reflect: &dyn PartialReflect) -> Option<Box<dyn Reflect>>;
}

/// Trait for serializing reflected types
pub trait ReflectSerialize {
    fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

// ============================================================================
// Function Reflection Types - 函数反射 (6个)
// ============================================================================

/// Dynamic function that can be called with reflected arguments
pub struct DynamicFunction {
    name: String,
    info: FunctionInfo,
    func: Box<dyn Fn(&[Box<dyn PartialReflect>]) -> FunctionResult>,
}

/// Information about a function signature
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub signature: SignatureInfo,
}

/// Error when function overloading fails
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionOverloadError {
    ArgumentCountMismatch { expected: usize, found: usize },
    ArgumentTypeMismatch { index: usize, expected: TypeId, found: TypeId },
}

/// Result of a function call
pub type FunctionResult = Result<Box<dyn PartialReflect>, Box<dyn std::error::Error>>;

/// Information about function return value
#[derive(Debug, Clone)]
pub struct ReturnInfo {
    pub type_path: TypePath,
    pub type_id: TypeId,
}

/// Information about function signature
#[derive(Debug, Clone)]
pub struct SignatureInfo {
    pub params: Vec<ParamInfo>,
    pub return_info: Option<ReturnInfo>,
}

/// Parameter information
#[derive(Debug, Clone)]
pub struct ParamInfo {
    pub name: String,
    pub type_path: TypePath,
    pub type_id: TypeId,
}

// ============================================================================
// Remote Type Support - 远程类型 (2个)
// ============================================================================

/// Trait for remote type wrappers
pub trait ReflectRemote {
    fn into_remote(self) -> Box<dyn PartialReflect>;
    fn as_remote(&self) -> &dyn PartialReflect;
    fn as_remote_mut(&mut self) -> &mut dyn PartialReflect;
}

/// Information about remote types
#[derive(Debug, Clone, PartialEq)]
pub struct RemoteInfo {
    pub type_path: TypePath,
    pub type_id: TypeId,
}

// ============================================================================
// Dynamic Types - 动态类型 (11个)
// ============================================================================

/// Dynamic array that can hold reflected values
pub struct DynamicArray {
    represented_type: Option<TypePath>,
    values: Vec<Box<dyn PartialReflect>>,
}

/// Dynamic enum that can represent any enum variant
pub struct DynamicEnum {
    represented_type: Option<TypePath>,
    variant_name: String,
    variant: DynamicVariant,
}

/// Dynamic list that can hold reflected values
pub struct DynamicList {
    represented_type: Option<TypePath>,
    values: Vec<Box<dyn PartialReflect>>,
}

/// Dynamic map that can hold reflected key-value pairs
pub struct DynamicMap {
    represented_type: Option<TypePath>,
    values: std::collections::HashMap<usize, (Box<dyn PartialReflect>, Box<dyn PartialReflect>)>,
}

/// Dynamic set that can hold reflected values
pub struct DynamicSet {
    represented_type: Option<TypePath>,
    values: Vec<Box<dyn PartialReflect>>,
}

/// Dynamic struct that can hold named fields
pub struct DynamicStruct {
    pub represented_type: Option<TypePath>,
    pub fields: std::collections::HashMap<String, Box<dyn PartialReflect>>,
}

/// Dynamic tuple that can hold unnamed fields
pub struct DynamicTuple {
    represented_type: Option<TypePath>,
    fields: Vec<Box<dyn PartialReflect>>,
}

/// Dynamic tuple struct
pub struct DynamicTupleStruct {
    represented_type: Option<TypePath>,
    fields: Vec<Box<dyn PartialReflect>>,
}

/// Dynamic enum variant
pub enum DynamicVariant {
    Unit,
    Tuple(DynamicTuple),
    Struct(DynamicStruct),
}

/// Dynamic struct variant for enums
pub struct DynamicStructVariant {
    name: String,
    fields: std::collections::HashMap<String, Box<dyn PartialReflect>>,
}

/// Dynamic tuple variant for enums
pub struct DynamicTupleVariant {
    name: String,
    fields: Vec<Box<dyn PartialReflect>>,
}

// ============================================================================
// Core Traits - 核心trait (36个)
// ============================================================================

/// Main reflection trait
pub trait Reflect: PartialReflect + Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn as_reflect(&self) -> &dyn Reflect;
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect;
    fn clone_value(&self) -> Box<dyn Reflect>;
    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>>;
}

/// Partial reflection trait (base trait for all reflected types)
pub trait PartialReflect: fmt::Debug + Send + Sync {
    fn get_represented_type_info(&self) -> Option<&'static TypeInfo>;
    fn into_partial_reflect(self: Box<Self>) -> Box<dyn PartialReflect>;
    fn as_partial_reflect(&self) -> &dyn PartialReflect;
    fn as_partial_reflect_mut(&mut self) -> &mut dyn PartialReflect;
    fn try_into_reflect(self: Box<Self>) -> Result<Box<dyn Reflect>, Box<dyn PartialReflect>>;
    fn try_as_reflect(&self) -> Option<&dyn Reflect>;
    fn try_as_reflect_mut(&mut self) -> Option<&mut dyn Reflect>;
    fn try_apply(&mut self, value: &dyn PartialReflect) -> Result<(), ApplyError>;
    fn reflect_kind(&self) -> ReflectKind;
    fn reflect_ref(&self) -> ReflectRef;
    fn reflect_mut(&mut self) -> ReflectMut;
    fn reflect_owned(self: Box<Self>) -> ReflectOwned;
    fn clone_value(&self) -> Box<dyn PartialReflect>;
}

/// Trait for creating types from reflection
pub trait FromReflect: Reflect {
    fn from_reflect(reflect: &dyn PartialReflect) -> Option<Self> where Self: Sized;
}

/// Trait for types that can provide type information
pub trait Typed: Reflect {
    fn type_info() -> &'static TypeInfo;
}

/// Array iterator
pub struct ArrayIter<'a> {
    array: &'a dyn Array,
    index: usize,
}

/// List iterator  
pub struct ListIter<'a> {
    list: &'a dyn List,
    index: usize,
}

/// Map iterator
pub struct MapIter<'a> {
    keys: Vec<&'a dyn PartialReflect>,
    map: &'a dyn Map,
    index: usize,
}

/// Tuple iterator
pub struct TupleIter<'a> {
    tuple: &'a dyn Tuple,
    index: usize,
}

/// Struct iterator
pub struct StructIter<'a> {
    pub fields: Vec<&'a str>,
    pub strukt: &'a dyn Struct,
    pub index: usize,
}

/// Array trait for reflected arrays
pub trait Array: PartialReflect {
    fn get(&self, index: usize) -> Option<&dyn PartialReflect>;
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn PartialReflect>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter(&self) -> ArrayIter;
    fn drain(self: Box<Self>) -> Vec<Box<dyn PartialReflect>>;
}

/// List trait for reflected lists
pub trait List: PartialReflect {
    fn get(&self, index: usize) -> Option<&dyn PartialReflect>;
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn PartialReflect>;
    fn insert(&mut self, index: usize, value: Box<dyn PartialReflect>);
    fn remove(&mut self, index: usize) -> Box<dyn PartialReflect>;
    fn push(&mut self, value: Box<dyn PartialReflect>);
    fn pop(&mut self) -> Option<Box<dyn PartialReflect>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter(&self) -> ListIter;
    fn drain(self: Box<Self>) -> Vec<Box<dyn PartialReflect>>;
}

/// Map trait for reflected maps
pub trait Map: PartialReflect {
    fn get(&self, key: &dyn PartialReflect) -> Option<&dyn PartialReflect>;
    fn get_mut(&mut self, key: &dyn PartialReflect) -> Option<&mut dyn PartialReflect>;
    fn get_at(&self, index: usize) -> Option<(&dyn PartialReflect, &dyn PartialReflect)>;
    fn get_at_mut(&mut self, index: usize) -> Option<(&dyn PartialReflect, &mut dyn PartialReflect)>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter(&self) -> MapIter;
    fn drain(self: Box<Self>) -> Vec<(Box<dyn PartialReflect>, Box<dyn PartialReflect>)>;
    fn clone_dynamic(&self) -> DynamicMap;
    fn insert_boxed(&mut self, key: Box<dyn PartialReflect>, value: Box<dyn PartialReflect>) -> Option<Box<dyn PartialReflect>>;
    fn remove(&mut self, key: &dyn PartialReflect) -> Option<Box<dyn PartialReflect>>;
}

/// Set trait for reflected sets
pub trait Set: PartialReflect {
    fn get(&self, value: &dyn PartialReflect) -> Option<&dyn PartialReflect>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter(&self) -> SetIter;
    fn drain(self: Box<Self>) -> Vec<Box<dyn PartialReflect>>;
    fn clone_dynamic(&self) -> DynamicSet;
    fn insert_boxed(&mut self, value: Box<dyn PartialReflect>) -> bool;
    fn remove(&mut self, value: &dyn PartialReflect) -> bool;
    fn contains(&self, value: &dyn PartialReflect) -> bool;
}

/// Set iterator
pub struct SetIter<'a> {
    set: &'a dyn Set,
    index: usize,
}

/// Enum trait for reflected enums
pub trait Enum: PartialReflect {
    fn field(&self, name: &str) -> Option<&dyn PartialReflect>;
    fn field_mut(&mut self, name: &str) -> Option<&mut dyn PartialReflect>;
    fn field_at(&self, index: usize) -> Option<&dyn PartialReflect>;
    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn PartialReflect>;
    fn index_of(&self, name: &str) -> Option<usize>;
    fn name_at(&self, index: usize) -> Option<&str>;
    fn iter_fields(&self) -> FieldIter;
    fn field_len(&self) -> usize;
    fn variant_name(&self) -> &str;
    fn variant_type(&self) -> VariantType;
    fn clone_dynamic(&self) -> DynamicEnum;
}

/// Field iterator for enums
pub struct FieldIter<'a> {
    enm: &'a dyn Enum,
    index: usize,
}

/// Struct trait for reflected structs
pub trait Struct: PartialReflect {
    fn field(&self, name: &str) -> Option<&dyn PartialReflect>;
    fn field_mut(&mut self, name: &str) -> Option<&mut dyn PartialReflect>;
    fn field_at(&self, index: usize) -> Option<&dyn PartialReflect>;
    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn PartialReflect>;
    fn name_at(&self, index: usize) -> Option<&str>;
    fn field_len(&self) -> usize;
    fn iter_fields(&self) -> StructIter;
    fn clone_dynamic(&self) -> DynamicStruct;
}

/// Tuple trait for reflected tuples
pub trait Tuple: PartialReflect {
    fn field(&self, index: usize) -> Option<&dyn PartialReflect>;
    fn field_mut(&mut self, index: usize) -> Option<&mut dyn PartialReflect>;
    fn field_len(&self) -> usize;
    fn iter_fields(&self) -> TupleIter;
    fn clone_dynamic(&self) -> DynamicTuple;
}

/// TupleStruct trait for reflected tuple structs
pub trait TupleStruct: PartialReflect {
    fn field(&self, index: usize) -> Option<&dyn PartialReflect>;
    fn field_mut(&mut self, index: usize) -> Option<&mut dyn PartialReflect>;
    fn field_len(&self) -> usize;
    fn iter_fields(&self) -> TupleIter;
    fn clone_dynamic(&self) -> DynamicTupleStruct;
}

/// Function trait for callable reflection
pub trait Function: Send + Sync {
    fn name(&self) -> &str;
    fn call(&self, args: &[Box<dyn PartialReflect>]) -> FunctionResult;
    fn clone_dynamic(&self) -> DynamicFunction;
}

/// Trait for converting into functions
pub trait IntoFunction<Marker> {
    fn into_function(self) -> DynamicFunction;
}

/// Trait for converting into mutable functions
pub trait IntoFunctionMut<Marker> {
    fn into_function(self) -> DynamicFunction;
}

/// Trait for return value conversion
pub trait IntoReturn {
    fn into_return(self) -> Box<dyn PartialReflect>;
}

/// Trait for dynamic typed values
pub trait DynamicTyped {
    fn represented_type(&self) -> Option<&TypePath>;
    fn set_represented_type(&mut self, represented_type: Option<TypePath>);
}

/// Trait for dynamic type paths
pub trait DynamicTypePath {
    fn reflect_type_path(&self) -> &str;
    fn reflect_short_type_path(&self) -> &str;
}

// ============================================================================
// Iterator Implementations
// ============================================================================

impl<'a> Iterator for ArrayIter<'a> {
    type Item = &'a dyn PartialReflect;
    
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.array.get(self.index)?;
        self.index += 1;
        Some(item)
    }
}

impl<'a> Iterator for ListIter<'a> {
    type Item = &'a dyn PartialReflect;
    
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.get(self.index)?;
        self.index += 1;
        Some(item)
    }
}

impl<'a> Iterator for TupleIter<'a> {
    type Item = &'a dyn PartialReflect;
    
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.tuple.field(self.index)?;
        self.index += 1;
        Some(item)
    }
}

// ============================================================================
// Error Implementations
// ============================================================================

impl std::error::Error for AccessError {}
impl std::error::Error for ApplyError {}
impl std::error::Error for FromReflectError {}
impl std::error::Error for ReflectPathError {}
impl std::error::Error for RegistrationError {}
impl std::error::Error for FunctionOverloadError {}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessError::MissingField(name) => write!(f, "Missing field: {}", name),
            AccessError::IndexOutOfBounds { index, len } => write!(f, "Index {} out of bounds (len: {})", index, len),
            AccessError::MissingKey(key) => write!(f, "Missing key: {}", key),
            AccessError::TypeMismatch => write!(f, "Type mismatch"),
        }
    }
}

impl fmt::Display for ApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplyError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {:?}, found {:?}", expected, found)
            }
            ApplyError::MismatchedKinds { expected, found } => {
                write!(f, "Mismatched kinds: expected {:?}, found {:?}", expected, found)
            }
        }
    }
}

impl fmt::Display for FromReflectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FromReflectError::TypeMismatch => write!(f, "Type mismatch"),
            FromReflectError::MissingField(name) => write!(f, "Missing field: {}", name),
            FromReflectError::InvalidVariant(name) => write!(f, "Invalid variant: {}", name),
        }
    }
}

impl fmt::Display for ReflectPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReflectPathError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ReflectPathError::AccessError(err) => write!(f, "Access error: {}", err),
        }
    }
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistrationError::AlreadyRegistered(type_id) => write!(f, "Type already registered: {:?}", type_id),
            RegistrationError::MissingTypeInfo(type_id) => write!(f, "Missing type info: {:?}", type_id),
        }
    }
}

impl fmt::Display for FunctionOverloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionOverloadError::ArgumentCountMismatch { expected, found } => {
                write!(f, "Argument count mismatch: expected {}, found {}", expected, found)
            }
            FunctionOverloadError::ArgumentTypeMismatch { index, expected, found } => {
                write!(f, "Argument type mismatch at index {}: expected {:?}, found {:?}", index, expected, found)
            }
        }
    }
}

// ============================================================================
// Include Zig FFI Bindings (Placeholder for now)
// ============================================================================

// Note: Zig implementations will be added in separate files
// include_zig!("zig/reflect_all.zig", {
//     // FFI functions will be defined here
// });

// ============================================================================
// Implementation of TypeRegistry
// ============================================================================

impl Default for TypeRegistry {
    fn default() -> Self {
        Self {
            registrations: std::collections::HashMap::new(),
        }
    }
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T: Typed>(&mut self) {
        let info = T::type_info();
        let registration = TypeRegistration {
            type_id: std::any::TypeId::of::<T>(),
            type_info: info as *const TypeInfo,
            type_path: TypePath {
                path: std::ptr::null(), // TODO: Implement TypePath construction
                len: 0,
            },
        };
        self.registrations.insert(std::any::TypeId::of::<T>(), registration);
    }
}

// ============================================================================
// Implementation of TypeRegistryArc
// ============================================================================

impl Default for TypeRegistryArc {
    fn default() -> Self {
        Self {
            inner: std::sync::Arc::new(std::sync::RwLock::new(TypeRegistry::default())),
        }
    }
}

impl Clone for TypeRegistryArc {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl TypeRegistryArc {
    pub fn read(&self) -> std::sync::RwLockReadGuard<TypeRegistry> {
        self.inner.read().unwrap()
    }

    pub fn write(&self) -> std::sync::RwLockWriteGuard<TypeRegistry> {
        self.inner.write().unwrap()
    }
}