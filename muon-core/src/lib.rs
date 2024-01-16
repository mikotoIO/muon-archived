pub trait Entity {
    fn scratch();

    #[doc(hidden)]
    fn __entity_name() -> &'static str;
    #[doc(hidden)]
    fn __entity_fields() -> &'static [&'static str];
    #[doc(hidden)]
    fn __entity_field_types() -> &'static [&'static str];

    fn get_scylla_schema() -> String {
        let name = Self::__entity_name();

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
        let pkey_statement = "    PRIMARY KEY (id)";
        format!(
            "CREATE TABLE {} {{\n{}\n{}\n}}",
            name, field_cql, pkey_statement
        )
    }
}
