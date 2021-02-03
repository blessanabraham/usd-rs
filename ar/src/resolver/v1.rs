use core::any::Any;

use crate::{Asset, AssetInfo, ResolverContext, ResolverError};

mod wrapper;

/// Trait for the asset resolution system. An asset resolver is
/// responsible for resolving asset information (including the asset's
/// physical path) from a logical path.
pub trait Resolver {
    /// Configures the resolver for a given asset path
    fn configure_resolver_for_asset(&mut self);

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
    fn anchor_relative_path(&self, anchor_path: &str, path: &str) -> &str;

    /// Returns true if the given path is a relative path.
    fn is_relative_path(&self) -> bool;

    /// Returns true if the given path is a repository path.
    fn is_repository_path(&self) -> bool;

    /// Returns whether this path is a search path.
    fn is_search_path(&self) -> bool;

    /// Returns the normalized extension for the given `path`.
    fn get_extension(&self) -> &str;

    /// Returns a normalized version of the given `path`.
    fn compute_normalized_path(&self) -> &str;

    /// Returns the computed repository path using the current resolver.
    fn compute_repository_path(&self) -> &str;

    /// Returns the local path for the given `path`.
    fn compute_local_path(&self, path: &str) -> &str;

    /// Returns the resolved filesystem path for the file identified by
    /// the given `path` if it exists. If the file does not exist,
    /// returns an empty string.
    fn resolve(&self, path: &str) -> &str;

    /// Binds the given context to this resolver.
    ///
    /// Clients should generally use [`ResolverContextBinder`] instead of calling
    /// this function directly.
    fn bind_context(&mut self, context: &ResolverContext, binding_data: &dyn Any);

    /// Unbind the given context from this resolver.
    ///
    /// Clients should generally use [`ResolverContextBinder`] instead of calling
    /// this function directly.
    fn unbind_context(&mut self, context: &ResolverContext, binding_data: &dyn Any);

    /// Return a default [`ResolverContext`] that may be bound to this resolver
    /// to resolve assets when no other context is explicitly specified.
    ///
    /// This function should not automatically bind this context, but should
    /// create one that may be used later.
    fn create_default_context(&self) -> ResolverContext;

    /// Return a default [`ResolverContext`] that may be bound to this resolver
    /// to resolve the asset located at `file_path` when no other context is
    /// explicitly specified.
    ///
    /// This function should not automatically bind this context, but should
    /// create one that may be used later.
    fn create_default_context_for_asset(&self, file_path: &str) -> ResolverContext;

    /// Refresh any caches associated with the given context.
    fn refresh_context(&mut self, context: ResolverContext);

    /// Returns the currently-bound asset resolver context.
    ///
    /// see [`Self::bind_context()`], [`Self::unbind_context()`]
    fn get_current_context(&self) -> ResolverContext;

    /// Returns the resolved filesystem path for the file identified
    /// by `path` following the same path resolution behavior as in
    /// [`Self::resolve()`].
    ///
    /// If the file identified by `path` represents an asset and
    /// `asset_info` is not [`None`], the resolver should populate
    /// `asset_info` with whatever additional metadata it knows or can
    /// reasonably compute about the asset without actually opening it.
    ///
    /// see [`Self::resolve()`].
    fn resolve_with_asset_info(&self, path: &str, asset_info: Option<AssetInfo>) -> &str;

    /// Update `asset_info` with respect to the given `file_version`.
    /// Note: This API is currently in flux. In general, you should prefer
    /// to call [`Self::resolve_with_asset_info()`]
    fn update_asset_info(
        &self,
        identifier: &str,
        file_path: &str,
        file_version: &str,
        asset_info: &mut AssetInfo,
    );

    /// Returns a value representing the last time the asset identified
    /// by `path` was modified. `resolved_path` is the resolved path
    /// of the asset.
    ///
    /// Implementations may use whatever value is most appropriate
    /// for this timestamp. The value must be equality comparable,
    /// and this function must return a different timestamp whenever
    /// an asset has been modified. For instance, if an asset is stored
    /// as a file on disk, the timestamp may simply be that file's mtime.
    ///
    /// If a timestamp cannot be retrieved, returns an [`ResolverError`].
    fn get_modification_timestamp(
        &self,
        path: &str,
        resolved_path: &str,
    ) -> Result<i64, ResolverError>;

