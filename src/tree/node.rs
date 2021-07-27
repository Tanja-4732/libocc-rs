/// This trait must be implemented on any struct used within the data hierarchy
pub trait Node {
    /// Replaces a node with a new one
    fn update(self, replacement: Self);

    /// Deletes a node
    fn delete(self);

    /// Returns a reference to the parent node (if any)
    fn get_parent(&self) -> Option<&Self>;

    /// Returns a mutable reference to the parent node (if any)
    fn get_parent_mut(&mut self) -> Option<&mut Self>;

    /// Returns a reference to the child nodes
    fn get_children(&self) -> [&Self];

    /// Returns a reference to the child nodes (mutable)
    fn get_children_mut(&mut self) -> [&mut Self];

    /// If this is the root node or not
    fn is_root(&self) -> bool;
}
