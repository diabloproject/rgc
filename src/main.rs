fn main() {
    println!("Hello, world!");
}

pub mod codegen {
    pub mod rust {
        use std::fmt::{Display, Formatter};

        pub struct Types {}

        impl Display for Types {
            fn fmt(&self, _f: &mut Formatter) -> std::fmt::Result {
                todo!()
            }
        }
    }
    pub mod typescript {
        use std::fmt::{Display, Formatter};

        pub struct Types {}

        impl Display for Types {
            fn fmt(&self, _f: &mut Formatter) -> std::fmt::Result {
                todo!()
            }
        }
    }
}
pub mod schema {
    pub fn parse(schema: &str) -> Assembly {
        let tokens = tokenize(schema);
        let ast = gen_ast(&tokens);
        let module = create_module(&ast);
        let assembly = generate(&module);
        return assembly;
    }

    pub(crate) fn tokenize(schema: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_token = Token::new();
        for c in schema.chars() {
            match c {
                ' ' | '\t' | '\n' => {
                    if !current_token.is_empty() {
                        current_token.finish();
                        tokens.push(current_token);
                        current_token = Token::new();
                    }
                }
                '{' | '}' | '.' | ',' | ':' | ';' => {
                    if !current_token.is_empty() {
                        current_token.finish();
                        tokens.push(current_token);
                        current_token = Token::new();
                    }
                    current_token.push(c);
                    current_token.finish();
                    tokens.push(current_token);
                    current_token = Token::new();
                }
                _ => {
                    current_token.push(c);
                }
            }
        }
        if !current_token.is_empty() {
            current_token.finish();
            tokens.push(current_token);
        }
        tokens
    }
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

    const KEYWORDS: &[&str] = &["type", "streaming", "sync"];
    const PUNCTUATIONS: &[char] = &['{', '}', '.', ',', ':', ';'];

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum TokenType {
        Keyword,
        Identifier,
        Punctuation,
        Defer,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct Token {
        pub ty: TokenType,
        pub value: String,
    }

    impl Token {
        fn new() -> Self {
            Self {
                ty: TokenType::Defer,
                value: String::new(),
            }
        }
        fn push(&mut self, c: char) {
            self.value.push(c);
        }
        fn is_empty(&self) -> bool {
            self.value.is_empty()
        }
        fn finish(&mut self) {
            if self.value.is_empty() {
                panic!("Token is empty");
            }
            if self.ty != TokenType::Defer {
                panic!("Token has been finished already");
            }
            if KEYWORDS.contains(&self.value.as_str()) {
                self.ty = TokenType::Keyword;
            } else if PUNCTUATIONS.contains(&self.value.chars().next().unwrap()) {
                if self.value.len() != 1 {
                    panic!("Punctuation token must be a single character");
                }
                self.ty = TokenType::Punctuation;
            } else {
                self.ty = TokenType::Identifier;
            }
        }
    }

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
        Streaming,
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
                                        ASTTypeKind::Streaming => SchemeFieldTypeKind::Streaming,
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

    use std::borrow::BorrowMut;
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
}

#[cfg(test)]
pub mod tests {
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
}
