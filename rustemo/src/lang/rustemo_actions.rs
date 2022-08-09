use std::collections::BTreeMap;

use rustemo_rt::lexer::Token;

pub type StrConst = String;
pub fn str_const<'i>(token: Token<&'i str>) -> StrConst {
    token.value.trim_matches('\'').trim_matches('"').into()
}

pub type Name = String;
pub fn name<'a>(token: Token<&'a str>) -> Name {
    token.value.into()
}

pub type IntConst = u32;
pub fn int_const<'a>(token: Token<&'a str>) -> IntConst {
    token.value.parse().unwrap()
}

pub type FloatConst = f32;
pub fn float_const<'a>(token: Token<&'a str>) -> FloatConst {
    token.value.parse().unwrap()
}

pub type BoolConst = bool;
pub fn bool_const<'a>(token: Token<&'a str>) -> BoolConst {
    if token.value == "true" {
        true
    } else {
        false
    }
}

pub type Action = String;
pub fn action<'a>(token: Token<&'a str>) -> Action {
    token.value.into()
}

pub type ActionOpt = Option<Action>;
pub fn action_opt_p0(action: Action) -> ActionOpt {
    Some(action)
}
pub fn action_opt_p1() -> ActionOpt {
    None
}

pub type RegexTerm = String;
pub fn regex_term<'a>(token: Token<&'a str>) -> RegexTerm {
    token.value[1..token.value.len() - 1]
        .replace(r"\/", "/")
        .into()
}

pub type WS = String;
pub fn ws<'a>(token: Token<&'a str>) -> WS {
    token.value.into()
}

pub type CommentLine = String;
pub fn comment_line<'a>(token: Token<&'a str>) -> CommentLine {
    token.value.into()
}

pub type NotComment = String;
pub fn not_comment<'a>(token: Token<&'a str>) -> NotComment {
    token.value.into()
}

#[derive(Debug)]
pub struct PGFile {
    pub imports: Option<Imports>,
    pub rules: Option<GrammarRules>,
    pub terminals: Option<Terminals>,
}
pub fn pg_file_p0(rules: GrammarRules) -> PGFile {
    PGFile {
        imports: None,
        rules: Some(rules),
        terminals: None,
    }
}
pub fn pg_file_p1(imports: Imports, rules: GrammarRules) -> PGFile {
    PGFile {
        imports: Some(imports),
        rules: Some(rules),
        terminals: None,
    }
}
pub fn pg_file_p2(rules: GrammarRules, terminals: Terminals) -> PGFile {
    PGFile {
        imports: None,
        rules: Some(rules),
        terminals: Some(terminals),
    }
}
pub fn pg_file_p3(
    imports: Imports,
    rules: GrammarRules,
    terminals: Terminals,
) -> PGFile {
    PGFile {
        imports: Some(imports),
        rules: Some(rules),
        terminals: Some(terminals),
    }
}
pub fn pg_file_p4(terminals: Terminals) -> PGFile {
    PGFile {
        imports: None,
        rules: None,
        terminals: Some(terminals),
    }
}

pub type Imports = Vec<ImportStm>;
pub fn imports_p0(mut imports: Imports, import: ImportStm) -> Imports {
    imports.push(import);
    imports
}
pub fn imports_p1(import: ImportStm) -> Imports {
    vec![import]
}

#[derive(Debug)]
pub struct ImportStm {
    pub path: String,
    pub name: Option<String>,
}
pub fn import_stm_p0(path: StrConst) -> ImportStm {
    ImportStm { path, name: None }
}
pub fn import_stm_p1(path: StrConst, name: Name) -> ImportStm {
    ImportStm {
        path,
        name: Some(name),
    }
}

pub type GrammarRules = Vec<GrammarRule>;
pub fn grammar_rules_p0(
    mut rules: GrammarRules,
    rule: GrammarRule,
) -> GrammarRules {
    rules.push(rule);
    rules
}
pub fn grammar_rules_p1(rule: GrammarRule) -> GrammarRules {
    vec![rule]
}

#[derive(Debug)]
pub struct GrammarRule {
    pub action: Option<Action>,
    pub name: String,
    pub rhs: GrammarRuleRHS,
    pub meta: ProdMetaDatas,
}
pub fn grammar_rule_p0(
    action: Option<Action>,
    name: Name,
    rhs: GrammarRuleRHS,
) -> GrammarRule {
    GrammarRule {
        action,
        name,
        rhs,
        meta: ProdMetaDatas::new(),
    }
}
pub fn grammar_rule_p1(
    action: Option<Action>,
    name: Name,
    meta: ProdMetaDatas,
    rhs: GrammarRuleRHS,
) -> GrammarRule {
    GrammarRule {
        action,
        name,
        rhs,
        meta,
    }
}

