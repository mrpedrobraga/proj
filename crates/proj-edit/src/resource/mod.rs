//! # Resource
//!
//! Module for managing Resource references and acquisition of Resource locks.

/// See the [module-level documentation][self].
pub trait ResourceSet {
    /// The type used to refer to resources in this set.
    type ResourceId;

    /// Gets a semi-exclusive lock to read a resource (if it exists), but not mutate it.
    ///
    /// Several parties can hold a [ResourceReadLock] at the same time,
    /// but if even one party is reading a resource, none can be writing to it.
    fn read(&self, id: Self::ResourceId) -> Option<ResourceReadLock>;

    /// Gets an exclusive lock to read and mutate a resource (if it exists).
    ///
    /// Only one party can hold a [ResourceWriteLock] at a time... however,
    /// some parties can get "sneak peeks" into what the current lock holder is doing.
    fn write(&self, id: Self::ResourceId) -> Option<ResourceWriteLock>;

    /// Gets a handle to peek into a resource's state (if it exists).
    ///
    /// This is useful for safely accompanying the edits made by a writer,
    /// as the writer provides "sneak peeks" into the intermediate states of its process.
    fn watch(&self, id: Self::ResourceId) -> Option<ResourceWatcher>;

    /// Like [Self::read], but instead of waiting, will fail immediately if a lock can not be acquired at this time.
    fn try_read(&self, id: Self::ResourceId)
        -> Option<Result<ResourceReadLock, ResourceLockError>>;

    /// Like [Self::write], but instead of waiting, will fail immediately if a lock can not be acquired at this time.
    fn try_write(
        &self,
        id: Self::ResourceId,
    ) -> Option<Result<ResourceWriteLock, ResourceLockError>>;
}

/// Marks a type as a resource identifier.
pub trait ResourceIdentifier {}

/// A lock allowing its owner to read the current state of a Resource safely.
pub struct ResourceReadLock {}

/// A lock allowing its owner to mutate a Resource safely.
pub struct ResourceWriteLock {}

/// A handle allowing its owner to observe the state of a Resource safely as it changes.
pub struct ResourceWatcher {}

/// An error describing the reason why a lock couldn't be acquired.
pub enum ResourceLockError {
    ResourceLockedForReading,
    ResourceLockedForWriting,
}
