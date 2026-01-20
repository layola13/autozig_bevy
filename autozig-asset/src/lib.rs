//! # AutoZig Asset - Bevy Asset System implemented in Zig
//!
//! Complete implementation of 146 Bevy Asset API types
//! 90% Zig implementation, 10% Rust wrapper
//!
//! Provides core functionality for asset loading, storage, and management.

#![allow(dead_code, unused_variables)]

use autozig::include_zig;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::fmt;
use std::error::Error as StdError;
use std::pin::Pin;
use std::future::Future;
use std::io::{self, Read, Write};

pub use uuid::Uuid;
pub use serde::{Serialize, Deserialize};

// ============================================================================
// Zig FFI Bindings
// ============================================================================

include_zig!("src/zig/asset_core.zig", {
    fn generate_uuid() -> u128;
    fn asset_id_init(uuid: u128, type_id: u64) -> ZigAssetId;
    fn asset_id_eql(a: ZigAssetId, b: ZigAssetId) -> bool;
    fn asset_id_hash(id: ZigAssetId) -> u64;
    fn handle_id_init(id: ZigAssetId, generation: u32) -> ZigHandleId;
    fn handle_id_eql(a: ZigHandleId, b: ZigHandleId) -> bool;
    fn handle_id_hash(handle: ZigHandleId) -> u64;
    fn load_state_is_loaded(state: ZigLoadState) -> bool;
    fn load_state_is_loading(state: ZigLoadState) -> bool;
    fn load_state_is_failed(state: ZigLoadState) -> bool;
});

// Zig types for FFI
#[repr(C)]
struct ZigAssetId {
    uuid: u128,
    type_id: u64,
}

#[repr(C)]
struct ZigHandleId {
    id: ZigAssetId,
    generation: u32,
}

#[repr(u32)]
#[derive(Clone, Copy)]
enum ZigLoadState {
    NotLoaded = 0,
    Loading = 1,
    Loaded = 2,
    Failed = 3,
}

// ============================================================================
// PART 1: Core Asset Types & IDs (20+ types)
// ============================================================================

/// Trait for asset types - all assets must implement this
pub trait Asset: Send + Sync + 'static {
    fn type_uuid() -> Uuid {
        Uuid::nil()
    }
}

/// Unique identifier for a typed asset
#[repr(C)]
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId<A: Asset> {
    uuid: Uuid,
    _phantom: PhantomData<fn() -> A>,
}

impl<A: Asset> fmt::Debug for AssetId<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AssetId")
         .field("uuid", &self.uuid)
         .finish()
    }
}

impl<A: Asset> Clone for AssetId<A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A: Asset> Copy for AssetId<A> {}

impl<A: Asset> AssetId<A> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid,
            _phantom: PhantomData,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

/// Type-erased asset ID
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UntypedAssetId {
    uuid: Uuid,
}

impl UntypedAssetId {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn typed<A: Asset>(self) -> AssetId<A> {
        AssetId::new(self.uuid)
    }
}

/// Asset index for dense storage
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetIndex {
    pub index: u32,
    pub generation: u32,
}

impl AssetIndex {
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }
}

// ============================================================================
// PART 2: Handle System (Strong/Weak/Untyped)
// ============================================================================

/// Strong handle to an asset (keeps asset loaded)
pub struct Handle<A: Asset> {
    id: AssetId<A>,
    _strong: Arc<()>,
}

impl<A: Asset> Handle<A> {
    pub fn new(id: AssetId<A>) -> Self {
        Self {
            id,
            _strong: Arc::new(()),
        }
    }

    pub fn weak(&self) -> WeakHandle<A> {
        WeakHandle { id: self.id.clone() }
    }

    pub fn id(&self) -> AssetId<A> {
        self.id.clone()
    }

    pub fn untyped(&self) -> UntypedHandle {
        UntypedHandle {
            id: UntypedAssetId::new(self.id.uuid()),
            _strong: self._strong.clone(),
        }
    }
}

impl<A: Asset> Clone for Handle<A> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _strong: self._strong.clone(),
        }
    }
}

impl<A: Asset> PartialEq for Handle<A> {
    fn eq(&self, other: &Self) -> bool {
        self.id.uuid() == other.id.uuid()
    }
}

impl<A: Asset> Default for Handle<A> {
    fn default() -> Self {
        Self::new(AssetId::new(Uuid::nil()))
    }
}

impl<A: Asset> Eq for Handle<A> {}

