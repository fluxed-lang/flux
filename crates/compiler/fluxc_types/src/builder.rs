use crate::{KeyedTableEntry, TableEntry, TypeField, TypeTable};

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
            fields: Some(
                self.fields
                    .iter()
                    .map(|(name, ty)| {
                        let field = TypeField { index, name: name.clone(), ty: *ty };
                        index += 1;
                        field
                    })
                    .collect(),
            ),
            size: self
                .fields
                .iter()
                .map(|(_, ty)| table.find(*ty).expect("failed to find type").size)
                .reduce(|size, out| match (size, out) {
                    (Some(a), Some(b)) => Some(a + b),
                    _ => None,
                })
                .and_then(|v| v),
        })
    }
}
