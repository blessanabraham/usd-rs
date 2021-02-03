use core::any::Any;

use crate::{Asset, AssetInfo, ResolvedPath, ResolverContext, ResolverError, WritableAsset};

mod wrapper;

/// Enumeration of write modes for open_asset_for_write
#[derive(Clone, Debug)]
pub enum WriteMode {
    /// Open asset for in-place updates. If the asset exists, its contents
    /// will not be discarded and writes may overwrite existing data.
    /// Otherwise, the asset will be created.
    Update = 0,

    /// Open asset for replacement. If the asset exists, its contents will
    /// be discarded by the time the ArWritableAsset is destroyed.
    /// Otherwise, the asset will be created.
    Replace,
}

/// Trait for the asset resolution system. An asset resolver is
/// responsible for resolving asset information (including the asset's
/// physical path) from a logical path.
pub trait Resolver {
    /// Identifiers are canonicalized asset paths that may be assigned
    /// to a logical asset to facilitate comparisons and lookups. They
    /// may be used to determine if different asset paths might refer to
    /// the same asset without performing resolution.
    ///
    /// Since identifiers are just a form of asset path, they may be used
    /// with other functions on ArResolver that require an asset path, like
    /// resolve.
    ///
    /// If two asset paths produce the same identifier, those asset paths
    /// must refer to the same asset. However, in some cases comparing
    /// identifiers may not be sufficient to determine if assets are equal.
    /// For example, there could be two assets with the same identifier
    /// but whose contents were read from different resolved paths because
    /// different resolver contexts were bound when those assets were loaded.

    /// Returns an identifier for the asset specified by `asset_path`.
    /// If `anchor_asset_path` is not [`None`], it is the resolved asset path
    /// that `asset_path` should be anchored to if it is a relative path.
    fn create_identifier(&self, asset_path: &str, anchor_asset_path: Option<&ResolvedPath>)
        -> &str;

    /// Returns an identifier for a new asset specified by `asset_path`.
    /// If `anchor_asset_path` is not empty, it is the resolved asset path
    /// that `asset_path` should be anchored to if it is a relative path.
    fn create_identifier_for_new_asset(
        &self,
        asset_path: &str,
        anchor_asset_path: &ResolvedPath,
    ) -> &str;

    /// Returns the resolved path for the asset identified by the given
    /// `asset_path` if it exists. If the asset does not exist, returns an empty
    /// [`ResolvedPath`].
    fn resolve(&self, asset_path: &str) -> ResolvedPath;

    /// Returns the resolved path for the given `asset_path` that may be used
    /// to create a new asset. If such a path cannot be computed for
    /// `asset_path`, returns an empty [`ResolvedPath`].
    ///
    /// Note that an asset might or might not already exist at the returned
    /// resolved path.
    fn resolve_for_new_asset(&self, asset_path: &str) -> ResolvedPath;

    /// Binds the given context to this resolver.
    ///
    /// Clients should generally use [`ResolverContextBinder`] instead of calling
    /// this function directly.
    ///
    /// see [`ResolverContextBinder`]
    fn bind_context(&mut self, context: &ResolverContext, binding_data: &dyn Any);

    /// Unbind the given context from this resolver.
    ///
    /// Clients should generally use [`ResolverContextBinder`] instead of calling
    /// this function directly.
    ///
    /// see [`ResolverContextBinder`]
    fn unbind_context(&mut self, context: &ResolverContext, binding_data: &dyn Any);

    /// Return a [`ResolverContext`] that may be bound to this resolver
    /// to resolve assets when no other context is explicitly specified.
    fn create_default_context(&self) -> ResolverContext;

    /// Return a [`ResolverContext`] that may be bound to this resolver
    /// to resolve the asset located at `asset_path` when no other context is
    /// explicitly specified.
    fn create_default_context_for_asset(&self, asset_path: &str) -> ResolverContext;

    /// Return a [`ResolverContext`] created from the primary [`Resolver`]
    /// implementation using the given `context_str`.
    fn create_context_from_string(&self, context_str: &str) -> ResolverContext;

    /// Return a [`ResolverContext`] created from the [`Resolver`] registered
    /// for the given `uri_scheme` using the given `context_str`.
    ///
    /// An empty [`uriScheme`] indicates the primary resolver and is
    /// equivalent to [`CreateContextFromString()`].
    ///
    /// If no resolver is registered for `uri_scheme`, returns an empty
    /// [`ResolverContext`].
    fn create_context_from_uri_and_string(
        &self,
        uri_scheme: &str,
        context_str: &str,
    ) -> ResolverContext;