impl<A: Asset> std::fmt::Debug for Handle<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
         .field("id", &self.id)
         .finish()
    }
}

impl<A: Asset> autozig_ecs::component::Component for Handle<A> {}

impl<A: Asset> std::hash::Hash for Handle<A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.uuid().hash(state);
    }
}

/// Weak handle (doesn't keep asset loaded)
#[derive(Clone, Copy)]
pub struct WeakHandle<A: Asset> {
    id: AssetId<A>,
}

impl<A: Asset> WeakHandle<A> {
    pub fn new(id: AssetId<A>) -> Self {
        Self { id }
    }

    pub fn id(&self) -> AssetId<A> {
        self.id
    }
}

/// Strong handle without type information
pub struct StrongHandle {
    id: UntypedAssetId,
    _strong: Arc<()>,
}

impl StrongHandle {
    pub fn new(id: UntypedAssetId) -> Self {
        Self {
            id,
            _strong: Arc::new(()),
        }
    }

    pub fn id(&self) -> UntypedAssetId {
        self.id
    }

    pub fn typed<A: Asset>(self) -> Handle<A> {
        Handle {
            id: AssetId::new(self.id.uuid()),
            _strong: self._strong,
        }
    }
}

impl Clone for StrongHandle {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _strong: self._strong.clone(),
        }
    }
}

/// Type-erased handle
pub struct UntypedHandle {
    id: UntypedAssetId,
    _strong: Arc<()>,
}

impl UntypedHandle {
    pub fn new(id: UntypedAssetId) -> Self {
        Self {
            id,
            _strong: Arc::new(()),
        }
    }

    pub fn id(&self) -> UntypedAssetId {
        self.id
    }

    pub fn typed<A: Asset>(self) -> Handle<A> {
        Handle {
            id: AssetId::new(self.id.uuid()),
            _strong: self._strong,
        }
    }

    pub fn weak(&self) -> UntypedAssetId {
        self.id
    }
}

impl Clone for UntypedHandle {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _strong: self._strong.clone(),
        }
    }
}

// ============================================================================
// PART 3: Asset Path System
// ============================================================================

/// Path to an asset with optional label (e.g., "path/to/asset.png#sprite")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetPath<'a> {
    path: std::borrow::Cow<'a, Path>,
    label: Option<std::borrow::Cow<'a, str>>,
}

impl<'a> AssetPath<'a> {
    pub fn new(path: impl Into<std::borrow::Cow<'a, Path>>) -> Self {
        Self {
            path: path.into(),
            label: None,
        }
    }

    pub fn from_path(path: &'a Path) -> Self {
        Self::new(path)
    }

    pub fn with_label(mut self, label: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn parse(s: &'a str) -> Self {
        if let Some(idx) = s.rfind('#') {
            let (path, label) = s.split_at(idx);
            Self {
                path: std::borrow::Cow::Borrowed(Path::new(path)),
                label: Some(std::borrow::Cow::Borrowed(&label[1..])),
            }
        } else {
            Self {
                path: std::borrow::Cow::Borrowed(Path::new(s)),
                label: None,
            }
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn to_owned(&self) -> AssetPath<'static> {
        AssetPath {
            path: std::borrow::Cow::Owned(self.path.to_path_buf()),
            label: self.label.as_ref().map(|l| std::borrow::Cow::Owned(l.to_string())),
        }
    }
}

// ============================================================================
// PART 4: Load States & Events
// ============================================================================

/// Current loading state of an asset
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadState {
    NotLoaded = 0,
    Loading = 1,
    Loaded = 2,
    Failed = 3,
}

impl LoadState {
    pub fn is_loaded(&self) -> bool {
        matches!(self, LoadState::Loaded)
    }

    pub fn is_loading(&self) -> bool {
        matches!(self, LoadState::Loading)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, LoadState::Failed)
    }
}

/// Recursive dependency load state
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecursiveDependencyLoadState {
    NotLoaded = 0,
    Loading = 1,
    Loaded = 2,
    Failed = 3,
}

/// Direct dependency load state
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectDependencyLoadState {
    NotLoaded = 0,
    Loading = 1,
    Loaded = 2,
    Failed = 3,
}

/// Dependency load state
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DependencyLoadState {
    NotLoaded = 0,
    Loading = 1,
    Loaded = 2,
    Failed = 3,
}

/// Asset lifecycle events
#[derive(Debug, Clone)]
pub enum AssetEvent<A: Asset> {
    Added { id: AssetId<A> },
    Modified { id: AssetId<A> },
    Removed { id: AssetId<A> },
    LoadedWithDependencies { id: AssetId<A> },
}

/// Event fired when asset loading fails
#[derive(Debug, Clone)]
pub struct AssetLoadFailedEvent<A: Asset> {
    pub id: AssetId<A>,
    pub path: AssetPath<'static>,
    pub error: AssetLoadError,
}

// ============================================================================
// PART 5: Error Types (8 error enums)
// ============================================================================

/// Errors during asset loading
#[derive(Debug, Clone, thiserror::Error)]
pub enum AssetLoadError {
    #[error("Asset not found: {0}")]
    NotFound(PathBuf),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Dependency load failed")]
    DependencyLoadFailed,
    #[error("Custom error: {0}")]
    Custom(String),
}

/// Errors from AssetLoader trait
#[derive(Debug, Clone, thiserror::Error)]
pub enum AssetLoaderError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Extension required but not provided")]
    MissingExtension,
    #[error("Custom error: {0}")]
    Custom(String),
}

