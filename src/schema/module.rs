use super::ast::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeModule {
    pub types: Vec<SchemeType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeType {
    pub name: String,
    pub fields: Vec<SchemeField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeField {
    pub name: String,
    pub ty: SchemeFieldType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeFieldType {
    pub kind: SchemeFieldTypeKind,
    pub ty_ref: SchemeTypeRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemeFieldTypeKind {
    Normal,
    Stream,
    Sync,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemeTypeRef {
    Builtin(BuiltinType),
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BuiltinType {
    Integer,
    Float,
    String,
    Boolean,
}
pub(crate) fn create_module(ast: &ASTRoot) -> SchemeModule {
    let mut types = Vec::new();
    for block in &ast.blocks {
        match block {
            ASTRootBlock::TypeDef(type_def) => {
                types.push(SchemeType {
                    name: type_def.name.clone(),
                    fields: type_def
                        .fields
                        .iter()
                        .map(|x| SchemeField {
                            name: x.name.clone(),
                            ty: SchemeFieldType {
                                kind: match &x.ty.kind {
                                    ASTTypeKind::Normal => SchemeFieldTypeKind::Normal,
                                    ASTTypeKind::Stream => SchemeFieldTypeKind::Stream,
                                    ASTTypeKind::Sync => SchemeFieldTypeKind::Sync,
                                },
                                ty_ref: match x.ty.name.as_str() {
                                    "integer" => SchemeTypeRef::Builtin(BuiltinType::Integer),
                                    "float" => SchemeTypeRef::Builtin(BuiltinType::Float),
                                    "string" => SchemeTypeRef::Builtin(BuiltinType::String),
                                    "boolean" => SchemeTypeRef::Builtin(BuiltinType::Boolean),
                                    _ => SchemeTypeRef::Custom(x.ty.name.clone()),
                                },
                            },
                        })
                        .collect::<_>(),
                });
            }
        }
    }
    SchemeModule { types }
}