    /// Returns an [`Asset`] object for the asset located at `resolved_path`.
    /// Returns an error if object could not be created.
    ///
    /// The returned ArAsset object provides functions for accessing the
    /// contents of the specified asset.
    ///
    /// Note that clients may still be using the data associated with
    /// this object even after the last shared_ptr has been destroyed. For
    /// example, a client may have created a memory mapping using the FILE*
    /// presented in the ArAsset object; this would preclude truncating or
    /// overwriting any of the contents of that file.
    fn open_asset(&self, resolved_path: &str) -> Result<&dyn Asset, ResolverError>;

    /// Create path needed to write a file to the given `path`.
    ///
    /// For example:
    /// - A filesystem-based resolver might create the directories specified
    ///   in `path`.
    /// - A database-based resolver might create a new table, or it might
    ///   ignore this altogether.
    ///
    /// In practice, when writing a layer, [`Self::can_write_layer_to_path()`] will be called
    /// first to check if writing is permitted. If this returns success, then
    /// [`Self::create_path_for_layer()`] will be called before writing the layer out.
    fn create_path_for_layer(&self, path: &str) -> Result<(), ResolverError>;

    /// Returns [`Result::Ok`] if a file may be written to the given `path`, [`ResolverError::CannotWriteLayerToPath`]
    /// otherwise.
    ///
    /// In practice, when writing a layer, [`Self::can_write_layer_to_path()`] will be called
    /// first to check if writing is permitted. If this returns success, then
    /// [`Self::create_path_for_layer()`] will be called before writing the layer out.
    fn can_write_layer_to_path(&self) -> Result<(), ResolverError>;

    /// Returns success if a new file may be created using the given.
    /// `identifier`, [`ResolverError::CannotCreateNewLayerWithIdentifier`] otherwise.
    fn can_create_new_layer_with_identifier(&self, identifier: &str) -> Result<(), ResolverError>;

    /// A scoped resolution cache indicates to the resolver that results of
    /// calls to resolve should be cached for a certain scope. This is
    /// important for performance and also for consistency -- it ensures
    /// that repeated calls to resolve with the same parameters will
    /// return the same result.
    ///
    /// A resolution cache scope is opened by a call to begin_cache_scope and
    /// must be closed with a matching call to EndCacheScope. The resolver must
    /// cache the results of resolve until the scope is closed. Note that these
    /// calls may be nested.
    ///
    /// Cache scopes are thread-specific: if multiple threads are running and
    /// a cache scope is opened in one of those threads, caching should be
    /// enabled in that thread only.
    ///
    /// When opening a scope, a resolver may return additional data for
    /// implementation-specific purposes. This data may be shared across
    /// threads, so long as it is safe to access this data concurrently.
    ///
    /// ArResolverScopedCache is an RAII object for managing cache scope
    /// lifetimes and data. Clients should generally use that class rather
    /// than calling the begin_cache_scope and EndCacheScope functions manually.
    ///
    /// see [`ResolverScopedCache`]

    /// Mark the start of a resolution caching scope.
    ///
    /// Clients should generally use [`ResolverScopedCache`] instead of calling
    /// this function directly.
    ///
    /// Resolvers may fill `cache_scope_data` with arbitrary data. Clients may
    /// also pass in a `cache_scope_data` populated by an earlier call to
    /// [`Self::begin_cache_scope()`] to allow the resolver access to that information.
    ///
    /// see [`ResolverScopedCache`]
    fn begin_cache_scope(&mut self, cache_scope_data: Option<&dyn Any>);

    /// Mark the end of a resolution caching scope.
    ///
    /// Clients should generally use [`ResolverScopedCache'] instead of calling
    /// this function directly.
    ///
    /// `cache_scope_data` should contain the data that was populated by the
    /// previous corresponding call to begin_cache_scope.
    ///
    /// see [`ResolverScopedCache`]
    fn end_cache_scope(&mut self, cache_scope_data: &dyn Any);
}