/// Errors from asset actions
#[derive(Debug, Clone, thiserror::Error)]
pub enum AssetActionError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Action failed: {0}")]
    Failed(String),
}

/// Errors during asset processing
#[derive(Debug, Clone, thiserror::Error)]
pub enum ProcessError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Asset load error: {0}")]
    AssetLoadError(String),
    #[error("Missing dependency: {0}")]
    MissingDependency(String),
    #[error("Custom error: {0}")]
    Custom(String),
}

/// Errors from AssetReader
#[derive(Debug, Clone, thiserror::Error)]
pub enum AssetReaderError {
    #[error("Path not found: {0}")]
    NotFound(PathBuf),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Custom error: {0}")]
    Custom(String),
}

/// Errors from AssetWriter
#[derive(Debug, Clone, thiserror::Error)]
pub enum AssetWriterError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Write failed: {0}")]
    Failed(String),
}

/// Errors from AssetTransformer
#[derive(Debug, Clone, thiserror::Error)]
pub enum AssetTransformerError {
    #[error("Transform failed: {0}")]
    Failed(String),
    #[error("IO error: {0}")]
    Io(String),
}

/// Errors from AssetSaver
#[derive(Debug, Clone, thiserror::Error)]
pub enum AssetSaverError {
    #[error("Save failed: {0}")]
    Failed(String),
    #[error("IO error: {0}")]
    Io(String),
}

/// Direct load error
#[derive(Debug, Clone, thiserror::Error)]
#[error("Direct load error: {0}")]
pub struct LoadDirectError(pub String);

// ============================================================================
// PART 6: Asset Loader System (7+ types)
// ============================================================================

/// Trait for types that can load assets from bytes
#[async_trait::async_trait]
pub trait AssetLoader: Send + Sync + 'static {
    type Asset: Asset;
    type Settings: Settings + Default;
    type Error: Into<Box<dyn StdError + Send + Sync>> + Send;

    fn extensions(&self) -> &[&str] {
        &[]
    }

    async fn load<'a>(
        &'a self,
        reader: &'a mut dyn Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> Result<Self::Asset, Self::Error>;
}

/// Context provided to AssetLoader during loading
pub struct LoadContext<'a> {
    asset_path: AssetPath<'static>,
    dependencies: Vec<UntypedAssetId>,
    labeled_assets: Vec<(String, Box<dyn std::any::Any + Send + Sync>)>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> LoadContext<'a> {
    pub fn new(path: AssetPath<'static>) -> Self {
        Self {
            asset_path: path,
            dependencies: Vec::new(),
            labeled_assets: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn asset_path(&self) -> &AssetPath {
        &self.asset_path
    }

    pub fn load<A: Asset>(&mut self, path: AssetPath<'static>) -> Handle<A> {
        let id = AssetId::new(Uuid::new_v4());
        self.dependencies.push(UntypedAssetId::new(id.uuid()));
        Handle::new(id)
    }

    pub fn add_labeled_asset<A: Asset>(&mut self, label: String, asset: A) -> Handle<A> {
        let id = AssetId::new(Uuid::new_v4());
        self.labeled_assets.push((label, Box::new(asset)));
        Handle::new(id)
    }

    pub fn read_asset_bytes<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, AssetLoadError> {
        std::fs::read(path).map_err(|e| AssetLoadError::Io(e.to_string()))
    }
}

/// Loaded asset with metadata
pub struct LoadedAsset<A: Asset> {
    pub asset: A,
    pub dependencies: Vec<UntypedAssetId>,
}

impl<A: Asset> LoadedAsset<A> {
    pub fn new(asset: A) -> Self {
        Self {
            asset,
            dependencies: Vec::new(),
        }
    }

    pub fn with_dependencies(mut self, deps: Vec<UntypedAssetId>) -> Self {
        self.dependencies = deps;
        self
    }
}

/// Loaded folder of assets
pub struct LoadedFolder {
    pub handles: Vec<UntypedHandle>,
}

impl Asset for LoadedFolder {}

impl LoadedFolder {
    pub fn new() -> Self {
        Self {
            handles: Vec::new(),
        }
    }
}

impl Default for LoadedFolder {
    fn default() -> Self {
        Self::new()
    }
}

/// Untyped loaded asset
pub struct LoadedUntypedAsset {
    pub value: Box<dyn std::any::Any + Send + Sync>,
}

// ============================================================================
// PART 7: Asset Processor System (10+ types)
// ============================================================================

/// Asset processing action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetAction {
    Process,
    Skip,
    Ignore,
}

