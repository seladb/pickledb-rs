use crate::pickledb::PickleDb;
use serde::Serialize;

/// A struct for extending PickleDB lists and adding more items to them
pub struct PickleDbListExtender<'a> {
    pub(crate) db: &'a mut PickleDb,
    pub(crate) list_name: String,
}

impl<'a> PickleDbListExtender<'a> {
    /// Add a single item to an existing list.
    ///
    /// As mentioned before, the lists are heterogeneous, meaning a single list can contain
    /// items of different types. That means that the item can be of any type that is serializable.
    /// That includes all primitive types, vectors, tuples and every struct that has the
    /// `#[derive(Serialize, Deserialize)` attribute.
    /// The method returns another `PickleDbListExtender` object that enables to continue adding
    /// items to the list.
    ///
    /// # Arguments
    ///
    /// * `value` - a reference of the item to add to the list
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
    /// // create a new list
    /// db.lcreate("list1").unwrap()
    ///
    /// // add items of different types to the list
    ///   .ladd(&100)
    ///   .ladd(&String::from("my string"))
    ///   .ladd(&vec!["aa", "bb", "cc"]);
    /// ```
    ///
    pub fn ladd<V>(&mut self, value: &V) -> PickleDbListExtender
    where
        V: Serialize,
    {
        self.db.ladd(&self.list_name, value).unwrap()
    }

    /// Add multiple items to an existing list.
    ///
    /// As mentioned before, the lists are heterogeneous, meaning a single list can contain
    /// items of different types. That means that the item can be of any type that is serializable.
    /// That includes all primitive types, vectors, tuples and every struct that has the
    /// `#[derive(Serialize, Deserialize)` attribute.
    /// This method adds multiple items to the list, but since they're in a vector that means all
    /// of them are of the same type. Of course it doesn't mean that the list cannot contain items
    /// of other types as well, as you can see in the example below.
    /// The method returns another `PickleDbListExtender` object that enables to continue adding
    /// items to the list.
    ///
    /// # Arguments
    ///
    /// * `seq` - an iterator containing references to the new items to add to the list
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
    /// // create a new list
    /// db.lcreate("list1");
    ///
    /// // add a bunch of numbers to the list
    /// db.lextend("list1", &vec![100, 200, 300]).unwrap()
    ///
    /// // add a bunch of strings to the list
    ///   .lextend(&vec!["aa", "bb", "cc"]);
    ///
    /// // now the list contains 6 items and looks like this: [100, 200, 300, "aa, "bb", "cc"]
    /// ```
    ///
    pub fn lextend<'i, V, I>(&mut self, seq: I) -> PickleDbListExtender
    where
        V: 'i + Serialize,
        I: IntoIterator<Item = &'i V>,
    {
        self.db.lextend(&self.list_name, seq).unwrap()
    }
}
