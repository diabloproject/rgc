use super::module::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assembly {
    pub types: Vec<Rc<RefCell<AssemblyType>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblyType {
    pub name: String,
    pub fields: Vec<AssemblyField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblyField {
    pub name: String,
    pub ty: AssemblyFieldType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblyFieldType {
    pub kind: AssemblyFieldTypeKind,
    pub ty_ref: AssemblyTypeRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblyFieldTypeKind {
    Normal,
    Streaming,
    Sync,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblyTypeRef {
    Builtin(BuiltinType),
    Custom(Rc<RefCell<AssemblyType>>),
}

pub fn generate(module: &SchemeModule) -> Assembly {
    let mut types: HashMap<String, (Rc<RefCell<AssemblyType>>, &SchemeType)> = HashMap::new();
    for type_def in &module.types {
        types.insert(
            type_def.name.clone(),
            (
                Rc::new(RefCell::new(AssemblyType {
                    name: type_def.name.clone(),
                    fields: vec![],
                })),
                type_def,
            ),
        );
    }

    for (_, (ty, type_def)) in types.iter() {
        for field in &type_def.fields {
            let ty_ref = match &field.ty.ty_ref {
                SchemeTypeRef::Builtin(ty_ref) => match &ty_ref {
                    &BuiltinType::Integer => AssemblyTypeRef::Builtin(BuiltinType::Integer),
                    &BuiltinType::Float => AssemblyTypeRef::Builtin(BuiltinType::Float),
                    &BuiltinType::String => AssemblyTypeRef::Builtin(BuiltinType::String),
                    &BuiltinType::Boolean => AssemblyTypeRef::Builtin(BuiltinType::Boolean),
                },
                SchemeTypeRef::Custom(ty_ref) => {
                    let ty = types.get(ty_ref).expect("Type not found").0.clone();
                    AssemblyTypeRef::Custom(ty)
                }
            };
            let f = AssemblyFieldType {
                kind: match field.ty.kind {
                    SchemeFieldTypeKind::Normal => AssemblyFieldTypeKind::Normal,
                    SchemeFieldTypeKind::Streaming => AssemblyFieldTypeKind::Streaming,
                    SchemeFieldTypeKind::Sync => AssemblyFieldTypeKind::Sync,
                },
                ty_ref,
            };
            (&mut (&*(ty.to_owned())).borrow_mut())
                .fields
                .push(AssemblyField {
                    name: field.name.clone(),
                    ty: f,
                });
        }
    }
    Assembly {
        types: types.iter().map(|(_, (ty, _))| ty.clone()).collect(),
    }
}