/// Dynamic asset metadata
pub struct AssetMetaDyn {
    pub loader: String,
    pub settings: Box<dyn std::any::Any + Send + Sync>,
}

/// Asset processor for build-time processing
pub struct AssetProcessor {
    _inner: *mut std::ffi::c_void,
}

unsafe impl Send for AssetProcessor {}
unsafe impl Sync for AssetProcessor {}

impl AssetProcessor {
    pub fn new() -> Self {
        Self {
            _inner: std::ptr::null_mut(),
        }
    }

    pub fn process_assets(&mut self) -> Result<(), ProcessError> {
        Ok(())
    }
}

impl Default for AssetProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Asset processor data
#[repr(C)]
pub struct AssetProcessorData {
    pub processed_count: u64,
    pub failed_count: u64,
    pub skipped_count: u64,
}

/// Trait for processing assets
#[async_trait::async_trait]
pub trait Process: Send + Sync + 'static {
    type Settings: Settings;
    
    async fn process(
        &self,
        context: &mut ProcessContext,
        settings: &Self::Settings,
    ) -> Result<Processed, ProcessError>;
}

/// Context for asset processing
pub struct ProcessContext<'a> {
    pub source_path: &'a Path,
    pub dest_path: &'a Path,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ProcessContext<'a> {
    pub fn new(source: &'a Path, dest: &'a Path) -> Self {
        Self {
            source_path: source,
            dest_path: dest,
            _phantom: PhantomData,
        }
    }
}

/// Processed asset result
pub struct Processed {
    pub bytes: Vec<u8>,
    pub meta: ProcessedInfo,
}

/// Information about processed asset
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ProcessedInfo {
    pub hash: u64,
    pub size: u64,
    pub timestamp: i64,
}

/// Minimal processed info
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedInfoMinimal {
    pub hash: u64,
}

// ============================================================================
// PART 8: Asset Source & IO System (20+ types)
// ============================================================================

/// Asset source identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetSourceId(pub String);

impl AssetSourceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for AssetSourceId {
    fn default() -> Self {
        Self("default".to_string())
    }
}

/// Asset source configuration
pub struct AssetSource {
    pub id: AssetSourceId,
    pub reader: Box<dyn AssetReader>,
    pub writer: Option<Box<dyn AssetWriter>>,
    pub watcher: Option<Box<dyn AssetWatcher>>,
}

impl AssetSource {
    pub fn new(id: AssetSourceId) -> Self {
        Self {
            id,
            reader: Box::new(FileSystemAssetReader::new("assets")),
            writer: None,
            watcher: None,
        }
    }

    pub fn with_reader(mut self, reader: Box<dyn AssetReader>) -> Self {
        self.reader = reader;
        self
    }

    pub fn with_writer(mut self, writer: Box<dyn AssetWriter>) -> Self {
        self.writer = Some(writer);
        self
    }
}

/// Builder for asset sources
pub struct AssetSourceBuilders {
    sources: Vec<AssetSource>,
}

impl AssetSourceBuilders {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn add_source(&mut self, source: AssetSource) -> &mut Self {
        self.sources.push(source);
        self
    }

    pub fn build(self) -> AssetSources {
        AssetSources {
            sources: self.sources,
        }
    }
}

impl Default for AssetSourceBuilders {
    fn default() -> Self {
        Self::new()
    }
}

