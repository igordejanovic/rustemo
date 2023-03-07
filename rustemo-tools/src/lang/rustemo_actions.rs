///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use super::rustemo::{Context, TokenKind};
use rustemo::{lexer, location::ValLoc};
use std::collections::BTreeMap;
pub type Name = ValLoc<String>;
pub type Token<'i> = lexer::Token<'i, str, TokenKind>;
pub fn name(ctx: &Context, token: Token) -> Name {
    Name::new(token.value.into(), Some(ctx.location))
}
pub type RegexTerm = String;
pub fn regex_term(_ctx: &Context, token: Token) -> RegexTerm {
    token.value[1..token.value.len() - 1].replace(r"\/", "/")
}
pub type IntConst = ValLoc<u32>;
pub fn int_const(ctx: &Context, token: Token) -> IntConst {
    IntConst::new(token.value.parse().unwrap(), Some(ctx.location))
}
pub type FloatConst = ValLoc<f32>;
pub fn float_const(ctx: &Context, token: Token) -> FloatConst {
    FloatConst::new(token.value.parse().unwrap(), Some(ctx.location))
}
pub type BoolConst = ValLoc<bool>;
pub fn bool_const(ctx: &Context, token: Token) -> BoolConst {
    BoolConst::new(token.value == "true", Some(ctx.location))
}
pub type StrConst = ValLoc<String>;
pub fn str_const(ctx: &Context, token: Token) -> StrConst {
    StrConst::new(
        token.value.trim_matches('\'').trim_matches('"').into(),
        Some(ctx.location),
    )
}
pub type Action = ValLoc<String>;
pub fn action(ctx: &Context, token: Token) -> Action {
    Action::new(token.value[1..].into(), Some(ctx.location))
}
#[derive(Debug, Clone, Default)]
pub struct File {
    pub file: String,
    pub imports: Option<Imports>,
    pub grammar_rules: Option<GrammarRules>,
    pub terminal_rules: Option<TerminalRules>,
}
pub fn file_c1(ctx: &Context, grammar_rules: GrammarRules) -> File {
    File {
        file: ctx.file.clone(),
        grammar_rules: Some(grammar_rules),
        ..Default::default()
    }
}
pub fn file_c2(ctx: &Context, imports: Imports, grammar_rules: GrammarRules) -> File {
    File {
        file: ctx.file.clone(),
        imports: Some(imports),
        grammar_rules: Some(grammar_rules),
        terminal_rules: None,
    }
}
pub fn file_c3(
    ctx: &Context,
    grammar_rules: GrammarRules,
    terminal_rules: TerminalRules,
) -> File {
    File {
        file: ctx.file.clone(),
        grammar_rules: Some(grammar_rules),
        terminal_rules: Some(terminal_rules),
        imports: None,
    }
}
pub fn file_c4(
    ctx: &Context,
    imports: Imports,
    grammar_rules: GrammarRules,
    terminal_rules: TerminalRules,
) -> File {
    File {
        file: ctx.file.clone(),
        imports: Some(imports),
        grammar_rules: Some(grammar_rules),
        terminal_rules: Some(terminal_rules),
    }
}
pub fn file_c5(ctx: &Context, terminal_rules: TerminalRules) -> File {
    File {
        file: ctx.file.clone(),
        terminal_rules: Some(terminal_rules),
        ..Default::default()
    }
}
pub type ImportStm1 = Vec<ImportStm>;
pub type Imports = ImportStm1;
pub fn import_stm1_c1(
    _ctx: &Context,
    mut import_stm1: ImportStm1,
    import_stm: ImportStm,
) -> ImportStm1 {
    import_stm1.push(import_stm);
    import_stm1
}
pub fn import_stm1_c2(_ctx: &Context, import_stm: ImportStm) -> ImportStm1 {
    vec![import_stm]
}
#[derive(Debug, Clone)]
pub struct ImportStm {
    pub path: StrConst,
    pub name: Option<Name>,
}
pub fn import_stm_c1(_ctx: &Context, path: StrConst) -> ImportStm {
    ImportStm { path, name: None }
}
pub fn import_stm_c2(_ctx: &Context, path: StrConst, name: Name) -> ImportStm {
    ImportStm {
        path,
        name: Some(name),
    }
}
pub type GrammarRule1 = Vec<GrammarRule>;
pub type GrammarRules = GrammarRule1;
pub fn grammar_rule1_c1(
    _ctx: &Context,
    mut grammar_rule1: GrammarRule1,
    grammar_rule: GrammarRule,
) -> GrammarRule1 {
    grammar_rule1.push(grammar_rule);
    grammar_rule1
}
pub fn grammar_rule1_c2(_ctx: &Context, grammar_rule: GrammarRule) -> GrammarRule1 {
    vec![grammar_rule]
}
#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub action: Option<Action>,
    pub name: Name,
    pub rhs: GrammarRuleRHS,
    pub meta: ProdMetaDatas,
}
pub fn grammar_rule_c1(
    _ctx: &Context,
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
pub fn grammar_rule_c2(
    _ctx: &Context,
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
pub type ActionOpt = Option<Action>;
pub fn action_opt_c1(_ctx: &Context, action: Action) -> ActionOpt {
    Some(action)
}
pub fn action_opt_empty(_ctx: &Context) -> ActionOpt {
    None
}
pub type GrammarRuleRHS = Vec<Production>;
pub fn grammar_rule_rhs_c1(
    _ctx: &Context,
    mut rhs: GrammarRuleRHS,
    production: Production,
) -> GrammarRuleRHS {
    rhs.push(production);
    rhs
}
pub fn grammar_rule_rhs_c2(_ctx: &Context, production: Production) -> GrammarRuleRHS {
    vec![production]
}
#[derive(Debug, Clone)]
pub struct Production {
    pub assignments: Assignments,
    pub meta: ProdMetaDatas,
}
pub fn production_c1(_ctx: &Context, assignments: Assignments) -> Production {
    Production {
        assignments,
        meta: ProdMetaDatas::new(),
    }
}
pub fn production_c2(
    _ctx: &Context,
    assignments: Assignments,
    meta: ProdMetaDatas,
) -> Production {
    Production { assignments, meta }
}
pub type TerminalRule1 = Vec<TerminalRule>;
pub type TerminalRules = TerminalRule1;
pub fn terminal_rule1_c1(
    _ctx: &Context,
    mut terminal_rule1: TerminalRule1,
    terminal_rule: TerminalRule,
) -> TerminalRule1 {
    terminal_rule1.push(terminal_rule);
    terminal_rule1
}
pub fn terminal_rule1_c2(_ctx: &Context, terminal_rule: TerminalRule) -> TerminalRule1 {
    vec![terminal_rule]
}
#[derive(Debug, Clone)]
pub struct TerminalRule {
    pub name: Name,
    pub action: Option<Action>,
    pub recognizer: Option<Recognizer>,
    pub meta: TermMetaDatas,
}
pub type Terminal = TerminalRule;
pub fn terminal_rule_c1(
    _ctx: &Context,
    action: Option<Action>,
    name: Name,
    recognizer: Recognizer,
) -> TerminalRule {
    TerminalRule {
        action,
        name,
        recognizer: Some(recognizer),
        meta: TermMetaDatas::new(),
    }
}
pub fn terminal_rule_c2(
    _ctx: &Context,
    action: Option<Action>,
    name: Name,
) -> TerminalRule {
    TerminalRule {
        action,
        name,
        recognizer: None,
        meta: TermMetaDatas::new(),
    }
}
pub fn terminal_rule_c3(
    _ctx: &Context,
    action: Option<Action>,
    name: Name,
    recognizer: Recognizer,
    meta: TermMetaDatas,
) -> TerminalRule {
    TerminalRule {
        action,
        name,
        recognizer: Some(recognizer),
        meta,
    }
}
pub fn terminal_rule_c4(
    _ctx: &Context,
    action: Option<Action>,
    name: Name,
    meta: TermMetaDatas,
) -> TerminalRule {
    TerminalRule {
        action,
        name,
        recognizer: None,
        meta,
    }
}
pub type ProdMetaData = BTreeMap<String, ConstVal>;
pub fn prod_meta_data_left(_ctx: &Context) -> ProdMetaData {
    ProdMetaData::from([("left".into(), ConstVal::Bool(true.into()))])
}
pub fn prod_meta_data_reduce(_ctx: &Context) -> ProdMetaData {
    ProdMetaData::from([("left".into(), ConstVal::Bool(true.into()))])
}
pub fn prod_meta_data_right(_ctx: &Context) -> ProdMetaData {
    ProdMetaData::from([("right".into(), ConstVal::Bool(true.into()))])
}
pub fn prod_meta_data_shift(_ctx: &Context) -> ProdMetaData {
    ProdMetaData::from([("right".into(), ConstVal::Bool(true.into()))])
}
pub fn prod_meta_data_dynamic(_ctx: &Context) -> ProdMetaData {
    ProdMetaData::from([("dynamic".into(), ConstVal::Bool(true.into()))])
}
pub fn prod_meta_data_nops(_ctx: &Context) -> ProdMetaData {
    ProdMetaData::from([("nops".into(), ConstVal::Bool(true.into()))])
}
pub fn prod_meta_data_nopse(_ctx: &Context) -> ProdMetaData {
    ProdMetaData::from([("nopse".into(), ConstVal::Bool(true.into()))])
}
pub fn prod_meta_data_priority(_ctx: &Context, prio: IntConst) -> ProdMetaData {
    ProdMetaData::from([("priority".into(), ConstVal::Int(prio))])
}
pub fn prod_meta_data_c9(_ctx: &Context, user: UserMetaData) -> ProdMetaData {
    ProdMetaData::from([(user.name.into(), user.value)])
}
pub fn prod_meta_data_c10(_ctx: &Context, prod_kind: ProdKind) -> ProdMetaData {
    ProdMetaData::from([("kind".into(), ConstVal::String(prod_kind))])
}
pub type ProdMetaDatas = ProdMetaData;
pub fn prod_meta_datas_c1(
    _ctx: &Context,
    mut metas: ProdMetaDatas,
    meta: ProdMetaData,
) -> ProdMetaDatas {
    metas.extend(meta);
    metas
}
pub fn prod_meta_datas_c2(_ctx: &Context, meta: ProdMetaData) -> ProdMetaDatas {
    meta
}
pub type TermMetaData = BTreeMap<String, ConstVal>;
pub fn term_meta_data_prefer(_ctx: &Context) -> TermMetaData {
    TermMetaData::from([("prefer".into(), ConstVal::Bool(true.into()))])
}
pub fn term_meta_data_finish(_ctx: &Context) -> TermMetaData {
    TermMetaData::from([("finish".into(), ConstVal::Bool(true.into()))])
}
pub fn term_meta_data_no_finish(_ctx: &Context) -> TermMetaData {
    TermMetaData::from([("finish".into(), ConstVal::Bool(false.into()))])
}
pub fn term_meta_data_dynamic(_ctx: &Context) -> TermMetaData {
    TermMetaData::from([("dynamic".into(), ConstVal::Bool(true.into()))])
}
pub fn term_meta_data_priority(_ctx: &Context, prio: IntConst) -> TermMetaData {
    TermMetaData::from([("priority".into(), ConstVal::Int(prio))])
}
pub fn term_meta_data_c6(_ctx: &Context, user: UserMetaData) -> TermMetaData {
    TermMetaData::from([(user.name.into(), user.value)])
}
pub type TermMetaDatas = TermMetaData;
pub fn term_meta_datas_c1(
    _ctx: &Context,
    mut metas: TermMetaDatas,
    meta: TermMetaData,
) -> TermMetaDatas {
    metas.extend(meta);
    metas
}
pub fn term_meta_datas_c2(_ctx: &Context, meta: TermMetaData) -> TermMetaDatas {
    meta
}
#[derive(Debug, Clone)]
pub struct UserMetaData {
    pub name: Name,
    pub value: ConstVal,
}
pub fn user_meta_data_c1(_ctx: &Context, name: Name, value: ConstVal) -> UserMetaData {
    UserMetaData { name, value }
}
pub type ProdKind = Name;
pub fn prod_kind_c1(_ctx: &Context, name: Name) -> ProdKind {
    name
}
#[derive(Debug, Clone)]
pub enum ConstVal {
    Int(IntConst),
    Float(FloatConst),
    Bool(BoolConst),
    String(StrConst),
}
pub fn const_val_c1(_ctx: &Context, int_const: IntConst) -> ConstVal {
    ConstVal::Int(int_const)
}
pub fn const_val_c2(_ctx: &Context, float_const: FloatConst) -> ConstVal {
    ConstVal::Float(float_const)
}
pub fn const_val_c3(_ctx: &Context, bool_const: BoolConst) -> ConstVal {
    ConstVal::Bool(bool_const)
}
pub fn const_val_c4(_ctx: &Context, str_const: StrConst) -> ConstVal {
    ConstVal::String(str_const)
}
#[derive(Debug, Clone)]
pub enum Assignment {
    PlainAssignment(PlainAssignment),
    BoolAssignment(BoolAssignment),
    GrammarSymbolRef(GrammarSymbolRef),
}
pub fn assignment_c1(_ctx: &Context, plain_assignment: PlainAssignment) -> Assignment {
    Assignment::PlainAssignment(plain_assignment)
}
pub fn assignment_c2(_ctx: &Context, bool_assignment: BoolAssignment) -> Assignment {
    Assignment::BoolAssignment(bool_assignment)
}
pub fn assignment_c3(
    _ctx: &Context,
    grammar_symbol_ref: GrammarSymbolRef,
) -> Assignment {
    Assignment::GrammarSymbolRef(grammar_symbol_ref)
}
pub type Assignment1 = Vec<Assignment>;
pub type Assignments = Assignment1;
pub fn assignment1_c1(
    _ctx: &Context,
    mut assignment1: Assignment1,
    assignment: Assignment,
) -> Assignment1 {
    assignment1.push(assignment);
    assignment1
}
pub fn assignment1_c2(_ctx: &Context, assignment: Assignment) -> Assignment1 {
    vec![assignment]
}
#[derive(Debug, Clone)]
pub struct NamedAssignment {
    pub name: Name,
    pub gsymref: GrammarSymbolRef,
}
pub type PlainAssignment = NamedAssignment;
pub fn plain_assignment_c1(
    _ctx: &Context,
    name: Name,
    gsymref: GrammarSymbolRef,
) -> PlainAssignment {
    PlainAssignment { name, gsymref }
}
pub type BoolAssignment = NamedAssignment;
pub fn bool_assignment_c1(
    _ctx: &Context,
    name: Name,
    gsymref: GrammarSymbolRef,
) -> BoolAssignment {
    BoolAssignment { name, gsymref }
}
#[derive(Debug, Clone)]
pub struct ProductionGroup(pub GrammarRuleRHS);
pub fn production_group_c1(
    _ctx: &Context,
    prod_rule_rhs: GrammarRuleRHS,
) -> ProductionGroup {
    ProductionGroup(prod_rule_rhs)
}
#[derive(Debug, Clone)]
pub struct GrammarSymbolRef {
    pub gsymbol: Option<GrammarSymbol>,
    pub repetition_op: RepetitionOperatorOpt,
    pub production_group: Option<ProductionGroup>,
}
pub fn grammar_symbol_ref_c1(
    _ctx: &Context,
    gsymbol: GrammarSymbol,
    repetition_operator: RepetitionOperatorOpt,
) -> GrammarSymbolRef {
    GrammarSymbolRef {
        gsymbol: Some(gsymbol),
        repetition_op: repetition_operator,
        production_group: None,
    }
}
pub fn grammar_symbol_ref_c2(
    _ctx: &Context,
    production_group: ProductionGroup,
    repetition_operator: RepetitionOperatorOpt,
) -> GrammarSymbolRef {
    GrammarSymbolRef {
        gsymbol: None,
        repetition_op: repetition_operator,
        production_group: Some(production_group),
    }
}
#[derive(Debug, Clone)]
pub struct RepetitionOperator {
    pub rep_op: RepetitionOperatorOp,
    pub rep_modifiers: Option<RepetitionModifiers>,
}
pub fn repetition_operator_c1(
    _ctx: &Context,
    repetition_operator_op: RepetitionOperatorOp,
    repetition_modifiers: Option<RepetitionModifiers>,
) -> RepetitionOperator {
    RepetitionOperator {
        rep_op: repetition_operator_op,
        rep_modifiers: repetition_modifiers,
    }
}
pub type RepetitionOperatorOpt = Option<RepetitionOperator>;
pub fn repetition_operator_opt_c1(
    _ctx: &Context,
    repetition_operator: RepetitionOperator,
) -> RepetitionOperatorOpt {
    Some(repetition_operator)
}
pub fn repetition_operator_opt_empty(_ctx: &Context) -> RepetitionOperatorOpt {
    None
}
#[derive(Debug, Clone)]
pub enum RepetitionOperatorOp {
    ZeroOrMore,
    ZeroOrMoreGreedy,
    OneOrMore,
    OneOrMoreGreedy,
    Optional,
    OptionalGreedy,
}
pub fn repetition_operator_op_zero_or_more(_ctx: &Context) -> RepetitionOperatorOp {
    RepetitionOperatorOp::ZeroOrMore
}
pub fn repetition_operator_op_zero_or_more_greedy(
    _ctx: &Context,
) -> RepetitionOperatorOp {
    RepetitionOperatorOp::ZeroOrMoreGreedy
}
pub fn repetition_operator_op_one_or_more(_ctx: &Context) -> RepetitionOperatorOp {
    RepetitionOperatorOp::OneOrMore
}
pub fn repetition_operator_op_one_or_more_greedy(
    _ctx: &Context,
) -> RepetitionOperatorOp {
    RepetitionOperatorOp::OneOrMoreGreedy
}
pub fn repetition_operator_op_optional(_ctx: &Context) -> RepetitionOperatorOp {
    RepetitionOperatorOp::Optional
}
pub fn repetition_operator_op_optional_greedy(_ctx: &Context) -> RepetitionOperatorOp {
    RepetitionOperatorOp::OptionalGreedy
}
pub type RepetitionModifiersOpt = Option<RepetitionModifiers>;
pub fn repetition_modifiers_opt_c1(
    _ctx: &Context,
    repetition_modifiers: RepetitionModifiers,
) -> RepetitionModifiersOpt {
    Some(repetition_modifiers)
}
pub fn repetition_modifiers_opt_empty(_ctx: &Context) -> RepetitionModifiersOpt {
    None
}
pub type RepetitionModifiers = Vec<RepetitionModifier>;
pub fn repetition_modifiers_c1(
    _ctx: &Context,
    repetition_modifier1: RepetitionModifier1,
) -> RepetitionModifiers {
    repetition_modifier1
}
pub type RepetitionModifier1 = Vec<RepetitionModifier>;
pub fn repetition_modifier1_c1(
    _ctx: &Context,
    mut repetition_modifier1: RepetitionModifier1,
    repetition_modifier: RepetitionModifier,
) -> RepetitionModifier1 {
    repetition_modifier1.push(repetition_modifier);
    repetition_modifier1
}
pub fn repetition_modifier1_c2(
    _ctx: &Context,
    repetition_modifier: RepetitionModifier,
) -> RepetitionModifier1 {
    vec![repetition_modifier]
}
pub type RepetitionModifier = Name;
pub fn repetition_modifier_c1(_ctx: &Context, name: Name) -> RepetitionModifier {
    name
}
#[derive(Debug, Clone)]
pub enum GrammarSymbol {
    Name(Name),
    StrConst(StrConst),
}
pub fn grammar_symbol_c1(_ctx: &Context, name: Name) -> GrammarSymbol {
    GrammarSymbol::Name(name)
}
pub fn grammar_symbol_c2(_ctx: &Context, str_const: StrConst) -> GrammarSymbol {
    GrammarSymbol::StrConst(str_const)
}
#[derive(Debug, Clone)]
pub enum Recognizer {
    StrConst(StrConst),
    RegexTerm(RegexTerm),
}
pub fn recognizer_c1(_ctx: &Context, str_const: StrConst) -> Recognizer {
    Recognizer::StrConst(str_const)
}
pub fn recognizer_c2(_ctx: &Context, regex_term: RegexTerm) -> Recognizer {
    Recognizer::RegexTerm(regex_term)
}
