// a Shiny test framework
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![crate_name = "shiny"]
#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate rustc;
extern crate syntax;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::abi;
use syntax::ptr::P;
use syntax::ast;
use syntax::codemap::DUMMY_SP;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax::parse::tts_to_parser;
use syntax::util::small_vector::SmallVector;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("describe", macro_describe);
}

fn is_item(stmt: &ast::Stmt) -> bool {
    match stmt.node {
        ast::StmtKind::Decl(ref decl, _) => {
            match decl.node {
                ast::DeclKind::Item(_) => true,
                _ => false,
            }
        },
        _ => false,
    }
}

pub fn macro_describe(cx: &mut ExtCtxt, _: Span, tts: &[ast::TokenTree]) -> Box<MacResult+'static> {
    let sess = cx.parse_sess();
    let ttsvec = tts.iter().map(|x| (*x).clone()).collect();

    let mut parser = tts_to_parser(sess, ttsvec, cx.cfg());

    let mut before_block = None;
    let mut test_blocks = vec!();

    loop {
        if parser.token == token::Eof {
            break;
        }

        // TODO: Handle the failure case
        let ident = parser.parse_ident().unwrap();
        match format!("{}", ident).as_str() {
            "before_each" => {
                if before_block.is_some() {
                    panic!("only one before_each block is allowed");
                }
                before_block = Some(parser.parse_block());
            },
            "it" => {
                // TODO: Handle the failure case
                let (name, _) = parser.parse_str().unwrap();
                let block = parser.parse_block();
                test_blocks.push((name.to_string(), block));
            }
            other => {
                let span = parser.span;
                parser.span_fatal(span, format!("expected one of: {} but found `{}`",
                    "`before_each`, `it`",
                    other).as_str());
            },
        }
    }

    let mut funcs = vec!();

    for (name, block) in test_blocks.into_iter() {
        let body = match before_block {
            None => block.unwrap().clone(),
            Some(ref before) => {
                match before {
                    &Ok(ref ubefore) => {
                        // TODO: Handle the failure case
                        let ublock = block.unwrap();
                        let items: Vec<ast::Stmt> = ubefore
                            .stmts.clone().into_iter().filter(is_item).collect();
                        let mut stmts = vec!();
                        stmts.extend(items);
                        stmts.extend(ubefore.stmts.clone());
                        stmts.extend(ublock.stmts.clone());
                        P(ast::Block {
                            stmts: stmts,
                            ..(*ublock).clone()
                        })
                    }
                    &Err(_) => panic!("boom")
                }
            }
        };

        let attr_test = cx.attribute(DUMMY_SP,
            cx.meta_word(DUMMY_SP, token::InternedString::new("test")));

        let func = P(ast::Item {
            ident: cx.ident_of(name.replace(" ", "_").as_str()),
            attrs: vec!(attr_test),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemKind::Fn(
                cx.fn_decl(Vec::new(), cx.ty(DUMMY_SP, ast::TyKind::Tup(Vec::new()))),
                ast::Unsafety::Normal,
                ast::Constness::NotConst,
                abi::Abi::Rust,
                ast::Generics::default(),
                body),
            vis: ast::Visibility::Inherited,
            span: DUMMY_SP,
        });
        funcs.push(func);
    }

    MacItems::new(funcs)
}

pub struct MacItems {
    items: Vec<P<ast::Item>>
}

impl MacItems {
    pub fn new(items: Vec<P<ast::Item>>) -> Box<MacResult+'static> {
        Box::new(MacItems { items: items })
    }
}

impl MacResult for MacItems {
    fn make_items(self: Box<MacItems>) -> Option<SmallVector<P<ast::Item>>> {
        Some(SmallVector::many(self.items.clone()))
    }
}
