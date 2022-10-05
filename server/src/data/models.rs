use diesel::PgConnection;

/// A trait for structs that represent a value not yet saved to the database
/// `T` The type of the same data represented when saved
pub trait UnsavedModel<T> {
    /// Save the item to the database
    // Take the ownership of the item so that an unsaved item can no longer be used when it has been saved
    fn save(self, connection: &mut PgConnection) -> T;
}
