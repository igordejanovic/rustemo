///! This file is maintained by rustemo but can be modified manually.
///! All manual changes will be preserved except non-doc comments.
use rustemo_rt::lexer::Token;
use std::collections::BTreeMap;
pub type Name = String;
pub fn name<I: AsRef<super::Input>>(token: Token<I>) -> Name {
    token.value.as_ref().into()
}
pub type RegexTerm = String;
pub fn regex_term<I: AsRef<super::Input>>(token: Token<I>) -> RegexTerm {
    token.value.as_ref()[1..token.value.as_ref().len() - 1].replace(r"\/", "/").into()
}
pub type IntConst = u32;
pub fn int_const<I: AsRef<super::Input>>(token: Token<I>) -> IntConst {
    token.value.as_ref().parse().unwrap()
}
pub type FloatConst = f32;
pub fn float_const<I: AsRef<super::Input>>(token: Token<I>) -> FloatConst {
    token.value.as_ref().parse().unwrap()
}
pub type BoolConst = bool;
pub fn bool_const<I: AsRef<super::Input>>(token: Token<I>) -> BoolConst {
    if token.value.as_ref() == "true" { true } else { false }
}
pub type StrConst = String;
pub fn str_const<I: AsRef<super::Input>>(token: Token<I>) -> StrConst {
    token.value.as_ref().trim_matches('\'').trim_matches('"').into()
}
pub type Action = String;
pub fn action<I: AsRef<super::Input>>(token: Token<I>) -> Action {
    token.value.as_ref()[1..].into()
}
pub type WS = String;
pub fn ws<I: AsRef<super::Input>>(token: Token<I>) -> WS {
    token.value.as_ref().into()
}
pub type CommentLine = String;
pub fn comment_line<I: AsRef<super::Input>>(token: Token<I>) -> CommentLine {
    token.value.as_ref().into()
}
pub type NotComment = String;
pub fn not_comment<I: AsRef<super::Input>>(token: Token<I>) -> NotComment {
    token.value.as_ref().into()
}
#[derive(Debug, Clone, Default)]
pub struct File {
    pub imports: Option<Imports>,
    pub grammar_rules: Option<GrammarRules>,
    pub terminal_rules: Option<TerminalRules>,
}
pub fn file_c1(grammar_rules: GrammarRules) -> File {
    File {
        grammar_rules: Some(grammar_rules),
        ..Default::default()
    }
}
pub fn file_c2(imports: Imports, grammar_rules: GrammarRules) -> File {
    File {
        imports: Some(imports),
        grammar_rules: Some(grammar_rules),
        terminal_rules: None,
    }
}
pub fn file_c3(grammar_rules: GrammarRules, terminal_rules: TerminalRules) -> File {
    File {
        grammar_rules: Some(grammar_rules),
        terminal_rules: Some(terminal_rules),
        imports: None,
    }
}
pub fn file_c4(
    imports: Imports,
    grammar_rules: GrammarRules,
    terminal_rules: TerminalRules,
) -> File {
    File {
        imports: Some(imports),
        grammar_rules: Some(grammar_rules),
        terminal_rules: Some(terminal_rules),
    }
}
pub fn file_c5(terminal_rules: TerminalRules) -> File {
    File {
        terminal_rules: Some(terminal_rules),
        ..Default::default()
    }
}
pub type ImportStm1 = Vec<ImportStm>;
pub type Imports = ImportStm1;
pub fn import_stm1_c1(mut import_stm1: ImportStm1, import_stm: ImportStm) -> ImportStm1 {
    import_stm1.push(import_stm);
    import_stm1
}
pub fn import_stm1_c2(import_stm: ImportStm) -> ImportStm1 {
    vec![import_stm]
}
#[derive(Debug, Clone)]
pub struct ImportStm {
    pub path: StrConst,
    pub name: Option<Name>,
}
pub fn import_stm_c1(path: StrConst) -> ImportStm {
    ImportStm { path, name: None }
}
pub fn import_stm_c2(path: StrConst, name: Name) -> ImportStm {
    ImportStm {
        path,
        name: Some(name),
    }
}
pub type GrammarRule1 = Vec<GrammarRule>;
pub type GrammarRules = GrammarRule1;
pub fn grammar_rule1_c1(
    mut grammar_rule1: GrammarRule1,
    grammar_rule: GrammarRule,
) -> GrammarRule1 {
    grammar_rule1.push(grammar_rule);
    grammar_rule1
}
pub fn grammar_rule1_c2(grammar_rule: GrammarRule) -> GrammarRule1 {
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
pub fn action_opt_c1(action: Action) -> ActionOpt {
    Some(action)
}
pub fn action_opt_empty() -> ActionOpt {
    None
}
pub type GrammarRuleRHS = Vec<Production>;
pub fn grammar_rule_rhs_c1(
    mut rhs: GrammarRuleRHS,
    production: Production,
) -> GrammarRuleRHS {
    rhs.push(production);
    rhs
}
pub fn grammar_rule_rhs_c2(production: Production) -> GrammarRuleRHS {
    vec![production]
}
#[derive(Debug, Clone)]
pub struct Production {
    pub assignments: Assignments,
    pub meta: ProdMetaDatas,
}
pub fn production_c1(assignments: Assignments) -> Production {
    Production {
        assignments,
        meta: ProdMetaDatas::new(),
    }
}
pub fn production_c2(assignments: Assignments, meta: ProdMetaDatas) -> Production {
    Production { assignments, meta }
}
pub type TerminalRule1 = Vec<TerminalRule>;
pub type TerminalRules = TerminalRule1;
pub fn terminal_rule1_c1(
    mut terminal_rule1: TerminalRule1,
    terminal_rule: TerminalRule,
) -> TerminalRule1 {
    terminal_rule1.push(terminal_rule);
    terminal_rule1
}
pub fn terminal_rule1_c2(terminal_rule: TerminalRule) -> TerminalRule1 {
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
pub fn terminal_rule_c2(action: Option<Action>, name: Name) -> TerminalRule {
    TerminalRule {
        action,
        name,
        recognizer: None,
        meta: TermMetaDatas::new(),
    }
}
pub fn terminal_rule_c3(
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
pub fn prod_meta_data_left() -> ProdMetaData {
    ProdMetaData::from([("left".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_reduce() -> ProdMetaData {
    ProdMetaData::from([("left".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_right() -> ProdMetaData {
    ProdMetaData::from([("right".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_shift() -> ProdMetaData {
    ProdMetaData::from([("right".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_dynamic() -> ProdMetaData {
    ProdMetaData::from([("dynamic".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_nops() -> ProdMetaData {
    ProdMetaData::from([("nops".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_nopse() -> ProdMetaData {
    ProdMetaData::from([("nopse".into(), ConstVal::Bool(true))])
}
pub fn prod_meta_data_priority(prio: IntConst) -> ProdMetaData {
    ProdMetaData::from([("priority".into(), ConstVal::Int(prio))])
}
pub fn prod_meta_data_c9(user: UserMetaData) -> ProdMetaData {
    ProdMetaData::from([(user.name, user.value)])
}
pub fn prod_meta_data_c10(prod_kind: ProdKind) -> ProdMetaData {
    ProdMetaData::from([("kind".into(), ConstVal::String(prod_kind))])
}
pub type ProdMetaDatas = ProdMetaData;
pub fn prod_meta_datas_c1(
    mut metas: ProdMetaDatas,
    meta: ProdMetaData,
) -> ProdMetaDatas {
    metas.extend(meta);
    metas
}
pub fn prod_meta_datas_c2(meta: ProdMetaData) -> ProdMetaDatas {
    meta
}
pub type TermMetaData = BTreeMap<String, ConstVal>;
pub fn term_meta_data_prefer() -> TermMetaData {
    TermMetaData::from([("prefer".into(), ConstVal::Bool(true))])
}
pub fn term_meta_data_finish() -> TermMetaData {
    TermMetaData::from([("finish".into(), ConstVal::Bool(true))])
}
pub fn term_meta_data_no_finish() -> TermMetaData {
    TermMetaData::from([("finish".into(), ConstVal::Bool(false))])
}
pub fn term_meta_data_dynamic() -> TermMetaData {
    TermMetaData::from([("dynamic".into(), ConstVal::Bool(true))])
}
pub fn term_meta_data_priority(prio: IntConst) -> TermMetaData {
    TermMetaData::from([("priority".into(), ConstVal::Int(prio))])
}
pub fn term_meta_data_c6(user: UserMetaData) -> TermMetaData {
    TermMetaData::from([(user.name, user.value)])
}
pub type TermMetaDatas = TermMetaData;
pub fn term_meta_datas_c1(
    mut metas: TermMetaDatas,
    meta: TermMetaData,
) -> TermMetaDatas {
    metas.extend(meta);
    metas
}
pub fn term_meta_datas_c2(meta: TermMetaData) -> TermMetaDatas {
    meta
}
#[derive(Debug, Clone)]
pub struct UserMetaData {
    pub name: Name,
    pub value: ConstVal,
}
pub fn user_meta_data_c1(name: Name, value: ConstVal) -> UserMetaData {
    UserMetaData { name, value }
}
pub type ProdKind = Name;
pub fn prod_kind_c1(name: Name) -> ProdKind {
    name
}
#[derive(Debug, Clone)]
pub enum ConstVal {
    Int(u32),
    Float(f32),
    Bool(bool),
    String(String),
}
pub fn const_val_c1(int_const: IntConst) -> ConstVal {
    ConstVal::Int(int_const)
}
pub fn const_val_c2(float_const: FloatConst) -> ConstVal {
    ConstVal::Float(float_const)
}
pub fn const_val_c3(bool_const: BoolConst) -> ConstVal {
    ConstVal::Bool(bool_const)
}
pub fn const_val_c4(str_const: StrConst) -> ConstVal {
    ConstVal::String(str_const)
}
#[derive(Debug, Clone)]
pub enum Assignment {
    PlainAssignment(PlainAssignment),
    BoolAssignment(BoolAssignment),
    GrammarSymbolRef(GrammarSymbolRef),
}
pub fn assignment_c1(plain_assignment: PlainAssignment) -> Assignment {
    Assignment::PlainAssignment(plain_assignment)
}
pub fn assignment_c2(bool_assignment: BoolAssignment) -> Assignment {
    Assignment::BoolAssignment(bool_assignment)
}
pub fn assignment_c3(grammar_symbol_ref: GrammarSymbolRef) -> Assignment {
    Assignment::GrammarSymbolRef(grammar_symbol_ref)
}
pub type Assignment1 = Vec<Assignment>;
pub type Assignments = Assignment1;
pub fn assignment1_c1(
    mut assignment1: Assignment1,
    assignment: Assignment,
) -> Assignment1 {
    assignment1.push(assignment);
    assignment1
}
pub fn assignment1_c2(assignment: Assignment) -> Assignment1 {
    vec![assignment]
}
#[derive(Debug, Clone)]
pub struct NamedAssignment {
    pub name: Name,
    pub gsymref: GrammarSymbolRef,
}
pub type PlainAssignment = NamedAssignment;
pub fn plain_assignment_c1(name: Name, gsymref: GrammarSymbolRef) -> PlainAssignment {
    PlainAssignment { name, gsymref }
}
pub type BoolAssignment = NamedAssignment;
pub fn bool_assignment_c1(name: Name, gsymref: GrammarSymbolRef) -> BoolAssignment {
    BoolAssignment { name, gsymref }
}
#[derive(Debug, Clone)]
pub struct ProductionGroup(pub GrammarRuleRHS);
pub fn production_group_c1(prod_rule_rhs: GrammarRuleRHS) -> ProductionGroup {
    ProductionGroup(prod_rule_rhs)
}
#[derive(Debug, Clone)]
pub struct GrammarSymbolRef {
    pub gsymbol: Option<GrammarSymbol>,
    pub repetition_op: RepetitionOperatorOpt,
    pub production_group: Option<ProductionGroup>,
}
pub fn grammar_symbol_ref_c1(
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
    repetition_operator: RepetitionOperator,
) -> RepetitionOperatorOpt {
    Some(repetition_operator)
}
pub fn repetition_operator_opt_empty() -> RepetitionOperatorOpt {
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
pub fn repetition_operator_op_zero_or_more() -> RepetitionOperatorOp {
    RepetitionOperatorOp::ZeroOrMore
}
pub fn repetition_operator_op_zero_or_more_greedy() -> RepetitionOperatorOp {
    RepetitionOperatorOp::ZeroOrMoreGreedy
}
pub fn repetition_operator_op_one_or_more() -> RepetitionOperatorOp {
    RepetitionOperatorOp::OneOrMore
}
pub fn repetition_operator_op_one_or_more_greedy() -> RepetitionOperatorOp {
    RepetitionOperatorOp::OneOrMoreGreedy
}
pub fn repetition_operator_op_optional() -> RepetitionOperatorOp {
    RepetitionOperatorOp::Optional
}
pub fn repetition_operator_op_optional_greedy() -> RepetitionOperatorOp {
    RepetitionOperatorOp::OptionalGreedy
}
pub type RepetitionModifiersOpt = Option<RepetitionModifiers>;
pub fn repetition_modifiers_opt_c1(
    repetition_modifiers: RepetitionModifiers,
) -> RepetitionModifiersOpt {
    Some(repetition_modifiers)
}
pub fn repetition_modifiers_opt_empty() -> RepetitionModifiersOpt {
    None
}
pub type RepetitionModifiers = Vec<RepetitionModifier>;
pub fn repetition_modifiers_c1(
    repetition_modifier1: RepetitionModifier1,
) -> RepetitionModifiers {
    repetition_modifier1
}
pub type RepetitionModifier1 = Vec<RepetitionModifier>;
pub fn repetition_modifier1_c1(
    mut repetition_modifier1: RepetitionModifier1,
    repetition_modifier: RepetitionModifier,
) -> RepetitionModifier1 {
    repetition_modifier1.push(repetition_modifier);
    repetition_modifier1
}
pub fn repetition_modifier1_c2(
    repetition_modifier: RepetitionModifier,
) -> RepetitionModifier1 {
    vec![repetition_modifier]
}
pub type RepetitionModifier = String;
pub fn repetition_modifier_c1(name: Name) -> RepetitionModifier {
    name
}
#[derive(Debug, Clone)]
pub enum GrammarSymbol {
    Name(Name),
    StrConst(StrConst),
}
pub fn grammar_symbol_c1(name: Name) -> GrammarSymbol {
    GrammarSymbol::Name(name)
}
pub fn grammar_symbol_c2(str_const: StrConst) -> GrammarSymbol {
    GrammarSymbol::StrConst(str_const)
}
#[derive(Debug, Clone)]
pub enum Recognizer {
    StrConst(StrConst),
    RegexTerm(RegexTerm),
}
pub fn recognizer_c1(str_const: StrConst) -> Recognizer {
    Recognizer::StrConst(str_const)
}
pub fn recognizer_c2(regex_term: RegexTerm) -> Recognizer {
    Recognizer::RegexTerm(regex_term)
}
pub type Layout = String;
pub fn layout_c1(layout_item0: LayoutItem0) -> Layout {
    layout_item0
}
pub type LayoutItem1 = String;
pub fn layout_item1_c1(
    mut layout_item1: LayoutItem1,
    layout_item: LayoutItem,
) -> LayoutItem1 {
    layout_item1.push_str(&layout_item);
    layout_item1
}
pub fn layout_item1_c2(layout_item: LayoutItem) -> LayoutItem1 {
    layout_item
}
pub type LayoutItem0 = LayoutItem1;
pub fn layout_item0_c1(layout_item1: LayoutItem1) -> LayoutItem0 {
    layout_item1
}
pub fn layout_item0_empty() -> LayoutItem0 {
    "".into()
}
pub type LayoutItem = String;
pub fn layout_item_c1(ws: WS) -> LayoutItem {
    ws
}
pub fn layout_item_c2(comment: Comment) -> LayoutItem {
    comment
}
pub type Comment = String;
pub fn comment_c1(corncs: Corncs) -> Comment {
    corncs
}
pub fn comment_c2(comment_line: CommentLine) -> Comment {
    comment_line
}
pub type Corncs = Cornc0;
pub fn corncs_c1(cornc0: Cornc0) -> Corncs {
    cornc0
}
pub type Cornc1 = String;
pub fn cornc1_c1(mut cornc1: Cornc1, cornc: Cornc) -> Cornc1 {
    cornc1.push_str(&cornc);
    cornc1
}
pub fn cornc1_c2(cornc: Cornc) -> Cornc1 {
    cornc
}
pub type Cornc0 = Cornc1;
pub fn cornc0_c1(cornc1: Cornc1) -> Cornc0 {
    cornc1
}
pub fn cornc0_empty() -> Cornc0 {
    "".into()
}
pub type Cornc = String;
pub fn cornc_c1(comment: Comment) -> Cornc {
    comment
}
pub fn cornc_c2(not_comment: NotComment) -> Cornc {
    not_comment
}
pub fn cornc_c3(ws: WS) -> Cornc {
    ws
}