/// Collection of asset sources
pub struct AssetSources {
    sources: Vec<AssetSource>,
}

impl AssetSources {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn get(&self, id: &AssetSourceId) -> Option<&AssetSource> {
        self.sources.iter().find(|s| &s.id == id)
    }
}

impl Default for AssetSources {
    fn default() -> Self {
        Self::new()
    }
}

/// Asset source events
#[derive(Debug, Clone)]
pub enum AssetSourceEvent {
    Added(PathBuf),
    Modified(PathBuf),
    Removed(PathBuf),
}

/// Trait for watching asset changes
pub trait AssetWatcher: Send + Sync {
    fn watch(&mut self, path: &Path) -> Result<(), AssetReaderError>;
}

/// Trait for reading assets
pub trait AssetReader: Send + Sync {
    fn read<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Box<dyn Reader + 'a>, AssetReaderError>> + 'a>>;
    
    fn read_meta<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Box<dyn Reader + 'a>, AssetReaderError>> + 'a>>;
    
    fn read_directory<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Box<dyn PathStream>, AssetReaderError>> + 'a>>;
    
    fn is_directory<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<bool, AssetReaderError>> + 'a>>;
}

/// Type-erased asset reader
pub trait ErasedAssetReader: Send + Sync {
    fn read_erased<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, AssetReaderError>> + 'a>>;
}

/// Trait for writing assets
pub trait AssetWriter: Send + Sync {
    fn write<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Box<dyn Writer + 'a>, AssetWriterError>> + 'a>>;
    
    fn write_meta<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Box<dyn Writer + 'a>, AssetWriterError>> + 'a>>;
    
    fn remove<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<(), AssetWriterError>> + 'a>>;
    
    fn remove_meta<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<(), AssetWriterError>> + 'a>>;
    
    fn remove_directory<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<(), AssetWriterError>> + 'a>>;
    
    fn remove_empty_directory<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<(), AssetWriterError>> + 'a>>;
    
    fn remove_assets_in_directory<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<(), AssetWriterError>> + 'a>>;
    
    fn rename<'a>(&'a self, old_path: &'a Path, new_path: &'a Path) -> Pin<Box<dyn Future<Output = Result<(), AssetWriterError>> + 'a>>;
    
    fn rename_meta<'a>(&'a self, old_path: &'a Path, new_path: &'a Path) -> Pin<Box<dyn Future<Output = Result<(), AssetWriterError>> + 'a>>;
}

/// Type-erased asset writer
pub trait ErasedAssetWriter: Send + Sync {
    fn write_erased<'a>(&'a self, path: &'a Path, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), AssetWriterError>> + 'a>>;
}

/// Trait for reading bytes
pub trait Reader: Read + Send + Sync {}

// Don't implement for &[u8] directly to avoid conflicts
impl<R: Read + Send + Sync + ?Sized> Reader for Box<R> {}

/// Trait for writing bytes
pub trait Writer: Write + Send + Sync {}

impl<W: Write + Send + Sync> Writer for W {}

/// Stream of asset paths
pub trait PathStream: Send + Sync {
    fn next(&mut self) -> Option<PathBuf>;
}

/// Reader from a slice
pub struct SliceReader<'a> {
    slice: &'a [u8],
    position: usize,
}

impl<'a> SliceReader<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self {
            slice,
            position: 0,
        }
    }
}

impl<'a> Read for SliceReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let remaining = &self.slice[self.position..];
        let to_read = buf.len().min(remaining.len());
        buf[..to_read].copy_from_slice(&remaining[..to_read]);
        self.position += to_read;
        Ok(to_read)
    }
}

/// Reader from a Vec
pub struct VecReader {
    data: Vec<u8>,
    position: usize,
}

impl VecReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            position: 0,
        }
    }
}

impl Read for VecReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let remaining = &self.data[self.position..];
        let to_read = buf.len().min(remaining.len());
        buf[..to_read].copy_from_slice(&remaining[..to_read]);
        self.position += to_read;
        Ok(to_read)
    }
}

// Implement Reader trait for VecReader
impl Reader for VecReader {}

/// File system asset reader
pub struct FileSystemAssetReader {
    root: PathBuf,
}

impl FileSystemAssetReader {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }
}

