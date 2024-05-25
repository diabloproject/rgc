use super::*;

const SIMPLE_SCHEMA: &str = r#"
    type Post {
        id: integer,
        title: string,
        body: string,
        author: string
    }
"#;

const STREAMING_SCHEMA: &str = r#"
    type Post {
        id: integer,
        title: string,
        body: streaming string,
        author: string
    }
"#;

const SYNC_SCHEMA: &str = r#"
    type Post {
        id: integer,
        title: string,
        body: string,
        author: string,
        likes: sync integer
    }
"#;

const MULTIPLE_TYPES_SCHEMA: &str = r#"
    type Post {
        id: integer,
        title: string,
        body: string,
        author: User
    }
    type User {
        id: integer,
        name: string,
        email: string
    }
"#;

#[test]
fn test_simple_tokenize() {
    use schema::*;
    let tokens = tokenize(SIMPLE_SCHEMA);
    let target_tokens = vec![
        Token {
            ty: TokenType::Keyword,
            value: "type".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "Post".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: "{".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "id".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "integer".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "title".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "string".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "body".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "string".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "author".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "string".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: "}".to_string(),
        },
    ];

    for i in 0..tokens.len() {
        if tokens[i] != target_tokens[i] {
            let offset = if (i as i32 - 2) < 0 { 0 } else { i - 2 };
            let size = if i + 2 >= tokens.len() {
                tokens.len() - i
            } else {
                2
            };
            let context = &tokens[offset..i + size];

            println!("Context: {:?}", context);
            panic!(
                "Tokens are not equal at index {}: {:?} != {:?}",
                i, tokens[i], target_tokens[i]
            );
        }
    }
}

#[test]
fn test_streaming_tokenize() {
    use schema::*;
    let tokens = tokenize(STREAMING_SCHEMA);
    let target_tokens = vec![
        Token {
            ty: TokenType::Keyword,
            value: "type".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "Post".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: "{".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "id".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "integer".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "title".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "string".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "body".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Keyword,
            value: "streaming".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "string".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "author".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "string".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: "}".to_string(),
        },
    ];

    for i in 0..tokens.len() {
        if tokens[i] != target_tokens[i] {
            let offset = if (i as i32 - 2) < 0 { 0 } else { i - 2 };
            let size = if i + 2 >= tokens.len() {
                tokens.len() - i
            } else {
                2
            };
            let context = &tokens[offset..i + size];

            println!("Context: {:?}", context);
            panic!(
                "Tokens are not equal at index {}: {:?} != {:?}",
                i, tokens[i], target_tokens[i]
            );
        }
    }
}

#[test]
fn test_sync_tokenize() {
    use schema::*;
    let tokens = tokenize(SYNC_SCHEMA);
    let target_tokens = vec![
        Token {
            ty: TokenType::Keyword,
            value: "type".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "Post".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: "{".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "id".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "integer".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "title".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "string".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "body".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "string".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "author".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "string".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ",".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "likes".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: ":".to_string(),
        },
        Token {
            ty: TokenType::Keyword,
            value: "sync".to_string(),
        },
        Token {
            ty: TokenType::Identifier,
            value: "integer".to_string(),
        },
        Token {
            ty: TokenType::Punctuation,
            value: "}".to_string(),
        },
    ];

    for i in 0..tokens.len() {
        if tokens[i] != target_tokens[i] {
            let offset = if (i as i32 - 2) < 0 { 0 } else { i - 2 };
            let size = if i + 2 >= tokens.len() {
                tokens.len() - i
            } else {
                2
            };
            let context = &tokens[offset..i + size];

            println!("Context: {:?}", context);
            panic!(
                "Tokens are not equal at index {}: {:?} != {:?}",
                i, tokens[i], target_tokens[i]
            );
        }
    }
}

#[test]
fn test_simple_ast() {
    use schema::*;
    let tokens = tokenize(SIMPLE_SCHEMA);
    let ast = gen_ast(&tokens);
    println!("{:?}", tokens);
    println!("{:?}", ast);

    let target_ast = ASTRoot {
        blocks: vec![ASTRootBlock::TypeDef(ASTTypeDef {
            name: "Post".to_string(),
            fields: vec![
                ASTField {
                    name: "id".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "integer".to_string(),
                    },
                },
                ASTField {
                    name: "title".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "string".to_string(),
                    },
                },
                ASTField {
                    name: "body".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "string".to_string(),
                    },
                },
                ASTField {
                    name: "author".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "string".to_string(),
                    },
                },
            ],
        })],
    };

    assert_eq!(ast, target_ast);
}

