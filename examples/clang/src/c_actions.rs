use super::c::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type integer_suffix_opt = String;
pub fn integer_suffix_opt(_ctx: &Ctx, token: Token) -> integer_suffix_opt {
    token.value.into()
}
pub type int_const = String;
pub fn int_const(_ctx: &Ctx, token: Token) -> int_const {
    token.value.into()
}
pub type float_const = String;
pub fn float_const(_ctx: &Ctx, token: Token) -> float_const {
    token.value.into()
}
pub type char_const = String;
pub fn char_const(_ctx: &Ctx, token: Token) -> char_const {
    token.value.into()
}
pub type hex_const = String;
pub fn hex_const(_ctx: &Ctx, token: Token) -> hex_const {
    token.value.into()
}
pub type string = String;
pub fn string(_ctx: &Ctx, token: Token) -> string {
    token.value.into()
}
pub type id = String;
pub fn id(_ctx: &Ctx, token: Token) -> id {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct translation_unit {
    pub external_decls: external_decl1,
}
pub fn translation_unit_c1(_ctx: &Ctx, external_decls: external_decl1) -> translation_unit {
    translation_unit { external_decls }
}
pub type external_decl1 = Vec<external_decl>;
pub fn external_decl1_c1(
    _ctx: &Ctx,
    mut external_decl1: external_decl1,
    external_decl: external_decl,
) -> external_decl1 {
    external_decl1.push(external_decl);
    external_decl1
}
pub fn external_decl1_external_decl(_ctx: &Ctx, external_decl: external_decl) -> external_decl1 {
    vec![external_decl]
}
#[derive(Debug, Clone)]
pub struct external_declC1 {
    pub function: function_definition,
}
#[derive(Debug, Clone)]
pub struct external_declC2 {
    pub declaration: Box<decl>,
}
#[derive(Debug, Clone)]
pub struct external_declC3 {
    pub ld: Box<line_directive>,
}
#[derive(Debug, Clone)]
pub enum external_decl {
    C1(external_declC1),
    C2(external_declC2),
    C3(external_declC3),
}
pub fn external_decl_c1(_ctx: &Ctx, function: function_definition) -> external_decl {
    external_decl::C1(external_declC1 { function })
}
pub fn external_decl_c2(_ctx: &Ctx, declaration: decl) -> external_decl {
    external_decl::C2(external_declC2 {
        declaration: Box::new(declaration),
    })
}
pub fn external_decl_c3(_ctx: &Ctx, ld: line_directive) -> external_decl {
    external_decl::C3(external_declC3 { ld: Box::new(ld) })
}
#[derive(Debug, Clone)]
pub struct line_directive {
    pub int_const: Box<int_const>,
    pub string: Box<string>,
    pub line_directive_int_opt: line_directive_intOpt,
}
pub fn line_directive_c1(
    _ctx: &Ctx,
    int_const: int_const,
    string: string,
    line_directive_int_opt: line_directive_intOpt,
) -> line_directive {
    line_directive {
        int_const: Box::new(int_const),
        string: Box::new(string),
        line_directive_int_opt,
    }
}
pub type line_directive_intOpt = Option<line_directive_int>;
pub fn line_directive_int_opt_line_directive_int(
    _ctx: &Ctx,
    line_directive_int: line_directive_int,
) -> line_directive_intOpt {
    Some(line_directive_int)
}
pub fn line_directive_int_opt_empty(_ctx: &Ctx) -> line_directive_intOpt {
    None
}
#[derive(Debug, Clone)]
pub struct line_directive_intC2 {
    pub line_directive_int: Box<line_directive_int>,
    pub int_const: Box<int_const>,
}
#[derive(Debug, Clone)]
pub enum line_directive_int {
    int_const(Box<int_const>),
    C2(line_directive_intC2),
}
pub fn line_directive_int_int_const(_ctx: &Ctx, int_const: int_const) -> line_directive_int {
    line_directive_int::int_const(Box::new(int_const))
}
pub fn line_directive_int_c2(
    _ctx: &Ctx,
    line_directive_int: line_directive_int,
    int_const: int_const,
) -> line_directive_int {
    line_directive_int::C2(line_directive_intC2 {
        line_directive_int: Box::new(line_directive_int),
        int_const: Box::new(int_const),
    })
}
#[derive(Debug, Clone)]
pub struct function_definitionC1 {
    pub decl_specs: decl_specs,
    pub declarator: Box<declarator>,
    pub decl_list: decl_list,
    pub body: compound_stat,
}
#[derive(Debug, Clone)]
pub struct function_definitionC2 {
    pub declarator: Box<declarator>,
    pub decl_list: Box<decl_list>,
    pub body: Box<compound_stat>,
}
#[derive(Debug, Clone)]
pub struct function_definitionC3 {
    pub decl_specs: Box<decl_specs>,
    pub declarator: Box<declarator>,
    pub body: Box<compound_stat>,
}
#[derive(Debug, Clone)]
pub struct function_definitionC4 {
    pub declarator: Box<declarator>,
    pub body: Box<compound_stat>,
}
#[derive(Debug, Clone)]
pub enum function_definition {
    C1(function_definitionC1),
    C2(function_definitionC2),
    C3(function_definitionC3),
    C4(function_definitionC4),
}
pub fn function_definition_c1(
    _ctx: &Ctx,
    decl_specs: decl_specs,
    declarator: declarator,
    decl_list: decl_list,
    body: compound_stat,
) -> function_definition {
    function_definition::C1(function_definitionC1 {
        decl_specs,
        declarator: Box::new(declarator),
        decl_list,
        body,
    })
}
pub fn function_definition_c2(
    _ctx: &Ctx,
    declarator: declarator,
    decl_list: decl_list,
    body: compound_stat,
) -> function_definition {
    function_definition::C2(function_definitionC2 {
        declarator: Box::new(declarator),
        decl_list: Box::new(decl_list),
        body: Box::new(body),
    })
}
pub fn function_definition_c3(
    _ctx: &Ctx,
    decl_specs: decl_specs,
    declarator: declarator,
    body: compound_stat,
) -> function_definition {
    function_definition::C3(function_definitionC3 {
        decl_specs: Box::new(decl_specs),
        declarator: Box::new(declarator),
        body: Box::new(body),
    })
}
pub fn function_definition_c4(
    _ctx: &Ctx,
    declarator: declarator,
    body: compound_stat,
) -> function_definition {
    function_definition::C4(function_definitionC4 {
        declarator: Box::new(declarator),
        body: Box::new(body),
    })
}
pub type decl = decl_body;
pub fn decl_decl_body(_ctx: &Ctx, decl_body: decl_body) -> decl {
    decl_body
}
#[derive(Debug, Clone)]
pub struct decl_body {
    pub decl_specs: Box<decl_specs>,
    pub init_decl_list: init_declarator_listOpt,
}
pub fn decl_body_c1(
    _ctx: &Ctx,
    decl_specs: decl_specs,
    init_decl_list: init_declarator_listOpt,
) -> decl_body {
    decl_body {
        decl_specs: Box::new(decl_specs),
        init_decl_list,
    }
}
pub type init_declarator_listOpt = Option<init_declarator_list>;
pub fn init_declarator_list_opt_init_declarator_list(
    _ctx: &Ctx,
    init_declarator_list: init_declarator_list,
) -> init_declarator_listOpt {
    Some(init_declarator_list)
}
pub fn init_declarator_list_opt_empty(_ctx: &Ctx) -> init_declarator_listOpt {
    None
}
pub type decl_list = decl1;
pub fn decl_list_decl1(_ctx: &Ctx, decl1: decl1) -> decl_list {
    decl1
}
pub type decl1 = Vec<decl>;
pub fn decl1_c1(_ctx: &Ctx, mut decl1: decl1, decl: decl) -> decl1 {
    decl1.push(decl);
    decl1
}
pub fn decl1_decl(_ctx: &Ctx, decl: decl) -> decl1 {
    vec![decl]
}
#[derive(Debug, Clone)]
pub struct decl_specsC1 {
    pub decl_specs: Box<decl_specs>,
    pub decl_spec: decl_spec,
}
#[derive(Debug, Clone)]
pub enum decl_specs {
    C1(decl_specsC1),
    decl_spec(Box<decl_spec>),
}
pub fn decl_specs_c1(_ctx: &Ctx, decl_specs: decl_specs, decl_spec: decl_spec) -> decl_specs {
    decl_specs::C1(decl_specsC1 {
        decl_specs: Box::new(decl_specs),
        decl_spec,
    })
}
pub fn decl_specs_decl_spec(_ctx: &Ctx, decl_spec: decl_spec) -> decl_specs {
    decl_specs::decl_spec(Box::new(decl_spec))
}
#[derive(Debug, Clone)]
pub struct decl_specC1 {
    pub storage_spec: storage_class_spec,
}
#[derive(Debug, Clone)]
pub struct decl_specC2 {
    pub type_spec: type_spec,
}
#[derive(Debug, Clone)]
pub struct decl_specC3 {
    pub type_qual: Box<type_qualifier>,
}
#[derive(Debug, Clone)]
pub enum decl_spec {
    C1(decl_specC1),
    C2(decl_specC2),
    C3(decl_specC3),
}
pub fn decl_spec_c1(_ctx: &Ctx, storage_spec: storage_class_spec) -> decl_spec {
    decl_spec::C1(decl_specC1 { storage_spec })
}
pub fn decl_spec_c2(_ctx: &Ctx, type_spec: type_spec) -> decl_spec {
    decl_spec::C2(decl_specC2 { type_spec })
}
pub fn decl_spec_c3(_ctx: &Ctx, type_qual: type_qualifier) -> decl_spec {
    decl_spec::C3(decl_specC3 {
        type_qual: Box::new(type_qual),
    })
}
#[derive(Debug, Clone)]
pub enum storage_class_spec {
    autokw,
    registerkw,
    statickw,
    externkw,
    typedefkw,
}
pub fn storage_class_spec_autokw(_ctx: &Ctx) -> storage_class_spec {
    storage_class_spec::autokw
}
pub fn storage_class_spec_registerkw(_ctx: &Ctx) -> storage_class_spec {
    storage_class_spec::registerkw
}
pub fn storage_class_spec_statickw(_ctx: &Ctx) -> storage_class_spec {
    storage_class_spec::statickw
}
pub fn storage_class_spec_externkw(_ctx: &Ctx) -> storage_class_spec {
    storage_class_spec::externkw
}
pub fn storage_class_spec_typedefkw(_ctx: &Ctx) -> storage_class_spec {
    storage_class_spec::typedefkw
}
#[derive(Debug, Clone)]
pub enum type_spec {
    voidt,
    chart,
    shortt,
    intt,
    longt,
    floatt,
    doublet,
    signedt,
    unsignedt,
    boolt,
    complext,
    struct_or_union_spec(struct_or_union_spec),
    enum_spec(enum_spec),
    typedef_name(typedef_name),
}
pub fn type_spec_voidt(_ctx: &Ctx) -> type_spec {
    type_spec::voidt
}
pub fn type_spec_chart(_ctx: &Ctx) -> type_spec {
    type_spec::chart
}
pub fn type_spec_shortt(_ctx: &Ctx) -> type_spec {
    type_spec::shortt
}
pub fn type_spec_intt(_ctx: &Ctx) -> type_spec {
    type_spec::intt
}
pub fn type_spec_longt(_ctx: &Ctx) -> type_spec {
    type_spec::longt
}
pub fn type_spec_floatt(_ctx: &Ctx) -> type_spec {
    type_spec::floatt
}
pub fn type_spec_doublet(_ctx: &Ctx) -> type_spec {
    type_spec::doublet
}
pub fn type_spec_signedt(_ctx: &Ctx) -> type_spec {
    type_spec::signedt
}
pub fn type_spec_unsignedt(_ctx: &Ctx) -> type_spec {
    type_spec::unsignedt
}
pub fn type_spec_boolt(_ctx: &Ctx) -> type_spec {
    type_spec::boolt
}
pub fn type_spec_complext(_ctx: &Ctx) -> type_spec {
    type_spec::complext
}
pub fn type_spec_struct_or_union_spec(
    _ctx: &Ctx,
    struct_or_union_spec: struct_or_union_spec,
) -> type_spec {
    type_spec::struct_or_union_spec(struct_or_union_spec)
}
pub fn type_spec_enum_spec(_ctx: &Ctx, enum_spec: enum_spec) -> type_spec {
    type_spec::enum_spec(enum_spec)
}
pub fn type_spec_typedef_name(_ctx: &Ctx, typedef_name: typedef_name) -> type_spec {
    type_spec::typedef_name(typedef_name)
}
#[derive(Debug, Clone)]
pub enum type_qualifier {
    constt,
    volatilet,
}
pub fn type_qualifier_constt(_ctx: &Ctx) -> type_qualifier {
    type_qualifier::constt
}
pub fn type_qualifier_volatilet(_ctx: &Ctx) -> type_qualifier {
    type_qualifier::volatilet
}
#[derive(Debug, Clone)]
pub struct struct_or_union_specC1 {
    pub struct_type: struct_or_union,
    pub id: id,
    pub fields: struct_decl1,
}
#[derive(Debug, Clone)]
pub struct struct_or_union_specC2 {
    pub struct_type: Box<struct_or_union>,
    pub fields: Box<struct_decl1>,
}
#[derive(Debug, Clone)]
pub struct struct_or_union_specC3 {
    pub struct_type: Box<struct_or_union>,
    pub id: Box<id>,
}
#[derive(Debug, Clone)]
pub enum struct_or_union_spec {
    C1(struct_or_union_specC1),
    C2(struct_or_union_specC2),
    C3(struct_or_union_specC3),
}
pub fn struct_or_union_spec_c1(
    _ctx: &Ctx,
    struct_type: struct_or_union,
    id: id,
    fields: struct_decl1,
) -> struct_or_union_spec {
    struct_or_union_spec::C1(struct_or_union_specC1 {
        struct_type,
        id,
        fields,
    })
}
pub fn struct_or_union_spec_c2(
    _ctx: &Ctx,
    struct_type: struct_or_union,
    fields: struct_decl1,
) -> struct_or_union_spec {
    struct_or_union_spec::C2(struct_or_union_specC2 {
        struct_type: Box::new(struct_type),
        fields: Box::new(fields),
    })
}
pub fn struct_or_union_spec_c3(
    _ctx: &Ctx,
    struct_type: struct_or_union,
    id: id,
) -> struct_or_union_spec {
    struct_or_union_spec::C3(struct_or_union_specC3 {
        struct_type: Box::new(struct_type),
        id: Box::new(id),
    })
}
pub type struct_decl1 = Vec<struct_decl>;
pub fn struct_decl1_c1(
    _ctx: &Ctx,
    mut struct_decl1: struct_decl1,
    struct_decl: struct_decl,
) -> struct_decl1 {
    struct_decl1.push(struct_decl);
    struct_decl1
}
pub fn struct_decl1_struct_decl(_ctx: &Ctx, struct_decl: struct_decl) -> struct_decl1 {
    vec![struct_decl]
}
#[derive(Debug, Clone)]
pub enum struct_or_union {
    structkw,
    unionkw,
    classkw,
}
pub fn struct_or_union_structkw(_ctx: &Ctx) -> struct_or_union {
    struct_or_union::structkw
}
pub fn struct_or_union_unionkw(_ctx: &Ctx) -> struct_or_union {
    struct_or_union::unionkw
}
pub fn struct_or_union_classkw(_ctx: &Ctx) -> struct_or_union {
    struct_or_union::classkw
}
pub type init_declarator_list = init_declarator1;
pub fn init_declarator_list_init_declarator1(
    _ctx: &Ctx,
    init_declarator1: init_declarator1,
) -> init_declarator_list {
    init_declarator1
}
pub type init_declarator1 = Vec<init_declarator>;
pub fn init_declarator1_c1(
    _ctx: &Ctx,
    mut init_declarator1: init_declarator1,
    init_declarator: init_declarator,
) -> init_declarator1 {
    init_declarator1.push(init_declarator);
    init_declarator1
}
pub fn init_declarator1_init_declarator(
    _ctx: &Ctx,
    init_declarator: init_declarator,
) -> init_declarator1 {
    vec![init_declarator]
}
#[derive(Debug, Clone)]
pub struct init_declaratorC1 {
    pub decl: Box<declarator>,
}
#[derive(Debug, Clone)]
pub struct init_declaratorC2 {
    pub decl: Box<declarator>,
    pub init: initializer,
}
#[derive(Debug, Clone)]
pub enum init_declarator {
    C1(init_declaratorC1),
    C2(init_declaratorC2),
}
pub fn init_declarator_c1(_ctx: &Ctx, decl: declarator) -> init_declarator {
    init_declarator::C1(init_declaratorC1 {
        decl: Box::new(decl),
    })
}
pub fn init_declarator_c2(_ctx: &Ctx, decl: declarator, init: initializer) -> init_declarator {
    init_declarator::C2(init_declaratorC2 {
        decl: Box::new(decl),
        init,
    })
}
#[derive(Debug, Clone)]
pub struct struct_declC1 {
    pub spec_qualifier_list: spec_qualifier_list,
    pub struct_declarator1: struct_declarator1,
}
#[derive(Debug, Clone)]
pub enum struct_decl {
    C1(struct_declC1),
    line_directive(line_directive),
}
pub fn struct_decl_c1(
    _ctx: &Ctx,
    spec_qualifier_list: spec_qualifier_list,
    struct_declarator1: struct_declarator1,
) -> struct_decl {
    struct_decl::C1(struct_declC1 {
        spec_qualifier_list,
        struct_declarator1,
    })
}
pub fn struct_decl_line_directive(_ctx: &Ctx, line_directive: line_directive) -> struct_decl {
    struct_decl::line_directive(line_directive)
}
pub type struct_declarator1 = Vec<struct_declarator>;
pub fn struct_declarator1_c1(
    _ctx: &Ctx,
    mut struct_declarator1: struct_declarator1,
    struct_declarator: struct_declarator,
) -> struct_declarator1 {
    struct_declarator1.push(struct_declarator);
    struct_declarator1
}
pub fn struct_declarator1_struct_declarator(
    _ctx: &Ctx,
    struct_declarator: struct_declarator,
) -> struct_declarator1 {
    vec![struct_declarator]
}
#[derive(Debug, Clone)]
pub struct spec_qualifier_listC1 {
    pub type_spec: Box<type_spec>,
    pub spec_qualifier_list: Box<spec_qualifier_list>,
}
#[derive(Debug, Clone)]
pub struct spec_qualifier_listC3 {
    pub type_qualifier: type_qualifier,
    pub spec_qualifier_list: Box<spec_qualifier_list>,
}
#[derive(Debug, Clone)]
pub enum spec_qualifier_list {
    C1(spec_qualifier_listC1),
    type_spec(Box<type_spec>),
    C3(spec_qualifier_listC3),
    type_qualifier(Box<type_qualifier>),
}
pub fn spec_qualifier_list_c1(
    _ctx: &Ctx,
    type_spec: type_spec,
    spec_qualifier_list: spec_qualifier_list,
) -> spec_qualifier_list {
    spec_qualifier_list::C1(spec_qualifier_listC1 {
        type_spec: Box::new(type_spec),
        spec_qualifier_list: Box::new(spec_qualifier_list),
    })
}
pub fn spec_qualifier_list_type_spec(_ctx: &Ctx, type_spec: type_spec) -> spec_qualifier_list {
    spec_qualifier_list::type_spec(Box::new(type_spec))
}
pub fn spec_qualifier_list_c3(
    _ctx: &Ctx,
    type_qualifier: type_qualifier,
    spec_qualifier_list: spec_qualifier_list,
) -> spec_qualifier_list {
    spec_qualifier_list::C3(spec_qualifier_listC3 {
        type_qualifier,
        spec_qualifier_list: Box::new(spec_qualifier_list),
    })
}
pub fn spec_qualifier_list_type_qualifier(
    _ctx: &Ctx,
    type_qualifier: type_qualifier,
) -> spec_qualifier_list {
    spec_qualifier_list::type_qualifier(Box::new(type_qualifier))
}
#[derive(Debug, Clone)]
pub struct struct_declaratorC2 {
    pub declarator: Box<declarator>,
    pub const_exp: Box<const_exp>,
}
#[derive(Debug, Clone)]
pub enum struct_declarator {
    declarator(declarator),
    C2(struct_declaratorC2),
    const_exp(Box<const_exp>),
}
pub fn struct_declarator_declarator(_ctx: &Ctx, declarator: declarator) -> struct_declarator {
    struct_declarator::declarator(declarator)
}
pub fn struct_declarator_c2(
    _ctx: &Ctx,
    declarator: declarator,
    const_exp: const_exp,
) -> struct_declarator {
    struct_declarator::C2(struct_declaratorC2 {
        declarator: Box::new(declarator),
        const_exp: Box::new(const_exp),
    })
}
pub fn struct_declarator_const_exp(_ctx: &Ctx, const_exp: const_exp) -> struct_declarator {
    struct_declarator::const_exp(Box::new(const_exp))
}
#[derive(Debug, Clone)]
pub struct enum_specC1 {
    pub name: Box<id>,
    pub fields: enumerator1,
    pub comma_opt: commaOpt,
}
#[derive(Debug, Clone)]
pub struct enum_specC2 {
    pub fields: Box<enumerator1>,
    pub comma_opt: Box<commaOpt>,
}
#[derive(Debug, Clone)]
pub struct enum_specC3 {
    pub name: Box<id>,
}
#[derive(Debug, Clone)]
pub enum enum_spec {
    C1(enum_specC1),
    C2(enum_specC2),
    C3(enum_specC3),
}
pub fn enum_spec_c1(_ctx: &Ctx, name: id, fields: enumerator1, comma_opt: commaOpt) -> enum_spec {
    enum_spec::C1(enum_specC1 {
        name: Box::new(name),
        fields,
        comma_opt,
    })
}
pub fn enum_spec_c2(_ctx: &Ctx, fields: enumerator1, comma_opt: commaOpt) -> enum_spec {
    enum_spec::C2(enum_specC2 {
        fields: Box::new(fields),
        comma_opt: Box::new(comma_opt),
    })
}
pub fn enum_spec_c3(_ctx: &Ctx, name: id) -> enum_spec {
    enum_spec::C3(enum_specC3 {
        name: Box::new(name),
    })
}
pub type enumerator1 = Vec<enumerator>;
pub fn enumerator1_c1(
    _ctx: &Ctx,
    mut enumerator1: enumerator1,
    enumerator: enumerator,
) -> enumerator1 {
    enumerator1.push(enumerator);
    enumerator1
}
pub fn enumerator1_enumerator(_ctx: &Ctx, enumerator: enumerator) -> enumerator1 {
    vec![enumerator]
}
pub type commaOpt = Option<CommaOptNoO>;
#[derive(Debug, Clone)]
pub enum CommaOptNoO {
    comma,
}
pub fn comma_opt_comma(_ctx: &Ctx) -> commaOpt {
    Some(CommaOptNoO::comma)
}
pub fn comma_opt_empty(_ctx: &Ctx) -> commaOpt {
    None
}
#[derive(Debug, Clone)]
pub struct enumeratorC2 {
    pub id: Box<id>,
    pub const_exp: Box<const_exp>,
}
#[derive(Debug, Clone)]
pub enum enumerator {
    id(Box<id>),
    C2(enumeratorC2),
}
pub fn enumerator_id(_ctx: &Ctx, id: id) -> enumerator {
    enumerator::id(Box::new(id))
}
pub fn enumerator_c2(_ctx: &Ctx, id: id, const_exp: const_exp) -> enumerator {
    enumerator::C2(enumeratorC2 {
        id: Box::new(id),
        const_exp: Box::new(const_exp),
    })
}
#[derive(Debug, Clone)]
pub struct declarator {
    pub pointer_opt: pointerOpt,
    pub dd: direct_declarator,
}
pub fn declarator_c1(_ctx: &Ctx, pointer_opt: pointerOpt, dd: direct_declarator) -> declarator {
    declarator { pointer_opt, dd }
}
pub type pointerOpt = Option<pointer>;
pub fn pointer_opt_pointer(_ctx: &Ctx, pointer: pointer) -> pointerOpt {
    Some(pointer)
}
pub fn pointer_opt_empty(_ctx: &Ctx) -> pointerOpt {
    None
}
#[derive(Debug, Clone)]
pub struct direct_declaratorC1 {
    pub name: Box<id>,
}
#[derive(Debug, Clone)]
pub struct direct_declaratorC3 {
    pub array: Box<direct_declarator>,
    pub const_exp: const_exp,
}
#[derive(Debug, Clone)]
pub struct direct_declaratorC4 {
    pub array: Box<direct_declarator>,
}
#[derive(Debug, Clone)]
pub struct direct_declaratorC5 {
    pub fnc_decl: Box<direct_declarator>,
    pub param_type_list: Box<param_type_list>,
}
#[derive(Debug, Clone)]
pub struct direct_declaratorC6 {
    pub fnc_decl: Box<direct_declarator>,
    pub id1: id1,
}
#[derive(Debug, Clone)]
pub struct direct_declaratorC7 {
    pub fnc_decl: Box<direct_declarator>,
}
#[derive(Debug, Clone)]
pub enum direct_declarator {
    C1(direct_declaratorC1),
    declarator(Box<declarator>),
    C3(direct_declaratorC3),
    C4(direct_declaratorC4),
    C5(direct_declaratorC5),
    C6(direct_declaratorC6),
    C7(direct_declaratorC7),
}
pub fn direct_declarator_c1(_ctx: &Ctx, name: id) -> direct_declarator {
    direct_declarator::C1(direct_declaratorC1 {
        name: Box::new(name),
    })
}
pub fn direct_declarator_declarator(_ctx: &Ctx, declarator: declarator) -> direct_declarator {
    direct_declarator::declarator(Box::new(declarator))
}
pub fn direct_declarator_c3(
    _ctx: &Ctx,
    array: direct_declarator,
    const_exp: const_exp,
) -> direct_declarator {
    direct_declarator::C3(direct_declaratorC3 {
        array: Box::new(array),
        const_exp,
    })
}
pub fn direct_declarator_c4(_ctx: &Ctx, array: direct_declarator) -> direct_declarator {
    direct_declarator::C4(direct_declaratorC4 {
        array: Box::new(array),
    })
}
pub fn direct_declarator_c5(
    _ctx: &Ctx,
    fnc_decl: direct_declarator,
    param_type_list: param_type_list,
) -> direct_declarator {
    direct_declarator::C5(direct_declaratorC5 {
        fnc_decl: Box::new(fnc_decl),
        param_type_list: Box::new(param_type_list),
    })
}
pub fn direct_declarator_c6(
    _ctx: &Ctx,
    fnc_decl: direct_declarator,
    id1: id1,
) -> direct_declarator {
    direct_declarator::C6(direct_declaratorC6 {
        fnc_decl: Box::new(fnc_decl),
        id1,
    })
}
pub fn direct_declarator_c7(_ctx: &Ctx, fnc_decl: direct_declarator) -> direct_declarator {
    direct_declarator::C7(direct_declaratorC7 {
        fnc_decl: Box::new(fnc_decl),
    })
}
pub type id1 = Vec<Box<id>>;
pub fn id1_c1(_ctx: &Ctx, mut id1: id1, id: id) -> id1 {
    id1.push(Box::new(id));
    id1
}
pub fn id1_id(_ctx: &Ctx, id: id) -> id1 {
    vec![Box::new(id)]
}
#[derive(Debug, Clone)]
pub struct pointer {
    pub type_qualifier0: type_qualifier0,
    pub pointer_opt: Box<pointerOpt>,
}
pub fn pointer_c1(
    _ctx: &Ctx,
    type_qualifier0: type_qualifier0,
    pointer_opt: pointerOpt,
) -> pointer {
    pointer {
        type_qualifier0,
        pointer_opt: Box::new(pointer_opt),
    }
}
pub type type_qualifier1 = Vec<Box<type_qualifier>>;
pub fn type_qualifier1_c1(
    _ctx: &Ctx,
    mut type_qualifier1: type_qualifier1,
    type_qualifier: type_qualifier,
) -> type_qualifier1 {
    type_qualifier1.push(Box::new(type_qualifier));
    type_qualifier1
}
pub fn type_qualifier1_type_qualifier(
    _ctx: &Ctx,
    type_qualifier: type_qualifier,
) -> type_qualifier1 {
    vec![Box::new(type_qualifier)]
}
pub type type_qualifier0 = Option<type_qualifier1>;
pub fn type_qualifier0_type_qualifier1(
    _ctx: &Ctx,
    type_qualifier1: type_qualifier1,
) -> type_qualifier0 {
    Some(type_qualifier1)
}
pub fn type_qualifier0_empty(_ctx: &Ctx) -> type_qualifier0 {
    None
}
#[derive(Debug, Clone)]
pub struct param_type_list {
    pub param_decl1: param_decl1,
    pub param_type_list_varargs_opt: param_type_list_varargsOpt,
}
pub fn param_type_list_c1(
    _ctx: &Ctx,
    param_decl1: param_decl1,
    param_type_list_varargs_opt: param_type_list_varargsOpt,
) -> param_type_list {
    param_type_list {
        param_decl1,
        param_type_list_varargs_opt,
    }
}
pub type param_decl1 = Vec<param_decl>;
pub fn param_decl1_c1(
    _ctx: &Ctx,
    mut param_decl1: param_decl1,
    param_decl: param_decl,
) -> param_decl1 {
    param_decl1.push(param_decl);
    param_decl1
}
pub fn param_decl1_param_decl(_ctx: &Ctx, param_decl: param_decl) -> param_decl1 {
    vec![param_decl]
}
pub type param_type_list_varargsOpt = Option<param_type_list_varargs>;
pub fn param_type_list_varargs_opt_param_type_list_varargs(
    _ctx: &Ctx,
    param_type_list_varargs: param_type_list_varargs,
) -> param_type_list_varargsOpt {
    Some(param_type_list_varargs)
}
pub fn param_type_list_varargs_opt_empty(_ctx: &Ctx) -> param_type_list_varargsOpt {
    None
}
#[derive(Debug, Clone)]
pub enum param_type_list_varargs {
    C1,
}
pub fn param_type_list_varargs_c1(_ctx: &Ctx) -> param_type_list_varargs {
    param_type_list_varargs::C1
}
#[derive(Debug, Clone)]
pub struct param_decl {
    pub decl_specs: Box<decl_specs>,
    pub param_decl_declarator_opt: param_decl_declaratorOpt,
}
pub fn param_decl_c1(
    _ctx: &Ctx,
    decl_specs: decl_specs,
    param_decl_declarator_opt: param_decl_declaratorOpt,
) -> param_decl {
    param_decl {
        decl_specs: Box::new(decl_specs),
        param_decl_declarator_opt,
    }
}
pub type param_decl_declaratorOpt = Option<param_decl_declarator>;
pub fn param_decl_declarator_opt_param_decl_declarator(
    _ctx: &Ctx,
    param_decl_declarator: param_decl_declarator,
) -> param_decl_declaratorOpt {
    Some(param_decl_declarator)
}
pub fn param_decl_declarator_opt_empty(_ctx: &Ctx) -> param_decl_declaratorOpt {
    None
}
#[derive(Debug, Clone)]
pub enum param_decl_declarator {
    declarator(Box<declarator>),
    abstract_declarator(Box<abstract_declarator>),
}
pub fn param_decl_declarator_declarator(
    _ctx: &Ctx,
    declarator: declarator,
) -> param_decl_declarator {
    param_decl_declarator::declarator(Box::new(declarator))
}
pub fn param_decl_declarator_abstract_declarator(
    _ctx: &Ctx,
    abstract_declarator: abstract_declarator,
) -> param_decl_declarator {
    param_decl_declarator::abstract_declarator(Box::new(abstract_declarator))
}
#[derive(Debug, Clone)]
pub struct initializerC2 {
    pub initializer_list: initializer_list,
    pub comma_opt: Box<commaOpt>,
}
#[derive(Debug, Clone)]
pub enum initializer {
    assignment_exp(Box<assignment_exp>),
    C2(initializerC2),
}
pub fn initializer_assignment_exp(_ctx: &Ctx, assignment_exp: assignment_exp) -> initializer {
    initializer::assignment_exp(Box::new(assignment_exp))
}
pub fn initializer_c2(
    _ctx: &Ctx,
    initializer_list: initializer_list,
    comma_opt: commaOpt,
) -> initializer {
    initializer::C2(initializerC2 {
        initializer_list,
        comma_opt: Box::new(comma_opt),
    })
}
#[derive(Debug, Clone)]
pub struct initializer_listC1 {
    pub line_directive_opt: line_directiveOpt,
    pub initializer: Box<initializer>,
}
#[derive(Debug, Clone)]
pub struct initializer_listC2 {
    pub initializer_list: Box<initializer_list>,
    pub line_directive_opt: Box<line_directiveOpt>,
    pub initializer: Box<initializer>,
}
#[derive(Debug, Clone)]
pub enum initializer_list {
    C1(initializer_listC1),
    C2(initializer_listC2),
}
pub fn initializer_list_c1(
    _ctx: &Ctx,
    line_directive_opt: line_directiveOpt,
    initializer: initializer,
) -> initializer_list {
    initializer_list::C1(initializer_listC1 {
        line_directive_opt,
        initializer: Box::new(initializer),
    })
}
pub fn initializer_list_c2(
    _ctx: &Ctx,
    initializer_list: initializer_list,
    line_directive_opt: line_directiveOpt,
    initializer: initializer,
) -> initializer_list {
    initializer_list::C2(initializer_listC2 {
        initializer_list: Box::new(initializer_list),
        line_directive_opt: Box::new(line_directive_opt),
        initializer: Box::new(initializer),
    })
}
pub type line_directiveOpt = Option<Box<line_directive>>;
pub fn line_directive_opt_line_directive(
    _ctx: &Ctx,
    line_directive: line_directive,
) -> line_directiveOpt {
    Some(Box::new(line_directive))
}
pub fn line_directive_opt_empty(_ctx: &Ctx) -> line_directiveOpt {
    None
}
#[derive(Debug, Clone)]
pub struct type_nameC1 {
    pub spec_qualifier_list: Box<spec_qualifier_list>,
    pub abstract_declarator: abstract_declarator,
}
#[derive(Debug, Clone)]
pub enum type_name {
    C1(type_nameC1),
    spec_qualifier_list(Box<spec_qualifier_list>),
}
pub fn type_name_c1(
    _ctx: &Ctx,
    spec_qualifier_list: spec_qualifier_list,
    abstract_declarator: abstract_declarator,
) -> type_name {
    type_name::C1(type_nameC1 {
        spec_qualifier_list: Box::new(spec_qualifier_list),
        abstract_declarator,
    })
}
pub fn type_name_spec_qualifier_list(
    _ctx: &Ctx,
    spec_qualifier_list: spec_qualifier_list,
) -> type_name {
    type_name::spec_qualifier_list(Box::new(spec_qualifier_list))
}
#[derive(Debug, Clone)]
pub struct abstract_declaratorC2 {
    pub pointer: Box<pointer>,
    pub direct_abstract_declarator: direct_abstract_declarator,
}
#[derive(Debug, Clone)]
pub enum abstract_declarator {
    pointer(Box<pointer>),
    C2(abstract_declaratorC2),
    direct_abstract_declarator(Box<direct_abstract_declarator>),
}
pub fn abstract_declarator_pointer(_ctx: &Ctx, pointer: pointer) -> abstract_declarator {
    abstract_declarator::pointer(Box::new(pointer))
}
pub fn abstract_declarator_c2(
    _ctx: &Ctx,
    pointer: pointer,
    direct_abstract_declarator: direct_abstract_declarator,
) -> abstract_declarator {
    abstract_declarator::C2(abstract_declaratorC2 {
        pointer: Box::new(pointer),
        direct_abstract_declarator,
    })
}
pub fn abstract_declarator_direct_abstract_declarator(
    _ctx: &Ctx,
    direct_abstract_declarator: direct_abstract_declarator,
) -> abstract_declarator {
    abstract_declarator::direct_abstract_declarator(Box::new(direct_abstract_declarator))
}
#[derive(Debug, Clone)]
pub struct direct_abstract_declaratorC2 {
    pub direct_abstract_declarator: Box<direct_abstract_declarator>,
    pub const_exp: Box<const_exp>,
}
#[derive(Debug, Clone)]
pub struct direct_abstract_declaratorC6 {
    pub direct_abstract_declarator: Box<direct_abstract_declarator>,
    pub param_type_list: param_type_list,
}
#[derive(Debug, Clone)]
pub enum direct_abstract_declarator {
    abstract_declarator(Box<abstract_declarator>),
    C2(direct_abstract_declaratorC2),
    const_exp(Box<const_exp>),
    direct_abstract_declarator1(Box<direct_abstract_declarator>),
    C5,
    C6(direct_abstract_declaratorC6),
    param_type_list(Box<param_type_list>),
    direct_abstract_declarator2(Box<direct_abstract_declarator>),
    C9,
}
pub fn direct_abstract_declarator_abstract_declarator(
    _ctx: &Ctx,
    abstract_declarator: abstract_declarator,
) -> direct_abstract_declarator {
    direct_abstract_declarator::abstract_declarator(Box::new(abstract_declarator))
}
pub fn direct_abstract_declarator_c2(
    _ctx: &Ctx,
    direct_abstract_declarator: direct_abstract_declarator,
    const_exp: const_exp,
) -> direct_abstract_declarator {
    direct_abstract_declarator::C2(direct_abstract_declaratorC2 {
        direct_abstract_declarator: Box::new(direct_abstract_declarator),
        const_exp: Box::new(const_exp),
    })
}
pub fn direct_abstract_declarator_const_exp(
    _ctx: &Ctx,
    const_exp: const_exp,
) -> direct_abstract_declarator {
    direct_abstract_declarator::const_exp(Box::new(const_exp))
}
pub fn direct_abstract_declarator_direct_abstract_declarator1(
    _ctx: &Ctx,
    direct_abstract_declarator: direct_abstract_declarator,
) -> direct_abstract_declarator {
    direct_abstract_declarator::direct_abstract_declarator1(Box::new(direct_abstract_declarator))
}
pub fn direct_abstract_declarator_c5(_ctx: &Ctx) -> direct_abstract_declarator {
    direct_abstract_declarator::C5
}
pub fn direct_abstract_declarator_c6(
    _ctx: &Ctx,
    direct_abstract_declarator: direct_abstract_declarator,
    param_type_list: param_type_list,
) -> direct_abstract_declarator {
    direct_abstract_declarator::C6(direct_abstract_declaratorC6 {
        direct_abstract_declarator: Box::new(direct_abstract_declarator),
        param_type_list,
    })
}
pub fn direct_abstract_declarator_param_type_list(
    _ctx: &Ctx,
    param_type_list: param_type_list,
) -> direct_abstract_declarator {
    direct_abstract_declarator::param_type_list(Box::new(param_type_list))
}
pub fn direct_abstract_declarator_direct_abstract_declarator2(
    _ctx: &Ctx,
    direct_abstract_declarator: direct_abstract_declarator,
) -> direct_abstract_declarator {
    direct_abstract_declarator::direct_abstract_declarator2(Box::new(direct_abstract_declarator))
}
pub fn direct_abstract_declarator_c9(_ctx: &Ctx) -> direct_abstract_declarator {
    direct_abstract_declarator::C9
}
pub type typedef_name = Box<id>;
pub fn typedef_name_id(_ctx: &Ctx, id: id) -> typedef_name {
    Box::new(id)
}
#[derive(Debug, Clone)]
pub enum stat {
    labeled_stat(labeled_stat),
    exp_stat(exp_stat),
    compound_stat(Box<compound_stat>),
    selection_stat(selection_stat),
    iteration_stat(iteration_stat),
    jump_stat(jump_stat),
    line_directive(Box<line_directive>),
}
pub fn stat_labeled_stat(_ctx: &Ctx, labeled_stat: labeled_stat) -> stat {
    stat::labeled_stat(labeled_stat)
}
pub fn stat_exp_stat(_ctx: &Ctx, exp_stat: exp_stat) -> stat {
    stat::exp_stat(exp_stat)
}
pub fn stat_compound_stat(_ctx: &Ctx, compound_stat: compound_stat) -> stat {
    stat::compound_stat(Box::new(compound_stat))
}
pub fn stat_selection_stat(_ctx: &Ctx, selection_stat: selection_stat) -> stat {
    stat::selection_stat(selection_stat)
}
pub fn stat_iteration_stat(_ctx: &Ctx, iteration_stat: iteration_stat) -> stat {
    stat::iteration_stat(iteration_stat)
}
pub fn stat_jump_stat(_ctx: &Ctx, jump_stat: jump_stat) -> stat {
    stat::jump_stat(jump_stat)
}
pub fn stat_line_directive(_ctx: &Ctx, line_directive: line_directive) -> stat {
    stat::line_directive(Box::new(line_directive))
}
#[derive(Debug, Clone)]
pub struct labeled_statC1 {
    pub id: Box<id>,
    pub stat: Box<stat>,
}
#[derive(Debug, Clone)]
pub struct labeled_statC2 {
    pub const_exp: Box<const_exp>,
    pub stat: Box<stat>,
}
#[derive(Debug, Clone)]
pub enum labeled_stat {
    C1(labeled_statC1),
    C2(labeled_statC2),
    stat(Box<stat>),
}
pub fn labeled_stat_c1(_ctx: &Ctx, id: id, stat: stat) -> labeled_stat {
    labeled_stat::C1(labeled_statC1 {
        id: Box::new(id),
        stat: Box::new(stat),
    })
}
pub fn labeled_stat_c2(_ctx: &Ctx, const_exp: const_exp, stat: stat) -> labeled_stat {
    labeled_stat::C2(labeled_statC2 {
        const_exp: Box::new(const_exp),
        stat: Box::new(stat),
    })
}
pub fn labeled_stat_stat(_ctx: &Ctx, stat: stat) -> labeled_stat {
    labeled_stat::stat(Box::new(stat))
}
pub type exp_stat = expOpt;
pub fn exp_stat_exp_opt(_ctx: &Ctx, exp_opt: expOpt) -> exp_stat {
    exp_opt
}
pub type expOpt = Option<Box<exp>>;
pub fn exp_opt_exp(_ctx: &Ctx, exp: exp) -> expOpt {
    Some(Box::new(exp))
}
pub fn exp_opt_empty(_ctx: &Ctx) -> expOpt {
    None
}
#[derive(Debug, Clone)]
pub enum block_item {
    decl(Box<decl>),
    stat(stat),
}
pub fn block_item_decl(_ctx: &Ctx, decl: decl) -> block_item {
    block_item::decl(Box::new(decl))
}
pub fn block_item_stat(_ctx: &Ctx, stat: stat) -> block_item {
    block_item::stat(stat)
}
pub type compound_stat = block_item0;
pub fn compound_stat_block_item0(_ctx: &Ctx, block_item0: block_item0) -> compound_stat {
    block_item0
}
pub type block_item1 = Vec<block_item>;
pub fn block_item1_c1(
    _ctx: &Ctx,
    mut block_item1: block_item1,
    block_item: block_item,
) -> block_item1 {
    block_item1.push(block_item);
    block_item1
}
pub fn block_item1_block_item(_ctx: &Ctx, block_item: block_item) -> block_item1 {
    vec![block_item]
}
pub type block_item0 = Option<block_item1>;
pub fn block_item0_block_item1(_ctx: &Ctx, block_item1: block_item1) -> block_item0 {
    Some(block_item1)
}
pub fn block_item0_empty(_ctx: &Ctx) -> block_item0 {
    None
}
#[derive(Debug, Clone)]
pub struct selection_statC1 {
    pub exp: Box<exp>,
    pub stat: Box<stat>,
}
#[derive(Debug, Clone)]
pub struct selection_statC2 {
    pub exp: Box<exp>,
    pub stat_5: Box<stat>,
    pub stat_7: Box<stat>,
}
#[derive(Debug, Clone)]
pub struct selection_statC3 {
    pub exp: Box<exp>,
    pub stat: Box<stat>,
}
#[derive(Debug, Clone)]
pub enum selection_stat {
    C1(selection_statC1),
    C2(selection_statC2),
    C3(selection_statC3),
}
pub fn selection_stat_c1(_ctx: &Ctx, exp: exp, stat: stat) -> selection_stat {
    selection_stat::C1(selection_statC1 {
        exp: Box::new(exp),
        stat: Box::new(stat),
    })
}
pub fn selection_stat_c2(_ctx: &Ctx, exp: exp, stat_5: stat, stat_7: stat) -> selection_stat {
    selection_stat::C2(selection_statC2 {
        exp: Box::new(exp),
        stat_5: Box::new(stat_5),
        stat_7: Box::new(stat_7),
    })
}
pub fn selection_stat_c3(_ctx: &Ctx, exp: exp, stat: stat) -> selection_stat {
    selection_stat::C3(selection_statC3 {
        exp: Box::new(exp),
        stat: Box::new(stat),
    })
}
#[derive(Debug, Clone)]
pub struct iteration_statC1 {
    pub exp: Box<exp>,
    pub stat: Box<stat>,
}
#[derive(Debug, Clone)]
pub struct iteration_statC2 {
    pub stat: Box<stat>,
    pub exp: Box<exp>,
}
#[derive(Debug, Clone)]
pub struct iteration_statC3 {
    pub exp_opt_3: Box<expOpt>,
    pub exp_opt_5: Box<expOpt>,
    pub exp_opt_7: Box<expOpt>,
    pub stat: Box<stat>,
}
#[derive(Debug, Clone)]
pub struct iteration_statC4 {
    pub decl_body: Box<decl_body>,
    pub exp_opt_5: Box<expOpt>,
    pub exp_opt_7: Box<expOpt>,
    pub stat: Box<stat>,
}
#[derive(Debug, Clone)]
pub enum iteration_stat {
    C1(iteration_statC1),
    C2(iteration_statC2),
    C3(iteration_statC3),
    C4(iteration_statC4),
}
pub fn iteration_stat_c1(_ctx: &Ctx, exp: exp, stat: stat) -> iteration_stat {
    iteration_stat::C1(iteration_statC1 {
        exp: Box::new(exp),
        stat: Box::new(stat),
    })
}
pub fn iteration_stat_c2(_ctx: &Ctx, stat: stat, exp: exp) -> iteration_stat {
    iteration_stat::C2(iteration_statC2 {
        stat: Box::new(stat),
        exp: Box::new(exp),
    })
}
pub fn iteration_stat_c3(
    _ctx: &Ctx,
    exp_opt_3: expOpt,
    exp_opt_5: expOpt,
    exp_opt_7: expOpt,
    stat: stat,
) -> iteration_stat {
    iteration_stat::C3(iteration_statC3 {
        exp_opt_3: Box::new(exp_opt_3),
        exp_opt_5: Box::new(exp_opt_5),
        exp_opt_7: Box::new(exp_opt_7),
        stat: Box::new(stat),
    })
}
pub fn iteration_stat_c4(
    _ctx: &Ctx,
    decl_body: decl_body,
    exp_opt_5: expOpt,
    exp_opt_7: expOpt,
    stat: stat,
) -> iteration_stat {
    iteration_stat::C4(iteration_statC4 {
        decl_body: Box::new(decl_body),
        exp_opt_5: Box::new(exp_opt_5),
        exp_opt_7: Box::new(exp_opt_7),
        stat: Box::new(stat),
    })
}
#[derive(Debug, Clone)]
pub enum jump_stat {
    id(Box<id>),
    C2,
    C3,
    exp(Box<exp>),
    C5,
}
pub fn jump_stat_id(_ctx: &Ctx, id: id) -> jump_stat {
    jump_stat::id(Box::new(id))
}
pub fn jump_stat_c2(_ctx: &Ctx) -> jump_stat {
    jump_stat::C2
}
pub fn jump_stat_c3(_ctx: &Ctx) -> jump_stat {
    jump_stat::C3
}
pub fn jump_stat_exp(_ctx: &Ctx, exp: exp) -> jump_stat {
    jump_stat::exp(Box::new(exp))
}
pub fn jump_stat_c5(_ctx: &Ctx) -> jump_stat {
    jump_stat::C5
}
#[derive(Debug, Clone)]
pub struct expC2 {
    pub exp: Box<exp>,
    pub assignment_exp: Box<assignment_exp>,
}
#[derive(Debug, Clone)]
pub enum exp {
    assignment_exp(assignment_exp),
    C2(expC2),
}
pub fn exp_assignment_exp(_ctx: &Ctx, assignment_exp: assignment_exp) -> exp {
    exp::assignment_exp(assignment_exp)
}
pub fn exp_c2(_ctx: &Ctx, exp: exp, assignment_exp: assignment_exp) -> exp {
    exp::C2(expC2 {
        exp: Box::new(exp),
        assignment_exp: Box::new(assignment_exp),
    })
}
#[derive(Debug, Clone)]
pub struct assignment_expC2 {
    pub unary_exp: Box<unary_exp>,
    pub assignment_operator: assignment_operator,
    pub assignment_exp: Box<assignment_exp>,
}
#[derive(Debug, Clone)]
pub enum assignment_exp {
    conditional_exp(Box<conditional_exp>),
    C2(assignment_expC2),
}
pub fn assignment_exp_conditional_exp(
    _ctx: &Ctx,
    conditional_exp: conditional_exp,
) -> assignment_exp {
    assignment_exp::conditional_exp(Box::new(conditional_exp))
}
pub fn assignment_exp_c2(
    _ctx: &Ctx,
    unary_exp: unary_exp,
    assignment_operator: assignment_operator,
    assignment_exp: assignment_exp,
) -> assignment_exp {
    assignment_exp::C2(assignment_expC2 {
        unary_exp: Box::new(unary_exp),
        assignment_operator,
        assignment_exp: Box::new(assignment_exp),
    })
}
#[derive(Debug, Clone)]
pub enum assignment_operator {
    assign,
    mulaop,
    divaop,
    modaop,
    sumaop,
    subaop,
    lshiftaop,
    rshiftaop,
    andaop,
    xoraop,
    oraop,
}
pub fn assignment_operator_assign(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::assign
}
pub fn assignment_operator_mulaop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::mulaop
}
pub fn assignment_operator_divaop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::divaop
}
pub fn assignment_operator_modaop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::modaop
}
pub fn assignment_operator_sumaop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::sumaop
}
pub fn assignment_operator_subaop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::subaop
}
pub fn assignment_operator_lshiftaop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::lshiftaop
}
pub fn assignment_operator_rshiftaop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::rshiftaop
}
pub fn assignment_operator_andaop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::andaop
}
pub fn assignment_operator_xoraop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::xoraop
}
pub fn assignment_operator_oraop(_ctx: &Ctx) -> assignment_operator {
    assignment_operator::oraop
}
#[derive(Debug, Clone)]
pub struct conditional_expC2 {
    pub logical_or_exp: Box<logical_or_exp>,
    pub exp: Box<exp>,
    pub conditional_exp: Box<conditional_exp>,
}
#[derive(Debug, Clone)]
pub enum conditional_exp {
    logical_or_exp(logical_or_exp),
    C2(conditional_expC2),
}
pub fn conditional_exp_logical_or_exp(
    _ctx: &Ctx,
    logical_or_exp: logical_or_exp,
) -> conditional_exp {
    conditional_exp::logical_or_exp(logical_or_exp)
}
pub fn conditional_exp_c2(
    _ctx: &Ctx,
    logical_or_exp: logical_or_exp,
    exp: exp,
    conditional_exp: conditional_exp,
) -> conditional_exp {
    conditional_exp::C2(conditional_expC2 {
        logical_or_exp: Box::new(logical_or_exp),
        exp: Box::new(exp),
        conditional_exp: Box::new(conditional_exp),
    })
}
pub type const_exp = conditional_exp;
pub fn const_exp_conditional_exp(_ctx: &Ctx, conditional_exp: conditional_exp) -> const_exp {
    conditional_exp
}
#[derive(Debug, Clone)]
pub struct logical_or_expC2 {
    pub logical_or_exp: Box<logical_or_exp>,
    pub logical_and_exp: Box<logical_and_exp>,
}
#[derive(Debug, Clone)]
pub enum logical_or_exp {
    logical_and_exp(logical_and_exp),
    C2(logical_or_expC2),
}
pub fn logical_or_exp_logical_and_exp(
    _ctx: &Ctx,
    logical_and_exp: logical_and_exp,
) -> logical_or_exp {
    logical_or_exp::logical_and_exp(logical_and_exp)
}
pub fn logical_or_exp_c2(
    _ctx: &Ctx,
    logical_or_exp: logical_or_exp,
    logical_and_exp: logical_and_exp,
) -> logical_or_exp {
    logical_or_exp::C2(logical_or_expC2 {
        logical_or_exp: Box::new(logical_or_exp),
        logical_and_exp: Box::new(logical_and_exp),
    })
}
#[derive(Debug, Clone)]
pub struct logical_and_expC2 {
    pub logical_and_exp: Box<logical_and_exp>,
    pub inclusive_or_exp: Box<inclusive_or_exp>,
}
#[derive(Debug, Clone)]
pub enum logical_and_exp {
    inclusive_or_exp(inclusive_or_exp),
    C2(logical_and_expC2),
}
pub fn logical_and_exp_inclusive_or_exp(
    _ctx: &Ctx,
    inclusive_or_exp: inclusive_or_exp,
) -> logical_and_exp {
    logical_and_exp::inclusive_or_exp(inclusive_or_exp)
}
pub fn logical_and_exp_c2(
    _ctx: &Ctx,
    logical_and_exp: logical_and_exp,
    inclusive_or_exp: inclusive_or_exp,
) -> logical_and_exp {
    logical_and_exp::C2(logical_and_expC2 {
        logical_and_exp: Box::new(logical_and_exp),
        inclusive_or_exp: Box::new(inclusive_or_exp),
    })
}
#[derive(Debug, Clone)]
pub struct inclusive_or_expC2 {
    pub inclusive_or_exp: Box<inclusive_or_exp>,
    pub exclusive_or_exp: Box<exclusive_or_exp>,
}
#[derive(Debug, Clone)]
pub enum inclusive_or_exp {
    exclusive_or_exp(exclusive_or_exp),
    C2(inclusive_or_expC2),
}
pub fn inclusive_or_exp_exclusive_or_exp(
    _ctx: &Ctx,
    exclusive_or_exp: exclusive_or_exp,
) -> inclusive_or_exp {
    inclusive_or_exp::exclusive_or_exp(exclusive_or_exp)
}
pub fn inclusive_or_exp_c2(
    _ctx: &Ctx,
    inclusive_or_exp: inclusive_or_exp,
    exclusive_or_exp: exclusive_or_exp,
) -> inclusive_or_exp {
    inclusive_or_exp::C2(inclusive_or_expC2 {
        inclusive_or_exp: Box::new(inclusive_or_exp),
        exclusive_or_exp: Box::new(exclusive_or_exp),
    })
}
#[derive(Debug, Clone)]
pub struct exclusive_or_expC2 {
    pub exclusive_or_exp: Box<exclusive_or_exp>,
    pub and_exp: Box<and_exp>,
}
#[derive(Debug, Clone)]
pub enum exclusive_or_exp {
    and_exp(and_exp),
    C2(exclusive_or_expC2),
}
pub fn exclusive_or_exp_and_exp(_ctx: &Ctx, and_exp: and_exp) -> exclusive_or_exp {
    exclusive_or_exp::and_exp(and_exp)
}
pub fn exclusive_or_exp_c2(
    _ctx: &Ctx,
    exclusive_or_exp: exclusive_or_exp,
    and_exp: and_exp,
) -> exclusive_or_exp {
    exclusive_or_exp::C2(exclusive_or_expC2 {
        exclusive_or_exp: Box::new(exclusive_or_exp),
        and_exp: Box::new(and_exp),
    })
}
#[derive(Debug, Clone)]
pub struct and_expC2 {
    pub and_exp: Box<and_exp>,
    pub equality_exp: Box<equality_exp>,
}
#[derive(Debug, Clone)]
pub enum and_exp {
    equality_exp(equality_exp),
    C2(and_expC2),
}
pub fn and_exp_equality_exp(_ctx: &Ctx, equality_exp: equality_exp) -> and_exp {
    and_exp::equality_exp(equality_exp)
}
pub fn and_exp_c2(_ctx: &Ctx, and_exp: and_exp, equality_exp: equality_exp) -> and_exp {
    and_exp::C2(and_expC2 {
        and_exp: Box::new(and_exp),
        equality_exp: Box::new(equality_exp),
    })
}
#[derive(Debug, Clone)]
pub struct equality_expC2 {
    pub equality_exp: Box<equality_exp>,
    pub relational_exp: Box<relational_exp>,
}
#[derive(Debug, Clone)]
pub struct equality_expC3 {
    pub equality_exp: Box<equality_exp>,
    pub relational_exp: Box<relational_exp>,
}
#[derive(Debug, Clone)]
pub enum equality_exp {
    relational_exp(relational_exp),
    C2(equality_expC2),
    C3(equality_expC3),
}
pub fn equality_exp_relational_exp(_ctx: &Ctx, relational_exp: relational_exp) -> equality_exp {
    equality_exp::relational_exp(relational_exp)
}
pub fn equality_exp_c2(
    _ctx: &Ctx,
    equality_exp: equality_exp,
    relational_exp: relational_exp,
) -> equality_exp {
    equality_exp::C2(equality_expC2 {
        equality_exp: Box::new(equality_exp),
        relational_exp: Box::new(relational_exp),
    })
}
pub fn equality_exp_c3(
    _ctx: &Ctx,
    equality_exp: equality_exp,
    relational_exp: relational_exp,
) -> equality_exp {
    equality_exp::C3(equality_expC3 {
        equality_exp: Box::new(equality_exp),
        relational_exp: Box::new(relational_exp),
    })
}
#[derive(Debug, Clone)]
pub struct relational_expC2 {
    pub relational_exp: Box<relational_exp>,
    pub shift_expression: Box<shift_expression>,
}
#[derive(Debug, Clone)]
pub struct relational_expC3 {
    pub relational_exp: Box<relational_exp>,
    pub shift_expression: Box<shift_expression>,
}
#[derive(Debug, Clone)]
pub struct relational_expC4 {
    pub relational_exp: Box<relational_exp>,
    pub shift_expression: Box<shift_expression>,
}
#[derive(Debug, Clone)]
pub struct relational_expC5 {
    pub relational_exp: Box<relational_exp>,
    pub shift_expression: Box<shift_expression>,
}
#[derive(Debug, Clone)]
pub enum relational_exp {
    shift_expression(shift_expression),
    C2(relational_expC2),
    C3(relational_expC3),
    C4(relational_expC4),
    C5(relational_expC5),
}
pub fn relational_exp_shift_expression(
    _ctx: &Ctx,
    shift_expression: shift_expression,
) -> relational_exp {
    relational_exp::shift_expression(shift_expression)
}
pub fn relational_exp_c2(
    _ctx: &Ctx,
    relational_exp: relational_exp,
    shift_expression: shift_expression,
) -> relational_exp {
    relational_exp::C2(relational_expC2 {
        relational_exp: Box::new(relational_exp),
        shift_expression: Box::new(shift_expression),
    })
}
pub fn relational_exp_c3(
    _ctx: &Ctx,
    relational_exp: relational_exp,
    shift_expression: shift_expression,
) -> relational_exp {
    relational_exp::C3(relational_expC3 {
        relational_exp: Box::new(relational_exp),
        shift_expression: Box::new(shift_expression),
    })
}
pub fn relational_exp_c4(
    _ctx: &Ctx,
    relational_exp: relational_exp,
    shift_expression: shift_expression,
) -> relational_exp {
    relational_exp::C4(relational_expC4 {
        relational_exp: Box::new(relational_exp),
        shift_expression: Box::new(shift_expression),
    })
}
pub fn relational_exp_c5(
    _ctx: &Ctx,
    relational_exp: relational_exp,
    shift_expression: shift_expression,
) -> relational_exp {
    relational_exp::C5(relational_expC5 {
        relational_exp: Box::new(relational_exp),
        shift_expression: Box::new(shift_expression),
    })
}
#[derive(Debug, Clone)]
pub struct shift_expressionC2 {
    pub shift_expression: Box<shift_expression>,
    pub additive_exp: Box<additive_exp>,
}
#[derive(Debug, Clone)]
pub struct shift_expressionC3 {
    pub shift_expression: Box<shift_expression>,
    pub additive_exp: Box<additive_exp>,
}
#[derive(Debug, Clone)]
pub enum shift_expression {
    additive_exp(additive_exp),
    C2(shift_expressionC2),
    C3(shift_expressionC3),
}
pub fn shift_expression_additive_exp(_ctx: &Ctx, additive_exp: additive_exp) -> shift_expression {
    shift_expression::additive_exp(additive_exp)
}
pub fn shift_expression_c2(
    _ctx: &Ctx,
    shift_expression: shift_expression,
    additive_exp: additive_exp,
) -> shift_expression {
    shift_expression::C2(shift_expressionC2 {
        shift_expression: Box::new(shift_expression),
        additive_exp: Box::new(additive_exp),
    })
}
pub fn shift_expression_c3(
    _ctx: &Ctx,
    shift_expression: shift_expression,
    additive_exp: additive_exp,
) -> shift_expression {
    shift_expression::C3(shift_expressionC3 {
        shift_expression: Box::new(shift_expression),
        additive_exp: Box::new(additive_exp),
    })
}
#[derive(Debug, Clone)]
pub struct additive_expC2 {
    pub additive_exp: Box<additive_exp>,
    pub mult_exp: Box<mult_exp>,
}
#[derive(Debug, Clone)]
pub struct additive_expC3 {
    pub additive_exp: Box<additive_exp>,
    pub mult_exp: Box<mult_exp>,
}
#[derive(Debug, Clone)]
pub enum additive_exp {
    mult_exp(mult_exp),
    C2(additive_expC2),
    C3(additive_expC3),
}
pub fn additive_exp_mult_exp(_ctx: &Ctx, mult_exp: mult_exp) -> additive_exp {
    additive_exp::mult_exp(mult_exp)
}
pub fn additive_exp_c2(
    _ctx: &Ctx,
    additive_exp: additive_exp,
    mult_exp: mult_exp,
) -> additive_exp {
    additive_exp::C2(additive_expC2 {
        additive_exp: Box::new(additive_exp),
        mult_exp: Box::new(mult_exp),
    })
}
pub fn additive_exp_c3(
    _ctx: &Ctx,
    additive_exp: additive_exp,
    mult_exp: mult_exp,
) -> additive_exp {
    additive_exp::C3(additive_expC3 {
        additive_exp: Box::new(additive_exp),
        mult_exp: Box::new(mult_exp),
    })
}
#[derive(Debug, Clone)]
pub struct mult_expC2 {
    pub mult_exp: Box<mult_exp>,
    pub cast_exp: Box<cast_exp>,
}
#[derive(Debug, Clone)]
pub struct mult_expC3 {
    pub mult_exp: Box<mult_exp>,
    pub cast_exp: Box<cast_exp>,
}
#[derive(Debug, Clone)]
pub struct mult_expC4 {
    pub mult_exp: Box<mult_exp>,
    pub cast_exp: Box<cast_exp>,
}
#[derive(Debug, Clone)]
pub enum mult_exp {
    cast_exp(cast_exp),
    C2(mult_expC2),
    C3(mult_expC3),
    C4(mult_expC4),
}
pub fn mult_exp_cast_exp(_ctx: &Ctx, cast_exp: cast_exp) -> mult_exp {
    mult_exp::cast_exp(cast_exp)
}
pub fn mult_exp_c2(_ctx: &Ctx, mult_exp: mult_exp, cast_exp: cast_exp) -> mult_exp {
    mult_exp::C2(mult_expC2 {
        mult_exp: Box::new(mult_exp),
        cast_exp: Box::new(cast_exp),
    })
}
pub fn mult_exp_c3(_ctx: &Ctx, mult_exp: mult_exp, cast_exp: cast_exp) -> mult_exp {
    mult_exp::C3(mult_expC3 {
        mult_exp: Box::new(mult_exp),
        cast_exp: Box::new(cast_exp),
    })
}
pub fn mult_exp_c4(_ctx: &Ctx, mult_exp: mult_exp, cast_exp: cast_exp) -> mult_exp {
    mult_exp::C4(mult_expC4 {
        mult_exp: Box::new(mult_exp),
        cast_exp: Box::new(cast_exp),
    })
}
#[derive(Debug, Clone)]
pub struct cast_expC2 {
    pub type_name: Box<type_name>,
    pub cast_exp: Box<cast_exp>,
}
#[derive(Debug, Clone)]
pub enum cast_exp {
    unary_exp(unary_exp),
    C2(cast_expC2),
}
pub fn cast_exp_unary_exp(_ctx: &Ctx, unary_exp: unary_exp) -> cast_exp {
    cast_exp::unary_exp(unary_exp)
}
pub fn cast_exp_c2(_ctx: &Ctx, type_name: type_name, cast_exp: cast_exp) -> cast_exp {
    cast_exp::C2(cast_expC2 {
        type_name: Box::new(type_name),
        cast_exp: Box::new(cast_exp),
    })
}
#[derive(Debug, Clone)]
pub struct unary_expC4 {
    pub unary_operator: unary_operator,
    pub cast_exp: Box<cast_exp>,
}
#[derive(Debug, Clone)]
pub struct unary_expC8 {
    pub type_name: Box<type_name>,
    pub exp: Box<exp>,
}
#[derive(Debug, Clone)]
pub enum unary_exp {
    postfix_exp(postfix_exp),
    unary_exp1(Box<unary_exp>),
    unary_exp2(Box<unary_exp>),
    C4(unary_expC4),
    unary_exp3(Box<unary_exp>),
    type_name1(type_name),
    type_name2(Box<type_name>),
    C8(unary_expC8),
}
pub fn unary_exp_postfix_exp(_ctx: &Ctx, postfix_exp: postfix_exp) -> unary_exp {
    unary_exp::postfix_exp(postfix_exp)
}
pub fn unary_exp_unary_exp1(_ctx: &Ctx, unary_exp: unary_exp) -> unary_exp {
    unary_exp::unary_exp1(Box::new(unary_exp))
}
pub fn unary_exp_unary_exp2(_ctx: &Ctx, unary_exp: unary_exp) -> unary_exp {
    unary_exp::unary_exp2(Box::new(unary_exp))
}
pub fn unary_exp_c4(_ctx: &Ctx, unary_operator: unary_operator, cast_exp: cast_exp) -> unary_exp {
    unary_exp::C4(unary_expC4 {
        unary_operator,
        cast_exp: Box::new(cast_exp),
    })
}
pub fn unary_exp_unary_exp3(_ctx: &Ctx, unary_exp: unary_exp) -> unary_exp {
    unary_exp::unary_exp3(Box::new(unary_exp))
}
pub fn unary_exp_type_name1(_ctx: &Ctx, type_name: type_name) -> unary_exp {
    unary_exp::type_name1(type_name)
}
pub fn unary_exp_type_name2(_ctx: &Ctx, type_name: type_name) -> unary_exp {
    unary_exp::type_name2(Box::new(type_name))
}
pub fn unary_exp_c8(_ctx: &Ctx, type_name: type_name, exp: exp) -> unary_exp {
    unary_exp::C8(unary_expC8 {
        type_name: Box::new(type_name),
        exp: Box::new(exp),
    })
}
#[derive(Debug, Clone)]
pub enum unary_operator {
    andop,
    mulop,
    sumop,
    subop,
    notop,
    lognotop,
}
pub fn unary_operator_andop(_ctx: &Ctx) -> unary_operator {
    unary_operator::andop
}
pub fn unary_operator_mulop(_ctx: &Ctx) -> unary_operator {
    unary_operator::mulop
}
pub fn unary_operator_sumop(_ctx: &Ctx) -> unary_operator {
    unary_operator::sumop
}
pub fn unary_operator_subop(_ctx: &Ctx) -> unary_operator {
    unary_operator::subop
}
pub fn unary_operator_notop(_ctx: &Ctx) -> unary_operator {
    unary_operator::notop
}
pub fn unary_operator_lognotop(_ctx: &Ctx) -> unary_operator {
    unary_operator::lognotop
}
#[derive(Debug, Clone)]
pub struct postfix_expC2 {
    pub postfix_exp: Box<postfix_exp>,
    pub exp: Box<exp>,
}
#[derive(Debug, Clone)]
pub struct postfix_expC3 {
    pub postfix_exp: Box<postfix_exp>,
    pub assignment_exp0: assignment_exp0,
}
#[derive(Debug, Clone)]
pub struct postfix_expC4 {
    pub postfix_exp: Box<postfix_exp>,
    pub id: Box<id>,
}
#[derive(Debug, Clone)]
pub struct postfix_expC5 {
    pub postfix_exp: Box<postfix_exp>,
    pub id: Box<id>,
}
#[derive(Debug, Clone)]
pub enum postfix_exp {
    primary_exp(primary_exp),
    C2(postfix_expC2),
    C3(postfix_expC3),
    C4(postfix_expC4),
    C5(postfix_expC5),
    postfix_exp1(Box<postfix_exp>),
    postfix_exp2(Box<postfix_exp>),
}
pub fn postfix_exp_primary_exp(_ctx: &Ctx, primary_exp: primary_exp) -> postfix_exp {
    postfix_exp::primary_exp(primary_exp)
}
pub fn postfix_exp_c2(_ctx: &Ctx, postfix_exp: postfix_exp, exp: exp) -> postfix_exp {
    postfix_exp::C2(postfix_expC2 {
        postfix_exp: Box::new(postfix_exp),
        exp: Box::new(exp),
    })
}
pub fn postfix_exp_c3(
    _ctx: &Ctx,
    postfix_exp: postfix_exp,
    assignment_exp0: assignment_exp0,
) -> postfix_exp {
    postfix_exp::C3(postfix_expC3 {
        postfix_exp: Box::new(postfix_exp),
        assignment_exp0,
    })
}
pub fn postfix_exp_c4(_ctx: &Ctx, postfix_exp: postfix_exp, id: id) -> postfix_exp {
    postfix_exp::C4(postfix_expC4 {
        postfix_exp: Box::new(postfix_exp),
        id: Box::new(id),
    })
}
pub fn postfix_exp_c5(_ctx: &Ctx, postfix_exp: postfix_exp, id: id) -> postfix_exp {
    postfix_exp::C5(postfix_expC5 {
        postfix_exp: Box::new(postfix_exp),
        id: Box::new(id),
    })
}
pub fn postfix_exp_postfix_exp1(_ctx: &Ctx, postfix_exp: postfix_exp) -> postfix_exp {
    postfix_exp::postfix_exp1(Box::new(postfix_exp))
}
pub fn postfix_exp_postfix_exp2(_ctx: &Ctx, postfix_exp: postfix_exp) -> postfix_exp {
    postfix_exp::postfix_exp2(Box::new(postfix_exp))
}
pub type assignment_exp1 = Vec<Box<assignment_exp>>;
pub fn assignment_exp1_c1(
    _ctx: &Ctx,
    mut assignment_exp1: assignment_exp1,
    assignment_exp: assignment_exp,
) -> assignment_exp1 {
    assignment_exp1.push(Box::new(assignment_exp));
    assignment_exp1
}
pub fn assignment_exp1_assignment_exp(
    _ctx: &Ctx,
    assignment_exp: assignment_exp,
) -> assignment_exp1 {
    vec![Box::new(assignment_exp)]
}
pub type assignment_exp0 = Option<assignment_exp1>;
pub fn assignment_exp0_assignment_exp1(
    _ctx: &Ctx,
    assignment_exp1: assignment_exp1,
) -> assignment_exp0 {
    Some(assignment_exp1)
}
pub fn assignment_exp0_empty(_ctx: &Ctx) -> assignment_exp0 {
    None
}
#[derive(Debug, Clone)]
pub struct primary_expC1 {
    pub var_ref: Box<id>,
}
#[derive(Debug, Clone)]
pub enum primary_exp {
    C1(primary_expC1),
    cconst(cconst),
    adj_strings(adj_strings),
    exp(exp),
}
pub fn primary_exp_c1(_ctx: &Ctx, var_ref: id) -> primary_exp {
    primary_exp::C1(primary_expC1 {
        var_ref: Box::new(var_ref),
    })
}
pub fn primary_exp_cconst(_ctx: &Ctx, cconst: cconst) -> primary_exp {
    primary_exp::cconst(cconst)
}
pub fn primary_exp_adj_strings(_ctx: &Ctx, adj_strings: adj_strings) -> primary_exp {
    primary_exp::adj_strings(adj_strings)
}
pub fn primary_exp_exp(_ctx: &Ctx, exp: exp) -> primary_exp {
    primary_exp::exp(exp)
}
#[derive(Debug, Clone)]
pub enum cconst {
    int_const(int_const),
    char_const(char_const),
    float_const(float_const),
    hexadecimal(hexadecimal),
}
pub fn cconst_int_const(_ctx: &Ctx, int_const: int_const) -> cconst {
    cconst::int_const(int_const)
}
pub fn cconst_char_const(_ctx: &Ctx, char_const: char_const) -> cconst {
    cconst::char_const(char_const)
}
pub fn cconst_float_const(_ctx: &Ctx, float_const: float_const) -> cconst {
    cconst::float_const(float_const)
}
pub fn cconst_hexadecimal(_ctx: &Ctx, hexadecimal: hexadecimal) -> cconst {
    cconst::hexadecimal(hexadecimal)
}
pub type adj_strings = string1;
pub fn adj_strings_string1(_ctx: &Ctx, string1: string1) -> adj_strings {
    string1
}
pub type string1 = Vec<string>;
pub fn string1_c1(_ctx: &Ctx, mut string1: string1, string: string) -> string1 {
    string1.push(string);
    string1
}
pub fn string1_string(_ctx: &Ctx, string: string) -> string1 {
    vec![string]
}
#[derive(Debug, Clone)]
pub struct hexadecimal {
    pub hex_const: hex_const,
    pub integer_suffix_opt_opt: integer_suffix_optOpt,
}
pub fn hexadecimal_c1(
    _ctx: &Ctx,
    hex_const: hex_const,
    integer_suffix_opt_opt: integer_suffix_optOpt,
) -> hexadecimal {
    hexadecimal {
        hex_const,
        integer_suffix_opt_opt,
    }
}
pub type integer_suffix_optOpt = Option<integer_suffix_opt>;
pub fn integer_suffix_opt_opt_integer_suffix_opt(
    _ctx: &Ctx,
    integer_suffix_opt: integer_suffix_opt,
) -> integer_suffix_optOpt {
    Some(integer_suffix_opt)
}
pub fn integer_suffix_opt_opt_empty(_ctx: &Ctx) -> integer_suffix_optOpt {
    None
}