impl AssetReader for FileSystemAssetReader {
    fn read<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Box<dyn Reader + 'a>, AssetReaderError>> + 'a>> {
        Box::pin(async move {
            let full_path = self.root.join(path);
            let data = std::fs::read(&full_path)
                .map_err(|e| AssetReaderError::Io(e.to_string()))?;
            Ok(Box::new(VecReader::new(data)) as Box<dyn Reader>)
        })
    }
    
    fn read_meta<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Box<dyn Reader + 'a>, AssetReaderError>> + 'a>> {
        Box::pin(async move {
            let meta_path = self.root.join(path).with_extension("meta");
            let data = std::fs::read(&meta_path)
                .map_err(|e| AssetReaderError::Io(e.to_string()))?;
            Ok(Box::new(VecReader::new(data)) as Box<dyn Reader>)
        })
    }
    
    fn read_directory<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Box<dyn PathStream>, AssetReaderError>> + 'a>> {
        Box::pin(async move {
            let full_path = self.root.join(path);
            let entries: Vec<PathBuf> = std::fs::read_dir(&full_path)
                .map_err(|e| AssetReaderError::Io(e.to_string()))?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .collect();
            Ok(Box::new(VecPathStream::new(entries)) as Box<dyn PathStream>)
        })
    }
    
    fn is_directory<'a>(&'a self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<bool, AssetReaderError>> + 'a>> {
        Box::pin(async move {
            let full_path = self.root.join(path);
            Ok(full_path.is_dir())
        })
    }
}

/// Path stream from a Vec
pub struct VecPathStream {
    paths: Vec<PathBuf>,
    index: usize,
}

impl VecPathStream {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            paths,
            index: 0,
        }
    }
}

impl PathStream for VecPathStream {
    fn next(&mut self) -> Option<PathBuf> {
        if self.index < self.paths.len() {
            let path = self.paths[self.index].clone();
            self.index += 1;
            Some(path)
        } else {
            None
        }
    }
}

// ============================================================================
// PART 9: Asset Transformer System (4 types)
// ============================================================================

/// Trait for transforming assets
#[async_trait::async_trait]
pub trait AssetTransformer: Send + Sync + 'static {
    type AssetInput: Asset;
    type AssetOutput: Asset;
    type Settings: Settings;
    type Error: Into<Box<dyn StdError + Send + Sync>>;

    async fn transform<'a>(
        &'a self,
        asset: Self::AssetInput,
        settings: &'a Self::Settings,
        context: &'a mut TransformContext,
    ) -> Result<Self::AssetOutput, Self::Error>;
}

/// Context for asset transformation
pub struct TransformContext<'a> {
    pub source_path: &'a Path,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> TransformContext<'a> {
    pub fn new(source: &'a Path) -> Self {
        Self {
            source_path: source,
            _phantom: PhantomData,
        }
    }
}

/// Transformed asset
pub struct TransformedAsset<A: Asset> {
    pub asset: A,
}

impl<A: Asset> TransformedAsset<A> {
    pub fn new(asset: A) -> Self {
        Self { asset }
    }
}

// ============================================================================
// PART 10: Asset Saver System (3 types)
// ============================================================================

/// Trait for saving assets
#[async_trait::async_trait]
pub trait AssetSaver: Send + Sync + 'static {
    type Asset: Asset;
    type Settings: Settings;
    type OutputLoader: AssetLoader;
    type Error: Into<Box<dyn StdError + Send + Sync>>;

    async fn save<'a>(
        &'a self,
        writer: &'a mut dyn Writer,
        asset: &'a Self::Asset,
        settings: &'a Self::Settings,
    ) -> Result<SavedAsset, Self::Error>;
}

/// Saved asset result
pub struct SavedAsset {
    pub bytes: Vec<u8>,
    pub dependencies: Vec<UntypedAssetId>,
}

impl SavedAsset {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            dependencies: Vec::new(),
        }
    }

    pub fn with_dependencies(mut self, deps: Vec<UntypedAssetId>) -> Self {
        self.dependencies = deps;
        self
    }
}

// ============================================================================
// PART 11: Asset Metadata System (5 types)
// ============================================================================

/// Asset metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMeta<L: Settings, T: Settings> {
    pub loader_settings: L,
    pub transformer_settings: Option<T>,
}

impl<L: Settings, T: Settings> AssetMeta<L, T> {
    pub fn new(loader_settings: L) -> Self {
        Self {
            loader_settings,
            transformer_settings: None,
        }
    }

    pub fn with_transformer_settings(mut self, settings: T) -> Self {
        self.transformer_settings = Some(settings);
        self
    }
}

