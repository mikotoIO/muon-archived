pub struct EntityMetadata {
    pub pkey: &'static str,
}

pub trait Entity {
    fn scratch();

    #[doc(hidden)]
    fn __entity_name() -> &'static str;
    #[doc(hidden)]
    fn __entity_fields() -> &'static [&'static str];
    #[doc(hidden)]
    fn __entity_field_types() -> &'static [&'static str];
    #[doc(hidden)]
    fn __entity_meta() -> &'static EntityMetadata {
        const PKEY: EntityMetadata = EntityMetadata { pkey: "id" };
        &PKEY
    }

    fn build_table_schema() -> String {
        let name = Self::__entity_name();
        let meta = Self::__entity_meta();

        let fields = Self::__entity_fields()
            .iter()
            .zip(Self::__entity_field_types().iter())
            .map(|(name, ty)| (name.to_string(), ty.to_string()))
            .collect::<Vec<_>>();

        let field_cql = fields
            .iter()
            .map(|x| format!("    {} {},", x.0, x.1))
            .collect::<Vec<_>>()
            .join("\n");
        let pkey_statement = format!("    PRIMARY KEY ({})", meta.pkey);
        format!(
            "CREATE TABLE {} {{\n{}\n{}\n}}",
            name, field_cql, pkey_statement
        )
    }
}
