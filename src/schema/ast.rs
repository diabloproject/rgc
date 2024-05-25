use super::tokenize::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ASTRoot {
    pub blocks: Vec<ASTRootBlock>,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ASTRootBlock {
    TypeDef(ASTTypeDef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ASTTypeDef {
    pub name: String,
    pub fields: Vec<ASTField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ASTField {
    pub name: String,
    pub ty: ASTType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ASTType {
    pub kind: ASTTypeKind,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ASTTypeKind {
    Normal,
    Streaming,
    Sync,
}

pub(crate) fn gen_ast(tokens: &[Token]) -> ASTRoot {
    let mut blocks = Vec::new();
    let mut current_block: Option<ASTRootBlock> = None;
    let mut i = 0;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum State {
        Root,
        TypeDefExpectingName,
        TypeDefExpectingBlock,
        TypeDefBlock,
        TypeDefBlockFieldExpectingColon,
        TypeDefBlockFieldExpectingTypeKind,
        TypeDefBlockFieldExpectingTypeName,
        TypeDefBlockFieldExpectingComma,
    }

    let mut state = State::Root;

    loop {
        if i >= tokens.len() {
            break;
        }
        let token = &tokens[i];

        match state {
            State::Root => {
                if token.ty == TokenType::Keyword && token.value == "type" {
                    state = State::TypeDefExpectingName;
                    i += 1;
                } else {
                    panic!("Unexpected token: {:?}", token);
                }
            }
            State::TypeDefExpectingName => {
                if token.ty == TokenType::Identifier {
                    current_block = Some(ASTRootBlock::TypeDef(ASTTypeDef {
                        name: token.value.clone(),
                        fields: Vec::new(),
                    }));
                    state = State::TypeDefExpectingBlock;
                    i += 1;
                } else {
                    panic!("Unexpected token: {:?}", token);
                }
            }
            State::TypeDefExpectingBlock => {
                if token.ty == TokenType::Punctuation && token.value == "{" {
                    state = State::TypeDefBlock;
                    i += 1;
                } else {
                    panic!("Unexpected token: {:?}", token);
                }
            }
            State::TypeDefBlock => {
                if token.ty == TokenType::Punctuation && token.value == "}" {
                    state = State::Root;
                    i += 1;
                } else if token.ty == TokenType::Identifier {
                    #[allow(irrefutable_let_patterns)]
                    let Some(ASTRootBlock::TypeDef(ref mut typedef)) = current_block.as_mut() else {
                        panic!("!!!BUG!!! Current parser state (TypeDefBlock) implies that the current block is a TypeDef, but it is not a TypeDef");
                    };
                    state = State::TypeDefBlockFieldExpectingColon;
                    let field = ASTField {
                        name: token.value.clone(),
                        ty: ASTType {
                            kind: ASTTypeKind::Normal,
                            name: "".to_string(),
                        },
                    };
                    typedef.fields.push(field);
                    i += 1;
                } else {
                    panic!("Unexpected token: {:?}", token);
                }
            }
            State::TypeDefBlockFieldExpectingColon => {
                if token.ty == TokenType::Punctuation && token.value == ":" {
                    state = State::TypeDefBlockFieldExpectingTypeKind;
                    i += 1;
                } else {
                    panic!("Unexpected token: {:?}", token);
                }
            }
            State::TypeDefBlockFieldExpectingTypeKind => {
                match (&token.ty, token.value.as_str()) {
                    (&TokenType::Keyword, "normal") => {
                        state = State::TypeDefBlockFieldExpectingTypeName;
                        let Some(ASTRootBlock::TypeDef(ref mut typedef)) =
                            current_block.as_mut()
                        else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeKind) implies that the current block is a TypeDef, but it is not a TypeDef");
                        };
                        let Some(field) = typedef.fields.last_mut() else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeKind) implies that the current block contains at least one field, but it does not");
                        };
                        field.ty.kind = ASTTypeKind::Normal;
                        i += 1;
                    }
                    (&TokenType::Keyword, "streaming") => {
                        state = State::TypeDefBlockFieldExpectingTypeName;
                        let Some(ASTRootBlock::TypeDef(ref mut typedef)) =
                            current_block.as_mut()
                        else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeKind) implies that the current block is a TypeDef, but it is not a TypeDef");
                        };
                        let Some(field) = typedef.fields.last_mut() else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeKind) implies that the current block contains at least one field, but it does not");
                        };
                        field.ty.kind = ASTTypeKind::Streaming;
                        i += 1;
                    }
                    (&TokenType::Keyword, "sync") => {
                        state = State::TypeDefBlockFieldExpectingTypeName;
                        let Some(ASTRootBlock::TypeDef(ref mut typedef)) =
                            current_block.as_mut()
                        else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeKind) implies that the current block is a TypeDef, but it is not a TypeDef");
                        };
                        let Some(field) = typedef.fields.last_mut() else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeKind) implies that the current block contains at least one field, but it does not");
                        };
                        field.ty.kind = ASTTypeKind::Sync;

                        i += 1;
                    }
                    (&TokenType::Identifier, _) => {
                        state = State::TypeDefBlockFieldExpectingComma;
                        let Some(ASTRootBlock::TypeDef(ref mut typedef)) =
                            current_block.as_mut()
                        else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeKind) implies that the current block is a TypeDef, but it is not a TypeDef");
                        };
                        let Some(field) = typedef.fields.last_mut() else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeKind) implies that the current block contains at least one field, but it does not");
                        };
                        field.ty.kind = ASTTypeKind::Normal;
                        field.ty.name = token.value.clone();
                        i += 1;
                    }

                    (ty, value) => panic!(
                        "Unexpected token: {:?}",
                        Token {
                            ty: ty.clone(),
                            value: value.to_string(),
                        }
                    ),
                }
            }
            State::TypeDefBlockFieldExpectingTypeName => {
                match (&token.ty, token.value.as_str()) {
                    (&TokenType::Identifier, typename) => {
                        state = State::TypeDefBlockFieldExpectingComma;
                        let Some(ASTRootBlock::TypeDef(ref mut typedef)) =
                            current_block.as_mut()
                        else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeName) implies that the current block is a TypeDef, but it is not a TypeDef");
                        };
                        let Some(field) = typedef.fields.last_mut() else {
                            panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingTypeName) implies that the current block contains at least one field, but it does not");
                        };
                        field.ty.name = typename.to_string();
                        i += 1;
                    }
                    (ty, value) => panic!(
                        "Unexpected token: {:?}",
                        Token {
                            ty: ty.clone(),
                            value: value.to_string(),
                        }
                    ),
                }
            }
            State::TypeDefBlockFieldExpectingComma => match (&token.ty, token.value.as_str()) {
                (&TokenType::Punctuation, ",") => {
                    state = State::TypeDefBlock;
                    i += 1;
                }
                (&TokenType::Punctuation, "}") => {
                    state = State::Root;
                    let Some(block) = current_block else {
                        panic!("!!!BUG!!! Current parser state (TypeDefBlockFieldExpectingComma) implies that the current block is ASTTypeDef, but it is not TypeDef");
                    };
                    blocks.push(block);
                    current_block = None;
                    i += 1;
                }
                (ty, value) => panic!(
                    "Unexpected token: {:?}",
                    Token {
                        ty: ty.clone(),
                        value: value.to_string(),
                    }
                ),
            },
        }
    }
    if state != State::Root {
        panic!("Unexpected EOF");
    }
    ASTRoot { blocks }
}