#[test]
fn test_streaming_ast() {
    use schema::*;
    let tokens = tokenize(STREAMING_SCHEMA);
    let ast = gen_ast(&tokens);
    println!("{:?}", tokens);
    println!("{:?}", ast);

    let target_ast = ASTRoot {
        blocks: vec![ASTRootBlock::TypeDef(ASTTypeDef {
            name: "Post".to_string(),
            fields: vec![
                ASTField {
                    name: "id".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "integer".to_string(),
                    },
                },
                ASTField {
                    name: "title".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "string".to_string(),
                    },
                },
                ASTField {
                    name: "body".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Streaming,
                        name: "string".to_string(),
                    },
                },
                ASTField {
                    name: "author".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "string".to_string(),
                    },
                },
            ],
        })],
    };

    assert_eq!(ast, target_ast);
}

#[test]
fn test_sync_ast() {
    use schema::*;
    let tokens = tokenize(SYNC_SCHEMA);
    let ast = gen_ast(&tokens);
    println!("{:?}", tokens);
    println!("{:?}", ast);

    let target_ast = ASTRoot {
        blocks: vec![ASTRootBlock::TypeDef(ASTTypeDef {
            name: "Post".to_string(),
            fields: vec![
                ASTField {
                    name: "id".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "integer".to_string(),
                    },
                },
                ASTField {
                    name: "title".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "string".to_string(),
                    },
                },
                ASTField {
                    name: "body".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "string".to_string(),
                    },
                },
                ASTField {
                    name: "author".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Normal,
                        name: "string".to_string(),
                    },
                },
                ASTField {
                    name: "likes".to_string(),
                    ty: ASTType {
                        kind: ASTTypeKind::Sync,
                        name: "integer".to_string(),
                    },
                },
            ],
        })],
    };

    assert_eq!(ast, target_ast);
}

#[test]
fn test_multiple_types_ast() {
    use schema::*;
    let tokens = tokenize(MULTIPLE_TYPES_SCHEMA);
    let ast = gen_ast(&tokens);
    println!("{:?}", tokens);
    println!("{:?}", ast);

    let target_ast = ASTRoot {
        blocks: vec![
            ASTRootBlock::TypeDef(ASTTypeDef {
                name: "Post".to_string(),
                fields: vec![
                    ASTField {
                        name: "id".to_string(),
                        ty: ASTType {
                            kind: ASTTypeKind::Normal,
                            name: "integer".to_string(),
                        },
                    },
                    ASTField {
                        name: "title".to_string(),
                        ty: ASTType {
                            kind: ASTTypeKind::Normal,
                            name: "string".to_string(),
                        },
                    },
                    ASTField {
                        name: "body".to_string(),
                        ty: ASTType {
                            kind: ASTTypeKind::Normal,
                            name: "string".to_string(),
                        },
                    },
                    ASTField {
                        name: "author".to_string(),
                        ty: ASTType {
                            kind: ASTTypeKind::Normal,
                            name: "User".to_string(),
                        },
                    },
                ],
            }),
            ASTRootBlock::TypeDef(ASTTypeDef {
                name: "User".to_string(),
                fields: vec![
                    ASTField {
                        name: "id".to_string(),
                        ty: ASTType {
                            kind: ASTTypeKind::Normal,
                            name: "integer".to_string(),
                        },
                    },
                    ASTField {
                        name: "name".to_string(),
                        ty: ASTType {
                            kind: ASTTypeKind::Normal,
                            name: "string".to_string(),
                        },
                    },
                    ASTField {
                        name: "email".to_string(),
                        ty: ASTType {
                            kind: ASTTypeKind::Normal,
                            name: "string".to_string(),
                        },
                    },
                ],
            }),
        ],
    };

    assert_eq!(ast, target_ast);
}

#[test]
fn test_simple_module() {
    use schema::*;
    let tokens = tokenize(SIMPLE_SCHEMA);
    let ast = gen_ast(&tokens);
    let module = create_module(&ast);
    println!("{:?}", module);

    assert_eq!(
        module,
        SchemeModule {
            types: vec![SchemeType {
                name: "Post".to_string(),
                fields: vec![
                    SchemeField {
                        name: "id".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::Integer),
                        },
                    },
                    SchemeField {
                        name: "title".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                        },
                    },
                    SchemeField {
                        name: "body".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                        },
                    },
                    SchemeField {
                        name: "author".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                        },
                    },
                ],
            }],
        }
    );
}

