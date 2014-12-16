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
#![feature(plugin_registrar)]

extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;
use syntax::abi;
use syntax::ptr::P;
use syntax::ast;
use syntax::ast_util::empty_generics;
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

        let ident = parser.parse_ident();
        match ident.as_str() {
            "before_each" => {
                if before_block.is_some() {
                    panic!("only one before_each block is allowed");
                }
                before_block = Some(parser.parse_block());
            },
            "it" => {
                let (name, _) = parser.parse_str();
                let block = parser.parse_block();
                test_blocks.push((name.get().to_string(), block));
            }
            other => {
                let span = parser.span;
                parser.span_fatal(span, format!("expected one of: {} but found `{}`",
                    "`before_each`, `it`",
                    other).as_slice());
            },
        }
    }

    let mut funcs = vec!();

    for (name, block) in test_blocks.into_iter() {
        let body = match before_block {
            None => block.clone(),
            Some(ref before) => {
                P(ast::Block {
                    view_items: before.view_items.clone() + block.view_items.as_slice(),
                    stmts: before.stmts.clone() + block.stmts.as_slice(),

                    ..block.deref().clone()
                })
            }
        };

        let attr_test = cx.attribute(DUMMY_SP,
            cx.meta_word(DUMMY_SP, token::InternedString::new("test")));

        let func = P(ast::Item {
            ident: cx.ident_of(name.replace(" ", "_").as_slice()),
            attrs: vec!(attr_test),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemFn(
                cx.fn_decl(Vec::new(), cx.ty(DUMMY_SP, ast::Ty_::TyTup(Vec::new()))),
                ast::Unsafety::Normal,
                abi::Rust,
                empty_generics(),
                body),
            vis: ast::Inherited,
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
        box MacItems { items: items } as Box<MacResult>
    }
}

impl MacResult for MacItems {
    fn make_items(self: Box<MacItems>) -> Option<SmallVector<P<ast::Item>>> {
        Some(SmallVector::many(self.items.clone()))
    }
}
