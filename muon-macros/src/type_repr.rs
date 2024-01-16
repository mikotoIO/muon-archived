use syn::{PathArguments, Type};

#[derive(Debug, Clone)]
pub enum ScyllaType {
    Basic(String),
    List(Box<ScyllaType>),
    HashMap(Box<ScyllaType>, Box<ScyllaType>),
    Unsupported,
}

impl ScyllaType {
    pub fn to_cql_type(&self) -> String {
        match self {
            ScyllaType::Basic(x) => match x.as_str() {
                // from https://rust-driver.docs.scylladb.com/stable/data-types/data-types.html
                "bool" => "boolean".to_string(),
                "i8" => "tinyint".to_string(),
                "i16" => "smallint".to_string(),
                "i32" => "int".to_string(),
                "i64" => "bigInt".to_string(),
                "f32" => "float".to_string(),
                "f64" => "double".to_string(),
                "String" => "text".to_string(),
                "Uuid" => "uuid".to_string(),
                "OffsetDateTime" => "timestamp".to_string(),
                x => x.to_string(), // assume it's a UDT
            },
            ScyllaType::List(x) => format!("list<{}>", x.to_cql_type()),
            ScyllaType::HashMap(x, y) => format!("map<{}, {}>", x.to_cql_type(), y.to_cql_type()),
            ScyllaType::Unsupported => panic!("This type not supported yet"),
        }
    }
}

impl From<Type> for ScyllaType {
    fn from(ty: Type) -> Self {
        match ty {
            Type::Path(x) => {
                let segments = x.path.segments.into_iter().collect::<Vec<_>>();
                if segments.len() != 1 {
                    panic!("unsupported syntax, sorry about that")
                }
                let segment = segments.into_iter().next().unwrap();
                let name = segment.ident.to_string();

                match segment.arguments {
                    PathArguments::None => ScyllaType::Basic(name),
                    PathArguments::AngleBracketed(param) => {
                        let param = param
                            .args
                            .into_iter()
                            .filter_map(|x| match x {
                                syn::GenericArgument::Type(x) => Some(x),
                                _ => None,
                            })
                            .map(|x| Self::from(x))
                            .collect::<Vec<_>>();

                        // test
                        match name.as_str() {
                            "Option" => param[0].clone(),
                            "Vec" => ScyllaType::List(Box::new(param[0].clone())),
                            "HashMap" => ScyllaType::HashMap(
                                Box::new(param[0].clone()),
                                Box::new(param[1].clone()),
                            ),
                            _ => ScyllaType::Unsupported,
                        }
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
