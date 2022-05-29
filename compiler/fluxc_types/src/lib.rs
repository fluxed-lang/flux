/// Datatype that contains type information.
#[derive(Debug)]
pub struct TypeTable {
    /// Entries within the table.
    entries: Vec<KeyedTableEntry>,
    /// THe next type ID.
    next_id: usize,
}

impl TypeTable {
    /// This method creates and returns an empty `TypeTable`.
    pub fn empty() -> Self {
        TypeTable { entries: vec![], next_id: 0 }
    }
    /// This method creates and returns a new `TypeTable` instance with the core
    /// types declared.
    pub fn with_core_types() -> Self {
        let mut table = Self::empty();
        // primitives
        table.append(TableEntry {
            name: "int".into(),
            size: Some(8),
            internal: false,
            fields: Some(vec![]),
        });
        table.append(TableEntry {
            name: "float".into(),
            size: Some(8),
            internal: false,
            fields: Some(vec![]),
        });
        table.append(TableEntry {
            name: "bool".into(),
            size: Some(1),
            internal: false,
            fields: Some(vec![]),
        });
        table.append(TableEntry {
            name: "char".into(),
            size: Some(8),
            internal: false,
            fields: Some(vec![]),
        });
        table.append(TableEntry {
            name: "unit".into(),
            size: Some(0),
            internal: false,
            fields: Some(vec![]),
        });
        // reference
        table.append(TableEntry {
            name: "ref".into(),
            size: Some(8),
            internal: false,
            fields: Some(vec![]),
        });
        table
    }
    /// Find a type with a particular ID.
    pub fn find(&self, id: usize) -> Option<&KeyedTableEntry> {
        match self.entries.binary_search_by_key(&id, |entry: &KeyedTableEntry| entry.id) {
            Ok(i) => self.entries.get(i),
            Err(_) => None,
        }
    }
    /// Find a type by name.
    pub fn find_by_name<S: AsRef<str>>(&self, name: S) -> Option<&KeyedTableEntry> {
        self.entries.iter().find(|entry| entry.name == name.as_ref())
    }
    /// Append an entry to the type table. Returns the type ID of the entry.
    pub fn append(&mut self, entry: TableEntry) -> &KeyedTableEntry {
        self.entries.push(KeyedTableEntry {
            id: self.next_id,
            name: entry.name,
            size: entry.size,
            internal: entry.internal,
            fields: entry.fields,
        });
        self.next_id += 1;
        // return the newly appended table entry
        self.entries.last().unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub struct KeyedTableEntry {
    /// The ID of this type.
    pub id: usize,
    /// The name of this type.
    pub name: String,
    /// The size of this type in bytes. If this typed is unsized,
    /// then this value is `None`.
    pub size: Option<u8>,
    /// Whether this type is internal or not.
    pub internal: bool,
    /// Fields on this type.
    pub fields: Option<Vec<TypeField>>,
}

#[derive(Debug, PartialEq)]
pub struct TableEntry {
    /// The name of this type.
    pub name: String,
    /// The size of this type in bytes. If this typed is unsized,
    /// then this value is `None`.
    pub size: Option<u8>,
    /// Whether this type is internal or not.
    pub internal: bool,
    /// Fields on this type.
    pub fields: Option<Vec<TypeField>>,
}

/// A field on a type.
#[derive(Debug, PartialEq)]
pub struct TypeField {
    /// The position of this field in the parent type.
    pub index: usize,
    /// The name of this field.
    pub name: String,
    /// The ID of the type of this field.
    pub ty: usize,
}

/// Utility type for creating struct types.
#[derive(Debug)]
pub struct StructBuilder {
    name: String,
    fields: Vec<(String, usize)>,
}

impl StructBuilder {
    /// Create a new struct builder.
    pub fn new<S: ToString>(name: S) -> Self {
        StructBuilder { name: name.to_string(), fields: vec![] }
    }
    /// Add a field to this type.
    pub fn field<S: ToString>(mut self, name: S, ty: &KeyedTableEntry) -> Self {
        self.fields.push((name.to_string(), ty.id));
        self
    }
    /// Build the output struct.
    pub fn build(self, table: &mut TypeTable) -> &KeyedTableEntry {
        let mut index = 0;
        table.append(TableEntry {
            name: self.name,
            internal: false,
            fields: Some(
                self.fields
                    .into_iter()
                    .map(|(name, ty)| {
                        let field = TypeField { index, name, ty };
                        index += 1;
                        field
                    })
                    .collect(),
            ),
            size: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::StructBuilder;
    use crate::{TypeField, TypeTable};

    #[test]
    fn test_create_struct() {
        let mut types = TypeTable::with_core_types();
        let int = types.find(0).unwrap();
        let my_struct = StructBuilder::new("Test").field("inner", int).build(&mut types);
        assert_eq!(my_struct.id, 6);
        assert_eq!(
            my_struct.fields,
            Some(vec![TypeField { index: 0, name: "inner".to_string(), ty: 0 }])
        )
    }
}