#[test]
fn test_streaming_module() {
    use schema::*;
    let tokens = tokenize(STREAMING_SCHEMA);
    let ast = gen_ast(&tokens);
    let module = create_module(&ast);
    println!("{:?}", module);

    assert_eq!(
        module,
        SchemeModule {
            types: vec![SchemeType {
                name: "Post".to_string(),
                fields: vec![
                    SchemeField {
                        name: "id".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::Integer),
                        },
                    },
                    SchemeField {
                        name: "title".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                        },
                    },
                    SchemeField {
                        name: "body".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Streaming,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                        },
                    },
                    SchemeField {
                        name: "author".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                        },
                    },
                ],
            }],
        }
    );
}

#[test]
fn test_sync_module() {
    use schema::*;
    let tokens = tokenize(SYNC_SCHEMA);
    let ast = gen_ast(&tokens);
    let module = create_module(&ast);
    println!("{:?}", module);

    assert_eq!(
        module,
        SchemeModule {
            types: vec![SchemeType {
                name: "Post".to_string(),
                fields: vec![
                    SchemeField {
                        name: "id".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::Integer),
                        },
                    },
                    SchemeField {
                        name: "title".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                        },
                    },
                    SchemeField {
                        name: "body".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                        },
                    },
                    SchemeField {
                        name: "author".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Normal,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                        },
                    },
                    SchemeField {
                        name: "likes".to_string(),
                        ty: SchemeFieldType {
                            kind: SchemeFieldTypeKind::Sync,
                            ty_ref: SchemeTypeRef::Builtin(BuiltinType::Integer),
                        },
                    },
                ],
            }],
        }
    );
}

#[test]
fn test_multiple_types_module() {
    use schema::*;
    let tokens = tokenize(MULTIPLE_TYPES_SCHEMA);
    let ast = gen_ast(&tokens);
    let module = create_module(&ast);
    println!("{:?}", module);

    assert_eq!(
        module,
        SchemeModule {
            types: vec![
                SchemeType {
                    name: "Post".to_string(),
                    fields: vec![
                        SchemeField {
                            name: "id".to_string(),
                            ty: SchemeFieldType {
                                kind: SchemeFieldTypeKind::Normal,
                                ty_ref: SchemeTypeRef::Builtin(BuiltinType::Integer),
                            },
                        },
                        SchemeField {
                            name: "title".to_string(),
                            ty: SchemeFieldType {
                                kind: SchemeFieldTypeKind::Normal,
                                ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                            },
                        },
                        SchemeField {
                            name: "body".to_string(),
                            ty: SchemeFieldType {
                                kind: SchemeFieldTypeKind::Normal,
                                ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                            },
                        },
                        SchemeField {
                            name: "author".to_string(),
                            ty: SchemeFieldType {
                                kind: SchemeFieldTypeKind::Normal,
                                ty_ref: SchemeTypeRef::Custom("User".to_string()),
                            },
                        },
                    ],
                },
                SchemeType {
                    name: "User".to_string(),
                    fields: vec![
                        SchemeField {
                            name: "id".to_string(),
                            ty: SchemeFieldType {
                                kind: SchemeFieldTypeKind::Normal,
                                ty_ref: SchemeTypeRef::Builtin(BuiltinType::Integer),
                            },
                        },
                        SchemeField {
                            name: "name".to_string(),
                            ty: SchemeFieldType {
                                kind: SchemeFieldTypeKind::Normal,
                                ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                            },
                        },
                        SchemeField {
                            name: "email".to_string(),
                            ty: SchemeFieldType {
                                kind: SchemeFieldTypeKind::Normal,
                                ty_ref: SchemeTypeRef::Builtin(BuiltinType::String),
                            },
                        },
                    ],
                },
            ],
        }
    );
}

#[test]
fn test_multiple_types_assembly() {
    use schema::*;
    let tokens = tokenize(MULTIPLE_TYPES_SCHEMA);
    let ast = gen_ast(&tokens);
    let module = create_module(&ast);
    let assembly = generate(&module);
    println!("{:?}", assembly);
}