    // Return a [`ResolverContext`] created by combining the [`ResolverContext`]
    /// objects created from the given `context_strings`.
    ///
    /// `context_strings` is a list of pairs of strings. The first element in the
    /// pair is the URI scheme for the [`Resolver`] that will be used to create
    /// the [`ResolverContext`] from the second element in the pair. An empty
    /// URI scheme indicates the primary resolver.
    ///
    /// # Example:
    /// ```
    /// let strings = vec![("", "context str 1"), ("my_scheme", "context str 2")];
    /// // let ctx = get_resolver().create_context_from_strings(&strings);
    /// ```
    ///
    /// This will use the primary resolver to create a [`ResolverContext`]
    /// using the string "context str 1" and use the resolver registered for
    /// the "my_scheme" URI scheme to create a [`ResolverContext`] using
    /// "context str 2". These contexts will be combined into a single
    /// [`ResolverContext`] and returned.
    ///
    /// If no resolver is registered for a URI scheme in an entry in
    /// `context_strings`, that entry will be ignored.
    fn create_context_from_strings(&self, context_strings: &[(&str, &str)]) -> ResolverContext;

    /// Refresh any caches associated with the given context.
    fn refresh_context(&mut self, context: &ResolverContext);

    /// Returns the asset resolver context currently bound in this thread.
    ///
    /// see [`Self::bind_context()`], [`Self::unbind_context()`]
    fn get_current_context(&self) -> &ResolverContext;

    /// Returns true if `asset_path` is a context-dependent path, false
    /// otherwise.
    ///
    /// A context-dependent path may result in different resolved paths
    /// depending on what asset resolver context is bound when Resolve
    /// is called. Assets located at the same context-dependent path may not
    /// be the same since those assets may have been loaded from different
    /// resolved paths. In this case, the assets' resolved paths must be
    /// consulted to determine if they are the same.
    fn is_context_dependent_path(&self, asset_path: &str) -> bool;

    /// Returns the file extension for the given `asset_path`. The returned
    /// extension does not include a "." at the beginning.
    fn get_extension(&self, asset_path: &str) -> &str;

    /// Returns a [`AssetInfo`] populated with additional metadata (if any)
    /// about the asset at the given `asset_path`. `resolved_path` is the
    /// resolved path computed for the given `asset_path`.
    fn get_asset_info(&self, asset_path: &str, resolved_path: &ResolvedPath) -> &AssetInfo;

    /// Return a value representing the last time the asset at the given
    /// `asset_path` was modified. `resolved_path` is the resolved path
    /// computed for the given `asset_path`. If a timestamp cannot be
    /// retrieved, return a [`ResolverError`].
    ///
    /// This timestamp may be equality compared to determine if an asset
    /// has been modified.
    fn get_modification_timestamp(
        &self,
        asset_path: &str,
        resolved_path: &str,
    ) -> Result<i64, ResolverError>;

    /// Returns an [`Asset`] object for the asset located at `resolved_path`.
    /// Returns an error if object could not be created.
    ///
    /// The returned [`Asset`] object provides functions for accessing the
    /// contents of the specified asset.
    fn open_asset(&self, resolved_path: &ResolvedPath) -> Result<&dyn Asset, ResolverError>;

    /// Returns an [`WritableAsset`] object for the asset located at
    /// `resolved_path` using the specified `write_mode`. Returns a [`ResolverError`]
    /// if object could not be created.
    ///
    /// The returned [`WritableAsset`] object provides functions for writing data
    /// to the specified asset.
    ///
    /// Note that support for reading an asset through other APIs while it
    /// is open for write is implementation-specific. For example, writes to
    /// an asset may or may not be immediately visible to other threads or
    /// processes depending on the implementation.
    fn open_asset_for_write(
        &self,
        resolved_path: &ResolvedPath,
        write_mode: WriteMode,
    ) -> dyn WritableAsset;

    /// Mark the start of a resolution caching scope.
    ///
    /// Clients should generally use [`ResolverScopedCache`] instead of calling
    /// this function directly.
    ///
    /// Resolvers may fill `cache_scope_data` with arbitrary data. Clients may
    /// also pass in a `cache_scope_data` populated by an earlier call to
    /// [`Resolver::begin_cache_scope()`] to allow the resolver access to that information.
    ///
    /// see [`ResolverScopedCache`]
    fn begin_cache_scope(&mut self, cache_scope_data: Option<&dyn Any>);

    /// Mark the end of a resolution caching scope.
    ///
    /// Clients should generally use [`ResolverScopedCache`] instead of calling
    /// this function directly.
    ///
    /// \p cacheScopeData should contain the data that was populated by the
    /// previous corresponding call to BeginCacheScope.
    ///
    /// see [`ResolverScopedCache`]
    fn end_cache_scope(&mut self, cache_scope_data: Option<&dyn Any>);