pub type GrammarRuleRHS = Vec<Production>;
pub fn grammar_rule_rhs_p0(
    mut rhs: GrammarRuleRHS,
    prod: Production,
) -> GrammarRuleRHS {
    rhs.push(prod);
    rhs
}
pub fn grammar_rule_rhs_p1(prod: Production) -> GrammarRuleRHS {
    vec![prod]
}

#[derive(Debug)]
pub struct Production {
    pub assignments: Assignments,
    pub meta: ProdMetaDatas,
}
pub fn production_p0(assignments: Assignments) -> Production {
    Production {
        assignments,
        meta: ProdMetaDatas::new(),
    }
}
pub fn production_p1(
    assignments: Assignments,
    meta: ProdMetaDatas,
) -> Production {
    Production { assignments, meta }
}

pub type Terminals = Vec<Terminal>;
pub type TerminalRules = Terminals;
pub fn terminal_rules_p0(mut rules: Terminals, rule: Terminal) -> Terminals {
    rules.push(rule);
    rules
}
pub fn terminal_rules_p1(rule: Terminal) -> Terminals {
    vec![rule]
}

#[derive(Debug)]
pub struct Terminal {
    pub name: String,
    pub action: Option<String>,
    pub recognizer: Option<Recognizer>,
    pub meta: TermMetaDatas,
}
pub type TerminalRule = Terminal;
pub fn terminal_rule_p0(
    action: Option<Action>,
    name: String,
    recognizer: Recognizer,
) -> Terminal {
    Terminal {
        name,
        action,
        recognizer: Some(recognizer),
        meta: TermMetaDatas::new(),
    }
}
pub fn terminal_rule_p1(action: Option<Action>, name: String) -> Terminal {
    Terminal {
        name,
        action,
        recognizer: None,
        meta: TermMetaDatas::new(),
    }
}
pub fn terminal_rule_p2(
    action: Option<Action>,
    name: String,
    recognizer: Recognizer,
    meta: TermMetaDatas,
) -> Terminal {
    Terminal {
        action,
        name,
        recognizer: Some(recognizer),
        meta,
    }
}
pub fn terminal_rule_p3(
    action: Option<Action>,
    name: String,
    meta: TermMetaDatas,
) -> Terminal {
    Terminal {
        name,
        action,
        recognizer: None,
        meta,
    }
}

pub type ProdKind = String;
pub fn prod_kind_p0(name: Name) -> Name {
    name
}

pub type ProdMetaData = BTreeMap<String, ConstVal>;

