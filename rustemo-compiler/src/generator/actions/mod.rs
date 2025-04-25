//! Parser/generator for actions file
//!
//! Provides default semantics actions implementation but allow for manual
//! changes.

use std::{collections::BTreeSet, path::PathBuf};

use quote::format_ident;
use syn::{self, parse_quote};

use crate::{
    error::Result,
    grammar::{types::to_snake_case, NonTerminal, Terminal},
    settings::{LexerType, Settings},
    Error,
};

use super::ParserGenerator;

mod production;

pub(crate) trait ActionsGenerator {
    fn terminal_type(&self, terminal: &Terminal, settings: &Settings) -> syn::Item {
        let type_name = format_ident!("{}", terminal.name);
        if settings.builder_loc_info {
            parse_quote! {
                pub type #type_name = ValLoc<String>;
            }
        } else {
            parse_quote! {
                pub type #type_name = String;
            }
        }
    }
    fn terminal_action(&self, terminal: &Terminal, settings: &Settings) -> syn::Item {
        let type_name = format_ident!("{}", terminal.name);
        let action_name = format_ident!("{}", to_snake_case(&terminal.name));
        let body: syn::Expr = if settings.builder_loc_info {
            parse_quote! { #type_name::new(token.value.into(), Some(_ctx.location())) }
        } else {
            parse_quote! { token.value.into() }
        };
        parse_quote! {
            pub fn #action_name(_ctx: &Ctx, token: Token) -> #type_name {
                #body
            }
        }
    }

    /// Create Rust types for the given non-terminal.
    fn nonterminal_types(&self, nonterminal: &NonTerminal, settings: &Settings) -> Vec<syn::Item>;

    /// Creates an action function for each production of the given non-terminal.
    fn nonterminal_actions(
        &self,
        nonterminal: &NonTerminal,
        settings: &Settings,
    ) -> Vec<(String, syn::Item)>;
}

pub(super) fn generate_parser_actions(generator: &ParserGenerator) -> Result<()> {
    let parser_mod = PathBuf::from(&generator.file_name)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let mut file_name = String::from(&generator.file_name);
    file_name.push_str("_actions.rs");
    let action_file = generator.out_dir_actions.join(file_name);

    let mut ast = if action_file.exists() && !generator.settings.force {
        log!("Parsing action file with Syn: {:?}", action_file);
        syn::parse_file(&std::fs::read_to_string(&action_file)?)?
    } else {
        // Create new empty file with common uses statements.
        log!("Creating: {:?}", action_file);
        let lexer_mod = format_ident!("{parser_mod}_lexer");
        let parser_mod = format_ident!("{}", parser_mod);
        let input_type: syn::Stmt = match generator.settings.lexer_type {
            LexerType::Default => parse_quote! {
                pub type Input = str;
            },
            LexerType::Custom => parse_quote! {
                use super::#lexer_mod::Input;
            },
        };
        let mut base_use: Vec<syn::Stmt> = vec![];
        if generator.settings.builder_loc_info {
            base_use.push(parse_quote! {use rustemo::{ValLoc, Context as C};})
        };
        base_use.push(parse_quote! {use rustemo::Token as RustemoToken;});
        base_use.push(parse_quote! {use super::#parser_mod::{TokenKind, Context};});
        parse_quote! {
            /// This file is maintained by rustemo but can be modified manually.
            /// All manual changes will be preserved except non-doc comments.
            #(#base_use)*
            #input_type
            pub type Ctx<'i> = Context<'i, Input>;
            #[allow(dead_code)]
            pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
        }
    };

    // Collect function and type names
    let mut type_names = BTreeSet::new();
    let mut action_names = BTreeSet::new();
    for item in &ast.items {
        match item {
            // Used for grammar rules of the form:
            // NT: First | Second | Third;
            // TODO: Are non-terminals allowed in the RHS?
            syn::Item::Enum(e) => {
                let type_name = e.ident.to_string();
                log!("Found enum type '{}'", type_name);
                type_names.insert(type_name);
            }
            syn::Item::Struct(e) => {
                let type_name = e.ident.to_string();
                log!("Found struct type '{}'", type_name);
                type_names.insert(type_name);
            }
            // Used for actions
            syn::Item::Fn(f) => {
                let type_name = f.sig.ident.to_string();
                log!("Found action function '{}'", type_name);
                action_names.insert(type_name);
            }
            syn::Item::Type(t) => {
                let type_name = t.ident.to_string();
                log!("Found type '{}'", type_name);
                type_names.insert(type_name);
            }
            // We don't need to do anything for other source items
            _ => (),
        };
    }

    let actions_generator: Box<dyn ActionsGenerator> = production::ProductionActionsGenerator::new(
        generator.grammar,
        generator.types.as_ref().unwrap(),
    );

    // Generate types and actions for terminals
    generator
        .grammar
        .terminals
        .iter()
        .filter(|t| t.has_content && t.reachable.get())
        .for_each(|terminal| {
            // Add terminal types
            let type_name = &terminal.name;
            if !type_names.contains(type_name) {
                log!("Create type for terminal '{type_name}'.");
                ast.items
                    .push(actions_generator.terminal_type(terminal, generator.settings));
            }
            // Add terminal actions
            let action_name = to_snake_case(&terminal.name);
            if !action_names.contains(&action_name) {
                log!("Create action function for terminal '{type_name}'.");
                ast.items
                    .push(actions_generator.terminal_action(terminal, generator.settings))
            }
        });

    // Generate types and actions for non-terminals
    generator
        .grammar
        .nonterminals()
        .iter()
        .filter(|nt| nt.reachable.get())
        .for_each(|nonterminal| {
            // Add non-terminal type
            if !type_names.contains(&nonterminal.name) {
                log!("Creating types for non-terminal '{}'.", nonterminal.name);
                for ty in actions_generator.nonterminal_types(nonterminal, generator.settings) {
                    ast.items.push(ty);
                }
            }

            // Add non-terminal actions
            for (action_name, action) in
                actions_generator.nonterminal_actions(nonterminal, generator.settings)
            {
                if !action_names.contains(&action_name) {
                    log!("Creating action '{action_name}'.");
                    ast.items.push(action);
                }
            }
        });

    println!("Writing actions file {action_file:?}");
    std::fs::create_dir_all(&generator.out_dir_actions).map_err(|e| {
        Error::Error(format!(
            "Cannot create directories for path '{:?}': {e:?}.",
            generator.out_dir_actions
        ))
    })?;
    std::fs::write(action_file, prettyplease::unparse(&ast))?;

    Ok(())
}