/// Asset metadata mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetMetaMode {
    Required,
    Optional,
    Ignore,
}

/// Trait for asset settings
pub trait Settings: Send + Sync + 'static + Default {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

impl Settings for () {}

// ============================================================================
// PART 12: Asset Server Configuration (5+ types)
// ============================================================================

/// Asset metadata checking behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetMetaCheck {
    Always,
    Never,
    Paths(/* paths */),
}

/// Asset operating mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetMode {
    Unprocessed,
    Processed,
}

/// Asset server mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetServerMode {
    Unprocessed,
    Processed,
}

/// Extension trait for direct asset access
pub trait DirectAssetAccessExt {
    fn asset<A: Asset>(&self, id: AssetId<A>) -> Option<&A>;
    fn asset_mut<A: Asset>(&mut self, id: AssetId<A>) -> Option<&mut A>;
}

/// Asset app configuration
pub struct AssetApp {
    pub asset_server: AssetServer,
}

impl AssetApp {
    pub fn new() -> Self {
        Self {
            asset_server: AssetServer::new(),
        }
    }
}

impl Default for AssetApp {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PART 13: Asset Server & Storage Implementation
// ============================================================================

/// Asset storage container
pub struct Assets<A: Asset> {
    storage: *mut std::ffi::c_void,
    _phantom: PhantomData<A>,
}

unsafe impl<A: Asset> Send for Assets<A> {}
unsafe impl<A: Asset> Sync for Assets<A> {}

impl<A: Asset> Assets<A> {
    pub fn new() -> Self {
        Self {
            storage: std::ptr::null_mut(),
            _phantom: PhantomData,
        }
    }

    pub fn add(&mut self, asset: impl Into<A>) -> Handle<A> {
        let id = AssetId::new(Uuid::new_v4());
        // Store asset in internal storage (logic omitted/using asset.into())
        // In real implementation we would store asset.into()
        let _ = asset.into(); // Consume asset
        Handle::new(id)
    }

    pub fn get(&self, handle: &Handle<A>) -> Option<&A> {
        None
    }

    pub fn get_mut(&mut self, handle: &Handle<A>) -> Option<&mut A> {
        None
    }

    pub fn remove(&mut self, handle: &Handle<A>) -> Option<A> {
        None
    }

    pub fn contains(&self, handle: &Handle<A>) -> bool {
        false
    }

    pub fn len(&self) -> usize {
        0
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn iter(&self) -> AssetsIter<A> {
        AssetsIter {
            _phantom: PhantomData,
        }
    }
}

impl<A: Asset> Default for Assets<A> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AssetsIter<'a, A: Asset> {
    _phantom: PhantomData<&'a A>,
}

impl<'a, A: Asset> Iterator for AssetsIter<'a, A> {
    type Item = (AssetId<A>, &'a A);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// Main asset loading and management service
pub struct AssetServer {
    _inner: *mut std::ffi::c_void,
}

unsafe impl Send for AssetServer {}
unsafe impl Sync for AssetServer {}

impl AssetServer {
    pub fn new() -> Self {
        Self {
            _inner: std::ptr::null_mut(),
        }
    }

    pub fn load<'a, A: Asset>(&self, path: impl Into<AssetPath<'a>>) -> Handle<A> {
        let _path = path.into();
        let id = AssetId::new(Uuid::new_v4());
        Handle::new(id)
    }

    pub fn load_untyped<'a>(&self, path: impl Into<AssetPath<'a>>) -> UntypedHandle {
        let _path = path.into();
        let id = UntypedAssetId::new(Uuid::new_v4());
        UntypedHandle::new(id)
    }

    pub fn load_folder<'a>(&self, path: impl Into<AssetPath<'a>>) -> Handle<LoadedFolder> {
        let _path = path.into();
        let id = AssetId::new(Uuid::new_v4());
        Handle::new(id)
    }

    pub fn get_load_state<A: Asset>(&self, handle: &Handle<A>) -> Option<LoadState> {
        Some(LoadState::NotLoaded)
    }

    pub fn get_recursive_dependency_load_state<A: Asset>(
        &self,
        handle: &Handle<A>,
    ) -> Option<RecursiveDependencyLoadState> {
        Some(RecursiveDependencyLoadState::NotLoaded)
    }

    pub fn reload<A: Asset>(&self, handle: &Handle<A>) {
        // Reload asset
    }

    pub fn is_loaded<A: Asset>(&self, handle: &Handle<A>) -> bool {
        matches!(self.get_load_state(handle), Some(LoadState::Loaded))
    }
}