pub fn prod_meta_data_p0() -> ProdMetaData {
    ProdMetaData::from([("left".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_p1() -> ProdMetaData {
    ProdMetaData::from([("left".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_p2() -> ProdMetaData {
    ProdMetaData::from([("right".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_p3() -> ProdMetaData {
    ProdMetaData::from([("right".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_p4() -> ProdMetaData {
    ProdMetaData::from([("dynamic".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_p5() -> ProdMetaData {
    ProdMetaData::from([("nops".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_p6() -> ProdMetaData {
    ProdMetaData::from([("nopse".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_p7(prio: IntConst) -> ProdMetaData {
    ProdMetaData::from([("priority".into(), ConstVal::Int(prio))])
}
pub fn prod_meta_data_p8(user: UserMetaData) -> ProdMetaData {
    ProdMetaData::from([(user.name, user.value)])
}
pub fn prod_meta_data_p9(kind: Name) -> ProdMetaData {
    ProdMetaData::from([("kind".into(), ConstVal::String(kind))])
}

pub type ProdMetaDatas = ProdMetaData;
pub fn prod_meta_datas_p0(
    mut metas: ProdMetaDatas,
    meta: ProdMetaData,
) -> ProdMetaDatas {
    metas.extend(meta);
    metas
}
pub fn prod_meta_datas_p1(meta: ProdMetaData) -> ProdMetaDatas {
    meta
}

pub type TermMetaData = BTreeMap<String, ConstVal>;
pub fn term_meta_data_p0() -> TermMetaData {
    TermMetaData::from([("prefer".into(), ConstVal::Bool(true))])
}
pub fn term_meta_data_p1() -> TermMetaData {
    TermMetaData::from([("finish".into(), ConstVal::Bool(true))])
}
pub fn term_meta_data_p2() -> TermMetaData {
    TermMetaData::from([("finish".into(), ConstVal::Bool(false))])
}
pub fn term_meta_data_p3() -> TermMetaData {
    TermMetaData::from([("dynamic".into(), ConstVal::Bool(true))])
}
pub fn term_meta_data_p4(prio: IntConst) -> TermMetaData {
    TermMetaData::from([("priority".into(), ConstVal::Int(prio))])
}
pub fn term_meta_data_p5(user: UserMetaData) -> TermMetaData {
    TermMetaData::from([(user.name, user.value)])
}

pub type TermMetaDatas = TermMetaData;
pub fn term_meta_datas_p0(
    mut metas: TermMetaDatas,
    meta: TermMetaData,
) -> TermMetaDatas {
    metas.extend(meta);
    metas
}
pub fn term_meta_datas_p1(meta: TermMetaData) -> TermMetaDatas {
    meta
}

#[derive(Debug)]
pub struct UserMetaData {
    name: Name,
    value: ConstVal,
}
pub fn user_meta_data_p0(name: Name, value: ConstVal) -> UserMetaData {
    UserMetaData { name, value }
}

#[derive(Debug)]
pub enum ConstVal {
    Int(u32),
    Float(f32),
    Bool(bool),
    String(String),
}
pub fn const_val_p0(int_const: IntConst) -> ConstVal {
    ConstVal::Int(int_const)
}
pub fn const_val_p1(float_const: FloatConst) -> ConstVal {
    ConstVal::Float(float_const)
}
pub fn const_val_p2(bool_const: BoolConst) -> ConstVal {
    ConstVal::Bool(bool_const)
}
pub fn const_val_p3(str_const: StrConst) -> ConstVal {
    ConstVal::String(str_const)
}

#[derive(Debug)]
pub enum Assignment {
    PlainAssignment(PlainAssignment),
    BoolAssignment(BoolAssignment),
    GrammarSymbolRef(GrammarSymbolRef),
}
pub fn assignment_p0(assig: PlainAssignment) -> Assignment {
    Assignment::PlainAssignment(assig)
}
pub fn assignment_p1(assig: BoolAssignment) -> Assignment {
    Assignment::BoolAssignment(assig)
}
pub fn assignment_p2(gsymref: GrammarSymbolRef) -> Assignment {
    Assignment::GrammarSymbolRef(gsymref)
}

pub type Assignments = Vec<Assignment>;
pub fn assignments_p0(
    mut assigns: Assignments,
    assign: Assignment,
) -> Assignments {
    assigns.push(assign);
    assigns
}
pub fn assignments_p1(assign: Assignment) -> Assignments {
    vec![assign]
}

#[derive(Debug)]
pub struct PlainAssignment {
    pub name: Name,
    pub gsymref: GrammarSymbolRef,
}
pub fn plain_assignment_p0(
    name: Name,
    gsymref: GrammarSymbolRef,
) -> PlainAssignment {
    PlainAssignment { name, gsymref }
}

pub type BoolAssignment = PlainAssignment;
pub fn bool_assignment_p0(
    name: Name,
    gsymref: GrammarSymbolRef,
) -> BoolAssignment {
    BoolAssignment { name, gsymref }
}

#[derive(Debug)]
pub struct ProductionGroup(pub GrammarRuleRHS);
pub fn production_group_p0(prod_rule_rhs: GrammarRuleRHS) -> ProductionGroup {
    ProductionGroup(prod_rule_rhs)
}

#[derive(Debug)]
pub struct GrammarSymbolRef {
    pub gsymbol: Option<GrammarSymbol>,
    pub repetition_operator: RepetitionOperatorOpt,
    pub production_group: Option<ProductionGroup>,
}
pub fn grammar_symbol_ref_p0(
    gsymbol: GrammarSymbol,
    repetition_operator: RepetitionOperatorOpt,
) -> GrammarSymbolRef {
    GrammarSymbolRef {
        gsymbol: Some(gsymbol),
        repetition_operator,
        production_group: None,
    }
}
pub fn grammar_symbol_ref_p1(
    prod_group: ProductionGroup,
    repetition_operator: RepetitionOperatorOpt,
) -> GrammarSymbolRef {
    GrammarSymbolRef {
        gsymbol: None,
        repetition_operator,
        production_group: Some(prod_group),
    }
}

#[derive(Debug)]
pub struct RepetitionOperator {
    pub repetition_operator_op: RepetitionOperatorOp,
    pub repetition_modifiers_exp: Option<RepetitionModifiersExp>,
}
pub fn repetition_operator_p0(
    repetition_operator_op: RepetitionOperatorOp,
    repetition_modifiers_exp: Option<RepetitionModifiersExp>,
) -> RepetitionOperator {
    RepetitionOperator {
        repetition_operator_op,
        repetition_modifiers_exp,
    }
}

pub type RepetitionOperatorOpt = Option<RepetitionOperator>;
pub fn repetition_operator_opt_p0(
    repetition_operator: RepetitionOperator,
) -> Option<RepetitionOperator> {
    Some(repetition_operator)
}
pub fn repetition_operator_opt_p1() -> Option<RepetitionOperator> {
    None
}

#[derive(Debug)]
pub enum RepetitionOperatorOp {
    ZeroOrMore,
    ZeroOrMoreGreedy,
    OneOrMore,
    OneOrMoreGreedy,
    Optional,
    OptionalGreedy,
}
pub fn repetition_operator_op_p0() -> RepetitionOperatorOp {
    RepetitionOperatorOp::ZeroOrMore
}
pub fn repetition_operator_op_p1() -> RepetitionOperatorOp {
    RepetitionOperatorOp::ZeroOrMoreGreedy
}
pub fn repetition_operator_op_p2() -> RepetitionOperatorOp {
    RepetitionOperatorOp::OneOrMore
}
pub fn repetition_operator_op_p3() -> RepetitionOperatorOp {
    RepetitionOperatorOp::OneOrMoreGreedy
}
pub fn repetition_operator_op_p4() -> RepetitionOperatorOp {
    RepetitionOperatorOp::Optional
}
pub fn repetition_operator_op_p5() -> RepetitionOperatorOp {
    RepetitionOperatorOp::OptionalGreedy
}

pub type RepetitionModifiersExp = RepetitionModifiers;
pub type RepetitionModifiersExpOpt = Option<RepetitionModifiers>;
pub fn repetition_modifiers_exp_p0(
    repetition_modifiers: RepetitionModifiers,
) -> RepetitionModifiersExp {
    repetition_modifiers
}

pub fn repetition_modifiers_exp_opt_p0(
    repetitioni_modifiers_exp: RepetitionModifiersExp,
) -> Option<RepetitionModifiersExp> {
    Some(repetitioni_modifiers_exp)
}
pub fn repetition_modifiers_exp_opt_p1() -> Option<RepetitionModifiersExp> {
    None
}

pub type RepetitionModifiers = Vec<RepetitionModifier>;
pub fn repetition_modifiers_p0(
    mut repetition_modifiers: RepetitionModifiers,
    repetition_modifier: RepetitionModifier,
) -> RepetitionModifiers {
    repetition_modifiers.push(repetition_modifier);
    repetition_modifiers
}
pub fn repetition_modifiers_p1(
    repetition_modifier: RepetitionModifier,
) -> RepetitionModifiers {
    vec![repetition_modifier]
}

#[derive(Debug)]
pub struct RepetitionModifier(pub Name);
pub fn repetition_modifier_p0(name: Name) -> RepetitionModifier {
    RepetitionModifier(name)
}

#[derive(Debug)]
pub enum GrammarSymbol {
    Name(Name),
    StrConst(StrConst),
}
pub fn grammar_symbol_p0(name: Name) -> GrammarSymbol {
    GrammarSymbol::Name(name)
}
pub fn grammar_symbol_p1(str_const: StrConst) -> GrammarSymbol {
    GrammarSymbol::StrConst(str_const)
}

#[derive(Debug)]
pub enum Recognizer {
    StrConst(StrConst),
    RegExTerm(RegexTerm),
}
pub fn recognizer_p0(str_const: StrConst) -> Recognizer {
    Recognizer::StrConst(str_const)
}
pub fn recognizer_p1(regex: RegexTerm) -> Recognizer {
    Recognizer::RegExTerm(regex)
}

pub type Layout = String;
pub fn layout_p0(item: LayoutItem) -> Layout {
    item
}
pub fn layout_p1(mut layout: Layout, item: LayoutItem) -> Layout {
    layout.push_str(&item);
    layout
}
pub fn layout_p2() -> Layout {
    "".into()
}

pub type LayoutItem = String;
pub fn layout_item_p0(ws: WS) -> LayoutItem {
    ws
}
pub fn layout_item_p1(comment: Comment) -> LayoutItem {
    comment
}

pub type Comment = String;
pub fn comment_p0(s: Corncs) -> Comment {
    s
}
pub fn comment_p1(s: CommentLine) -> Comment {
    s
}

pub type Corncs = String;
pub fn corncs_p0(s: Cornc) -> Corncs {
    s
}
pub fn corncs_p1(mut ss: Corncs, s: Cornc) -> Corncs {
    ss.push_str(&s);
    ss
}
pub fn corncs_p2() -> Corncs {
    "".into()
}

pub type Cornc = String;
pub fn cornc_p0(s: Comment) -> Cornc {
    s
}
pub fn cornc_p1(s: NotComment) -> Cornc {
    s
}
pub fn cornc_p2(s: WS) -> Cornc {
    s
}