    /// Configures the resolver for a given asset path
    /// Default implementation does nothing.
    ///
    /// deprecated
    #[deprecated(since = "2.0", note = "Default implementation does nothing")]
    fn configure_resolver_for_asset(&mut self, _path: &str) {}

    /// Returns the path formed by anchoring `path` to `anchor_path`.
    ///
    /// If `anchor_path` ends with a trailing '/', it is treated as
    /// a directory to which `path` will be anchored. Otherwise, it
    /// is treated as a file and `path` will be anchored to its
    /// containing directory.
    ///
    /// If `anchor_path` is empty, `path` will be returned as-is.
    ///
    /// If `path` is empty or not a relative path, it will be
    /// returned as-is.
    ///
    /// deprecated Planned for removal in favor of [`Self::create_identifier()`].
    #[deprecated(since = "2.0", note = "Please use create_identifier() instead")]
    fn anchor_relative_path(&self, anchor_path: &str, path: &str) -> &str;

    /// Returns true if the given path is a relative path.
    ///
    /// deprecated
    #[deprecated(since = "2.0")]
    fn is_relative_path(&self, path: &str) -> bool;

    /// Returns whether this path is a search path.
    /// The default implementation returns false.
    ///
    /// deprecated
    #[deprecated(since = "2.0")]
    fn is_search_path(&self, _path: &str) -> bool {
        false
    }

    /// Returns true if the given path is a repository path.
    ///
    /// deprecated
    #[deprecated(since = "2.0")]
    fn is_repository_path(&self, path: &str) -> bool;

    /// Fetch the asset identified by `path` to the filesystem location
    /// specified by `resolved_path`. `resolved_path` is the resolved path
    /// that results from calling [`Self::resolve()`] or [`Self::resolve_with_asset_info()`] on
    /// `path`.
    ///
    /// This method provides a way for consumers that expect assets
    /// to exist as physical files on disk to retrieve data from
    /// systems that store data in external data stores, e.g. databases,
    /// etc.
    ///
    /// Returns true if the asset was successfully fetched to the specified
    /// `resolved_path` or if no fetching was required. If `resolved_path`
    /// is not a local path or the asset could not be fetched to that path,
    /// returns false.
    ///
    /// The default implementation assumes no fetching is required and returns
    /// true.
    ///
    /// deprecated Planned for removal in favor or using [`Self::open_asset()`] to read
    /// data instead of requiring assets to be fetched to disk.
    #[deprecated(since = "2.0", note = "Please use open_asset() instead")]
    fn fetch_to_local_resolved_path(&self, _path: &str, _resolved_path: &str) -> bool {
        true
    }

    /// Create path needed to write a file to the given `path`.
    ///
    /// For example:
    /// - A filesystem-based resolver might create the directories specified
    ///   in `path`.
    /// - A database-based resolver might create a new table, or it might
    ///   ignore this altogether.
    ///
    /// In practice, when writing a layer, [`Self::can_write_layer_to_path()`]
    /// will be called first to check if writing is permitted. If this returns true,
    /// then [`Self::create_path_for_layer()`] will be called before writing the layer out.
    ///
    /// Returns [`ResolverError`] on error.
    ///
    /// deprecated Planned for removal in favor of making [`Self::open_asset_for_write()`]
    /// responsible for creating any intermediate path that might be needed.
    #[deprecated(since = "2.0", note = "Please use open_asset_for_write() instead")]
    fn create_path_for_layer(&self, path: &str) -> Result<(), ResolverError>;

    /// Returns [`Result::Ok`] if a file may be written to the given `path`,
    /// [`ResolverError`] otherwise.
    ///
    /// In practice, when writing a layer, [`Self::can_write_layer_to_path{}`] will be called
    /// first to check if writing is permitted. If this returns [`Ok`], then
    /// [`Self::create_path_for_layer()`] will be called before writing the layer out.
    ///
    /// The default implementation returns [`Result::Ok`].
    ///
    /// deprecated Planned for removal in favor of making [`Self::open_asset_for_write()`]
    /// responsible for determining if a layer can be written to a given path.
    #[deprecated(since = "2.0", note = "Please use open_asset_for_write() instead")]
    fn can_write_layer_to_path(&self, _path: &str) -> Result<(), ResolverError> {
        Ok(())
    }

    /// Returns [`Result::Ok`] if a new file may be created using the given
    /// `identifier`, [`ResolverError`] otherwise.
    ///
    /// The default implementation returns [`Result::Ok`].
    ///
    /// deprecated Planned for removal in favor of using [`Self::resolve_for_new_asset()`]
    /// to determine if a new layer can be created with a given identifier.
    #[deprecated(since = "2.0", note = "Please use resolve_for_new_asset() instead")]
    fn can_create_new_layer_with_identifier(&self, _identifier: &str) -> Result<(), ResolverError> {
        Ok(())
    }
}