// /// Returns the configured asset resolver.
// ///
// /// When first called, this function will determine the ArResolver subclass
// /// to use for asset resolution via the following process:
// ///
// /// - If a preferred resolver has been set via \ref ArSetPreferredResolver,
// ///   it will be selected.
// ///
// /// - Otherwise, a list of available ArResolver subclasses in plugins will
// ///   be generated. If multiple ArResolver subclasses are found, the list
// ///   will be sorted by typename. ArDefaultResolver will be added as the last
// ///   element of this list, and the first resolver in the list will be
// ///   selected.
// ///
// /// - The plugin for the selected subclass will be loaded and an instance
// ///   of the subclass will be constructed.
// ///
// /// - If an error occurs, an ArDefaultResolver will be constructed.
// ///
// /// The constructed ArResolver subclass will be cached and used to service
// /// function calls made on the returned resolver.
// ///
// /// Note that this function may not return the constructed subclass itself,
// /// meaning that dynamic casts to the subclass type may fail. See
// /// ArGetUnderlyingResolver if access to this object is needed.
// pub fn get_resolver() -> impl Resolver {
//     todo!()
// }
//
// /// Set the preferred [`Resolver`] subclass used by [`get_resolver`].
// ///
// /// Consumers may override [`get_resolver`]'s plugin resolver discovery and
// /// force the use of a specific resolver subclass by calling this
// /// function with the typename of the implementation to use.
// ///
// /// If the subclass specified by `resolver_type_name` cannot be found,
// /// `get_resolver` will issue a warning and fall back to using
// /// [`DefaultResolver`].
// ///
// /// This must be called before the first call to ArGetResolver.
// pub fn set_preferred_resolver(resolver_type_name: &str) {
//     todo!()
// }
//
// /// # Advanced API
// ///
// /// <section class="warning">
// /// These functions should typically not be used by consumers except
// /// in very specific cases. Consumers who want to retrieve an ArResolver to
// /// perform asset resolution should use \ref ArGetResolver.
// /// </section>
//
// /// Returns the underlying ArResolver instance used by ArGetResolver.
// ///
// /// This function returns the instance of the ArResolver subclass used by
// /// ArGetResolver and can be dynamic_cast to that type.
// ///
// /// <section class="warning">
// /// This functions should typically not be used by consumers except
// /// in very specific cases. Consumers who want to retrieve an ArResolver to
// /// perform asset resolution should use \ref ArGetResolver.
// /// </section>
// pub fn get_underlying_resolver() -> impl Resolver {
//     todo!()
// }
//
// /// Returns list of TfTypes for available ArResolver subclasses.
// ///
// /// This function returns the list of ArResolver subclasses used to determine
// /// the resolver implementation returned by [`get_resolver`]. See
// /// documentation on that function for more details.
// ///
// /// If this function is called from within a call (or calls) to
// /// [`create_resolver`], the [`Resolver`] subclass(es) being created will
// /// be removed from the returned list.
// ///
// /// <section class="warning>
// /// This functions should typically not be used by consumers except
// /// in very specific cases. Consumers who want to retrieve a [`Resolver`] to
// /// perform asset resolution should use [`get_resolver`].
// /// </section>
// pub fn get_available_resolvers() -> Vec<String> {
//     todo!()
// }
//
// /// Construct an instance of the [`Resolver`] subclass specified by
// /// `resolver_type`.
// ///
// /// This function will load the plugin for the given `resolver_type` and
// /// construct and return a new instance of the specified [`Resolver`] subclass.
// /// If an error occurs, coding errors will be emitted and this function
// /// will return an [`DefaultResolver`] instance.
// ///
// /// Note that this function *does not* change the resolver used by
// /// [`get_resolver`] to an instance of `resolver_type`.
// ///
// /// This function is not safe to call concurrently with itself or
// /// [`get_available_resolvers`].
// ///
// /// <section class="warning">
// /// This functions should typically not be used by consumers except
// /// in very specific cases. Consumers who want to retrieve an ArResolver to
// /// perform asset resolution should use [`get_resolver`].
// /// </section>
// pub fn create_resolver(resolver_type: &str) -> impl Resolver {
//     todo!()
// }