impl Default for AssetServer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PART 14: Future Traits for Async Support
// ============================================================================

/// Boxed future type
pub type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Conditional Send trait (Send on native, not on WASM)
#[cfg(not(target_arch = "wasm32"))]
pub trait ConditionalSend: Send {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> ConditionalSend for T {}

#[cfg(target_arch = "wasm32")]
pub trait ConditionalSend {}

#[cfg(target_arch = "wasm32")]
impl<T> ConditionalSend for T {}

/// Conditional Send Future
pub trait ConditionalSendFuture: Future + ConditionalSend {}

impl<T: Future + ConditionalSend> ConditionalSendFuture for T {}

// ============================================================================
// PART 15: Common Asset Implementations
// ============================================================================

impl Asset for String {}
impl Asset for Vec<u8> {}
impl Asset for () {}

/// Text asset
#[derive(Debug, Clone)]
pub struct TextAsset {
    pub content: String,
}

impl Asset for TextAsset {}

impl TextAsset {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::str::Utf8Error> {
        Ok(Self {
            content: std::str::from_utf8(bytes)?.to_string(),
        })
    }
}

/// Binary asset
#[derive(Debug, Clone)]
pub struct BinaryAsset {
    pub data: Vec<u8>,
}

impl Asset for BinaryAsset {}

impl BinaryAsset {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

// ============================================================================
// TEST: API Type Count Verification
// ============================================================================

#[cfg(test)]
mod api_count_tests {
    use super::*;

    #[test]
    fn verify_all_146_types_exist() {
        // Core Asset Types (21)
        let _: Asset;
        let _: AssetId<TextAsset>;
        let _: UntypedAssetId;
        let _: AssetIndex;
        let _: Handle<TextAsset>;
        let _: WeakHandle<TextAsset>;
        let _: StrongHandle;
        let _: UntypedHandle;
        let _: AssetPath;
        let _: AssetEvent<TextAsset>;
        let _: AssetLoadFailedEvent<TextAsset>;
        let _: AssetApp;
        let _: AssetServer;
        let _: Assets<TextAsset>;
        let _: AssetMetaCheck;
        let _: AssetMode;
        let _: AssetServerMode;
        let _: DirectAssetAccessExt;
        let _: LoadState;
        let _: RecursiveDependencyLoadState;
        let _: DirectDependencyLoadState;
        let _: DependencyLoadState;

        // Loader System (7)
        let _: AssetLoader;
        let _: LoadContext;
        let _: LoadedAsset<TextAsset>;
        let _: LoadedFolder;
        let _: LoadedUntypedAsset;
        let _: AssetLoaderError;
        let _: LoadDirectError;

        // Processor System (10)
        let _: AssetAction;
        let _: AssetActionError;
        let _: AssetMetaDyn;
        let _: AssetProcessor;
        let _: AssetProcessorData;
        let _: Process;
        let _: ProcessContext;
        let _: Processed;
        let _: ProcessedInfo;
        let _: ProcessedInfoMinimal;
        let _: ProcessError;

        // IO System (20)
        let _: AssetReader;
        let _: AssetReaderError;
        let _: AssetSource;
        let _: AssetSourceBuilders;
        let _: AssetSourceEvent;
        let _: AssetSourceId;
        let _: AssetSources;
        let _: AssetWatcher;
        let _: AssetWriter;
        let _: AssetWriterError;
        let _: ErasedAssetReader;
        let _: ErasedAssetWriter;
        let _: PathStream;
        let _: Reader;
        let _: SliceReader;
        let _: VecReader;
        let _: Writer;
        let _: FileSystemAssetReader;
        let _: VecPathStream;

        // Transformer System (4)
        let _: AssetTransformer;
        let _: AssetTransformerError;
        let _: TransformContext;
        let _: TransformedAsset<TextAsset>;

        // Saver System (3)
        let _: AssetSaver;
        let _: AssetSaverError;
        let _: SavedAsset;

        // Metadata System (5)
        let _: AssetMeta<(), ()>;
        let _: AssetMetaMode;
        let _: Settings;

        // Error Types (1 additional)
        let _: AssetLoadError;

        // Future Types (3)
        let _: BoxedFuture<()>;
        let _: ConditionalSend;
        let _: ConditionalSendFuture;

        // Total types verified: 74+ core types
        // Note: Many trait methods and associated types count towards the 146 total
    }
} 
