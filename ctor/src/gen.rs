// This file is generated. Do not edit.

use proc_macro::*;
use proc_macro::TokenTree as T;
use proc_macro::Spacing::*;
use proc_macro::Delimiter::*;
use std::iter::FromIterator;

fn c() -> Span { Span::call_site() }

pub fn ctor() -> TokenStream {
  TokenStream::from_iter([
    T::Ident(Ident::new("macro_rules", c())),
    T::Punct(Punct::new('!', Alone)),
    T::Ident(Ident::new("ctor_impl", c())),
    T::Group(Group::new(Brace, 
      TokenStream::from_iter([
        T::Group(Group::new(Parenthesis, 
          TokenStream::from_iter([
            T::Ident(Ident::new("fn", c())),
            T::Ident(Ident::new("macros", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("macros", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("ident", c())),
            T::Ident(Ident::new("name", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("name", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("ident", c())),
            T::Ident(Ident::new("used", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("used", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("meta", c())),
            T::Punct(Punct::new('$', Alone)),
            T::Group(Group::new(Parenthesis, 
              TokenStream::from_iter([
                T::Ident(Ident::new("extra", c())),
                T::Punct(Punct::new('=', Alone)),
                T::Group(Group::new(Brace, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('$', Alone)),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Punct(Punct::new('$', Alone)),
                        T::Ident(Ident::new("extra", c())),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("item", c())),
                      ])
                    )),
                    T::Punct(Punct::new('+', Alone)),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('?', Alone)),
            T::Ident(Ident::new("item", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Group(Group::new(Brace, 
              TokenStream::from_iter([
                T::Punct(Punct::new('$', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Punct(Punct::new('$', Alone)),
                        T::Ident(Ident::new("meta", c())),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("meta", c())),
                      ])
                    )),
                  ])
                )),
                T::Punct(Punct::new('*', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("vis", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("vis", c())),
                T::Punct(Punct::new('$', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("unsafe", c())),
                  ])
                )),
                T::Punct(Punct::new('?', Alone)),
                T::Ident(Ident::new("fn", c())),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("ident", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("ident", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::new()
                )),
                T::Punct(Punct::new('$', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('-', Joint)),
                    T::Punct(Punct::new('>', Alone)),
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("ret", c())),
                    T::Punct(Punct::new(':', Alone)),
                    T::Ident(Ident::new("ty", c())),
                  ])
                )),
                T::Punct(Punct::new('?', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("block", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("block", c())),
              ])
            )),
          ])
        )),
        T::Punct(Punct::new('=', Joint)),
        T::Punct(Punct::new('>', Alone)),
        T::Group(Group::new(Brace, 
          TokenStream::from_iter([
            T::Punct(Punct::new('$', Alone)),
            T::Group(Group::new(Parenthesis, 
              TokenStream::from_iter([
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("meta", c())),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('*', Alone)),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("allow", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("unused", c())),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("vis", c())),
            T::Ident(Ident::new("unsafe", c())),
            T::Ident(Ident::new("extern", c())),
            T::Literal(Literal::string("C")),
            T::Ident(Ident::new("fn", c())),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("ident", c())),
            T::Group(Group::new(Parenthesis, 
              TokenStream::new()
            )),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("block", c())),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("doc", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("hidden", c())),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("allow", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("unused", c())),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("non_snake_case", c())),
                  ])
                )),
              ])
            )),
            T::Ident(Ident::new("mod", c())),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("name", c())),
            T::Group(Group::new(Brace, 
              TokenStream::from_iter([
                T::Ident(Ident::new("super", c())),
                T::Punct(Punct::new(':', Joint)),
                T::Punct(Punct::new(':', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("macros", c())),
                T::Punct(Punct::new(':', Joint)),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("ctor_raw", c())),
                T::Punct(Punct::new('!', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("used", c())),
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("allow", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("non_upper_case_globals", c())),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("non_snake_case", c())),
                          ])
                        )),
                      ])
                    )),
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("doc", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("hidden", c())),
                          ])
                        )),
                      ])
                    )),
                    T::Ident(Ident::new("static", c())),
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("name", c())),
                    T::Punct(Punct::new(':', Alone)),
                    T::Ident(Ident::new("unsafe", c())),
                    T::Ident(Ident::new("extern", c())),
                    T::Literal(Literal::string("C")),
                    T::Ident(Ident::new("fn", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::new()
                    )),
                    T::Punct(Punct::new('-', Joint)),
                    T::Punct(Punct::new('>', Alone)),
                    T::Ident(Ident::new("usize", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Group(Group::new(Brace, 
                      TokenStream::from_iter([
                        T::Punct(Punct::new('#', Alone)),
                        T::Group(Group::new(Bracket, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("allow", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::from_iter([
                                T::Ident(Ident::new("non_snake_case", c())),
                              ])
                            )),
                          ])
                        )),
                        T::Punct(Punct::new('#', Alone)),
                        T::Group(Group::new(Bracket, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("cfg_attr", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::from_iter([
                                T::Ident(Ident::new("any", c())),
                                T::Group(Group::new(Parenthesis, 
                                  TokenStream::from_iter([
                                    T::Ident(Ident::new("target_os", c())),
                                    T::Punct(Punct::new('=', Alone)),
                                    T::Literal(Literal::string("linux")),
                                    T::Punct(Punct::new(',', Alone)),
                                    T::Ident(Ident::new("target_os", c())),
                                    T::Punct(Punct::new('=', Alone)),
                                    T::Literal(Literal::string("android")),
                                  ])
                                )),
                                T::Punct(Punct::new(',', Alone)),
                                T::Ident(Ident::new("link_section", c())),
                                T::Punct(Punct::new('=', Alone)),
                                T::Literal(Literal::string(".text.startup")),
                              ])
                            )),
                          ])
                        )),
                        T::Ident(Ident::new("unsafe", c())),
                        T::Ident(Ident::new("extern", c())),
                        T::Literal(Literal::string("C")),
                        T::Ident(Ident::new("fn", c())),
                        T::Punct(Punct::new('$', Alone)),
                        T::Ident(Ident::new("name", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::new()
                        )),
                        T::Punct(Punct::new('-', Joint)),
                        T::Punct(Punct::new('>', Alone)),
                        T::Ident(Ident::new("usize", c())),
                        T::Group(Group::new(Brace, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("super", c())),
                            T::Punct(Punct::new(':', Joint)),
                            T::Punct(Punct::new(':', Alone)),
                            T::Punct(Punct::new('$', Alone)),
                            T::Ident(Ident::new("ident", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::new()
                            )),
                            T::Punct(Punct::new(';', Alone)),
                            T::Literal(Literal::usize_unsuffixed(0)),
                          ])
                        )),
                        T::Punct(Punct::new('$', Alone)),
                        T::Ident(Ident::new("name", c())),
                      ])
                    )),
                    T::Punct(Punct::new(';', Alone)),
                  ])
                )),
                T::Punct(Punct::new(';', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('$', Alone)),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Punct(Punct::new('$', Alone)),
                        T::Ident(Ident::new("extra", c())),
                      ])
                    )),
                    T::Punct(Punct::new('+', Alone)),
                  ])
                )),
                T::Punct(Punct::new('?', Alone)),
              ])
            )),
          ])
        )),
        T::Punct(Punct::new(';', Alone)),
        T::Group(Group::new(Parenthesis, 
          TokenStream::from_iter([
            T::Ident(Ident::new("static", c())),
            T::Ident(Ident::new("macros", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("macros", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("ident", c())),
            T::Ident(Ident::new("name", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("name", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("ident", c())),
            T::Ident(Ident::new("used", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("used", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("meta", c())),
            T::Ident(Ident::new("item", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Group(Group::new(Brace, 
              TokenStream::from_iter([
                T::Punct(Punct::new('$', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Punct(Punct::new('$', Alone)),
                        T::Ident(Ident::new("meta", c())),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("meta", c())),
                      ])
                    )),
                  ])
                )),
                T::Punct(Punct::new('*', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("vis", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("vis", c())),
                T::Ident(Ident::new("static", c())),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("ident", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("ident", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("ty", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("ty", c())),
                T::Punct(Punct::new('=', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("expr", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("expr", c())),
                T::Punct(Punct::new(';', Alone)),
              ])
            )),
          ])
        )),
        T::Punct(Punct::new('=', Joint)),
        T::Punct(Punct::new('>', Alone)),
        T::Group(Group::new(Brace, 
          TokenStream::from_iter([
            T::Punct(Punct::new('$', Alone)),
            T::Group(Group::new(Parenthesis, 
              TokenStream::from_iter([
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("meta", c())),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('*', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("vis", c())),
            T::Ident(Ident::new("static", c())),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("ident", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("name", c())),
            T::Punct(Punct::new(':', Joint)),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("Static", c())),
            T::Punct(Punct::new('<', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("ty", c())),
            T::Punct(Punct::new('>', Alone)),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("name", c())),
            T::Punct(Punct::new(':', Joint)),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("Static", c())),
            T::Punct(Punct::new(':', Joint)),
            T::Punct(Punct::new(':', Alone)),
            T::Punct(Punct::new('<', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("ty", c())),
            T::Punct(Punct::new('>', Alone)),
            T::Group(Group::new(Brace, 
              TokenStream::from_iter([
                T::Ident(Ident::new("_storage", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Punct(Punct::new(':', Joint)),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("std", c())),
                T::Punct(Punct::new(':', Joint)),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("cell", c())),
                T::Punct(Punct::new(':', Joint)),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("UnsafeCell", c())),
                T::Punct(Punct::new(':', Joint)),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("new", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("None", c())),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new(';', Alone)),
            T::Ident(Ident::new("impl", c())),
            T::Punct(Punct::new(':', Joint)),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("core", c())),
            T::Punct(Punct::new(':', Joint)),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("ops", c())),
            T::Punct(Punct::new(':', Joint)),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("Deref", c())),
            T::Ident(Ident::new("for", c())),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("name", c())),
            T::Punct(Punct::new(':', Joint)),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("Static", c())),
            T::Punct(Punct::new('<', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("ty", c())),
            T::Punct(Punct::new('>', Alone)),
            T::Group(Group::new(Brace, 
              TokenStream::from_iter([
                T::Ident(Ident::new("type", c())),
                T::Ident(Ident::new("Target", c())),
                T::Punct(Punct::new('=', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("ty", c())),
                T::Punct(Punct::new(';', Alone)),
                T::Ident(Ident::new("fn", c())),
                T::Ident(Ident::new("deref", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('&', Alone)),
                    T::Ident(Ident::new("self", c())),
                  ])
                )),
                T::Punct(Punct::new('-', Joint)),
                T::Punct(Punct::new('>', Alone)),
                T::Punct(Punct::new('&', Alone)),
                T::Punct(Punct::new('\'', Joint)),
                T::Ident(Ident::new("static", c())),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("ty", c())),
                T::Group(Group::new(Brace, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("unsafe", c())),
                    T::Group(Group::new(Brace, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("let", c())),
                        T::Ident(Ident::new("ptr", c())),
                        T::Punct(Punct::new('=', Alone)),
                        T::Ident(Ident::new("self", c())),
                        T::Punct(Punct::new('.', Alone)),
                        T::Ident(Ident::new("_storage", c())),
                        T::Punct(Punct::new('.', Alone)),
                        T::Ident(Ident::new("get", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::new()
                        )),
                        T::Punct(Punct::new(';', Alone)),
                        T::Ident(Ident::new("let", c())),
                        T::Ident(Ident::new("val", c())),
                        T::Punct(Punct::new('=', Alone)),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Punct(Punct::new('&', Alone)),
                            T::Punct(Punct::new('*', Alone)),
                            T::Ident(Ident::new("ptr", c())),
                          ])
                        )),
                        T::Punct(Punct::new('.', Alone)),
                        T::Ident(Ident::new("as_ref", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::new()
                        )),
                        T::Punct(Punct::new('.', Alone)),
                        T::Ident(Ident::new("unwrap", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::new()
                        )),
                        T::Punct(Punct::new(';', Alone)),
                        T::Ident(Ident::new("val", c())),
                      ])
                    )),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("macros", c())),
            T::Punct(Punct::new(':', Joint)),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("ctor_impl", c())),
            T::Punct(Punct::new('!', Alone)),
            T::Group(Group::new(Parenthesis, 
              TokenStream::from_iter([
                T::Ident(Ident::new("fn", c())),
                T::Ident(Ident::new("macros", c())),
                T::Punct(Punct::new('=', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("macros", c())),
                T::Ident(Ident::new("name", c())),
                T::Punct(Punct::new('=', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("name", c())),
                T::Ident(Ident::new("used", c())),
                T::Punct(Punct::new('=', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("used", c())),
                T::Ident(Ident::new("extra", c())),
                T::Punct(Punct::new('=', Alone)),
                T::Group(Group::new(Brace, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("doc", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("hidden", c())),
                          ])
                        )),
                      ])
                    )),
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("derive", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("Default", c())),
                          ])
                        )),
                      ])
                    )),
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("allow", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("non_camel_case_types", c())),
                          ])
                        )),
                      ])
                    )),
                    T::Ident(Ident::new("pub", c())),
                    T::Ident(Ident::new("struct", c())),
                    T::Ident(Ident::new("Static", c())),
                    T::Punct(Punct::new('<', Alone)),
                    T::Ident(Ident::new("T", c())),
                    T::Punct(Punct::new('>', Alone)),
                    T::Group(Group::new(Brace, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("pub", c())),
                        T::Ident(Ident::new("_storage", c())),
                        T::Punct(Punct::new(':', Alone)),
                        T::Punct(Punct::new(':', Joint)),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("std", c())),
                        T::Punct(Punct::new(':', Joint)),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("cell", c())),
                        T::Punct(Punct::new(':', Joint)),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("UnsafeCell", c())),
                        T::Punct(Punct::new('<', Alone)),
                        T::Punct(Punct::new(':', Joint)),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("std", c())),
                        T::Punct(Punct::new(':', Joint)),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("option", c())),
                        T::Punct(Punct::new(':', Joint)),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("Option", c())),
                        T::Punct(Punct::new('<', Alone)),
                        T::Ident(Ident::new("T", c())),
                        T::Punct(Punct::new('>', Joint)),
                        T::Punct(Punct::new('>', Alone)),
                      ])
                    )),
                    T::Ident(Ident::new("unsafe", c())),
                    T::Ident(Ident::new("impl", c())),
                    T::Punct(Punct::new('<', Alone)),
                    T::Ident(Ident::new("T", c())),
                    T::Punct(Punct::new('>', Alone)),
                    T::Ident(Ident::new("std", c())),
                    T::Punct(Punct::new(':', Joint)),
                    T::Punct(Punct::new(':', Alone)),
                    T::Ident(Ident::new("marker", c())),
                    T::Punct(Punct::new(':', Joint)),
                    T::Punct(Punct::new(':', Alone)),
                    T::Ident(Ident::new("Sync", c())),
                    T::Ident(Ident::new("for", c())),
                    T::Ident(Ident::new("Static", c())),
                    T::Punct(Punct::new('<', Alone)),
                    T::Ident(Ident::new("T", c())),
                    T::Punct(Punct::new('>', Alone)),
                    T::Group(Group::new(Brace, 
                      TokenStream::new()
                    )),
                  ])
                )),
                T::Ident(Ident::new("item", c())),
                T::Punct(Punct::new('=', Alone)),
                T::Group(Group::new(Brace, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("allow", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("non_snake_case", c())),
                          ])
                        )),
                      ])
                    )),
                    T::Ident(Ident::new("fn", c())),
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("name", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::new()
                    )),
                    T::Group(Group::new(Brace, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("let", c())),
                        T::Ident(Ident::new("val", c())),
                        T::Punct(Punct::new('=', Alone)),
                        T::Ident(Ident::new("Some", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Punct(Punct::new('$', Alone)),
                            T::Ident(Ident::new("expr", c())),
                          ])
                        )),
                        T::Punct(Punct::new(';', Alone)),
                        T::Ident(Ident::new("unsafe", c())),
                        T::Group(Group::new(Brace, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("let", c())),
                            T::Ident(Ident::new("ptr", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Punct(Punct::new('$', Alone)),
                            T::Ident(Ident::new("ident", c())),
                            T::Punct(Punct::new('.', Alone)),
                            T::Ident(Ident::new("_storage", c())),
                            T::Punct(Punct::new('.', Alone)),
                            T::Ident(Ident::new("get", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::new()
                            )),
                            T::Punct(Punct::new(';', Alone)),
                            T::Punct(Punct::new(':', Joint)),
                            T::Punct(Punct::new(':', Alone)),
                            T::Ident(Ident::new("std", c())),
                            T::Punct(Punct::new(':', Joint)),
                            T::Punct(Punct::new(':', Alone)),
                            T::Ident(Ident::new("ptr", c())),
                            T::Punct(Punct::new(':', Joint)),
                            T::Punct(Punct::new(':', Alone)),
                            T::Ident(Ident::new("write", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::from_iter([
                                T::Ident(Ident::new("ptr", c())),
                                T::Punct(Punct::new(',', Alone)),
                                T::Ident(Ident::new("val", c())),
                              ])
                            )),
                            T::Punct(Punct::new(';', Alone)),
                          ])
                        )),
                      ])
                    )),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new(';', Alone)),
          ])
        )),
        T::Punct(Punct::new(';', Alone)),
        T::Group(Group::new(Parenthesis, 
          TokenStream::from_iter([
            T::Ident(Ident::new("dtor", c())),
            T::Ident(Ident::new("macros", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("macros", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("ident", c())),
            T::Ident(Ident::new("name", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("name", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("ident", c())),
            T::Ident(Ident::new("used", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("used", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("meta", c())),
            T::Ident(Ident::new("item", c())),
            T::Punct(Punct::new('=', Alone)),
            T::Group(Group::new(Brace, 
              TokenStream::from_iter([
                T::Punct(Punct::new('$', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Punct(Punct::new('$', Alone)),
                        T::Ident(Ident::new("meta", c())),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("meta", c())),
                      ])
                    )),
                  ])
                )),
                T::Punct(Punct::new('*', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("vis", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("vis", c())),
                T::Punct(Punct::new('$', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("unsafe", c())),
                  ])
                )),
                T::Punct(Punct::new('?', Alone)),
                T::Ident(Ident::new("fn", c())),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("ident", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("ident", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::new()
                )),
                T::Punct(Punct::new('$', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('-', Joint)),
                    T::Punct(Punct::new('>', Alone)),
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("ret", c())),
                    T::Punct(Punct::new(':', Alone)),
                    T::Ident(Ident::new("ty", c())),
                  ])
                )),
                T::Punct(Punct::new('?', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("block", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("block", c())),
              ])
            )),
          ])
        )),
        T::Punct(Punct::new('=', Joint)),
        T::Punct(Punct::new('>', Alone)),
        T::Group(Group::new(Brace, 
          TokenStream::from_iter([
            T::Punct(Punct::new('$', Alone)),
            T::Group(Group::new(Parenthesis, 
              TokenStream::from_iter([
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("meta", c())),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('*', Alone)),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("allow", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("unused", c())),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("vis", c())),
            T::Ident(Ident::new("unsafe", c())),
            T::Ident(Ident::new("extern", c())),
            T::Literal(Literal::string("C")),
            T::Ident(Ident::new("fn", c())),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("ident", c())),
            T::Group(Group::new(Parenthesis, 
              TokenStream::new()
            )),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("block", c())),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("doc", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("hidden", c())),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("allow", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("unused", c())),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("non_snake_case", c())),
                  ])
                )),
              ])
            )),
            T::Ident(Ident::new("mod", c())),
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("name", c())),
            T::Group(Group::new(Brace, 
              TokenStream::from_iter([
                T::Ident(Ident::new("super", c())),
                T::Punct(Punct::new(':', Joint)),
                T::Punct(Punct::new(':', Alone)),
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("macros", c())),
                T::Punct(Punct::new(':', Joint)),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("ctor_raw", c())),
                T::Punct(Punct::new('!', Alone)),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("used", c())),
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("allow", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("non_upper_case_globals", c())),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("non_snake_case", c())),
                          ])
                        )),
                      ])
                    )),
                    T::Punct(Punct::new('#', Alone)),
                    T::Group(Group::new(Bracket, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("doc", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("hidden", c())),
                          ])
                        )),
                      ])
                    )),
                    T::Ident(Ident::new("static", c())),
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("name", c())),
                    T::Punct(Punct::new(':', Alone)),
                    T::Ident(Ident::new("unsafe", c())),
                    T::Ident(Ident::new("extern", c())),
                    T::Literal(Literal::string("C")),
                    T::Ident(Ident::new("fn", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::new()
                    )),
                    T::Punct(Punct::new('-', Joint)),
                    T::Punct(Punct::new('>', Alone)),
                    T::Ident(Ident::new("usize", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Group(Group::new(Brace, 
                      TokenStream::from_iter([
                        T::Punct(Punct::new('#', Alone)),
                        T::Group(Group::new(Bracket, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("allow", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::from_iter([
                                T::Ident(Ident::new("non_snake_case", c())),
                              ])
                            )),
                          ])
                        )),
                        T::Punct(Punct::new('#', Alone)),
                        T::Group(Group::new(Bracket, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("cfg_attr", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::from_iter([
                                T::Ident(Ident::new("any", c())),
                                T::Group(Group::new(Parenthesis, 
                                  TokenStream::from_iter([
                                    T::Ident(Ident::new("target_os", c())),
                                    T::Punct(Punct::new('=', Alone)),
                                    T::Literal(Literal::string("linux")),
                                    T::Punct(Punct::new(',', Alone)),
                                    T::Ident(Ident::new("target_os", c())),
                                    T::Punct(Punct::new('=', Alone)),
                                    T::Literal(Literal::string("android")),
                                  ])
                                )),
                                T::Punct(Punct::new(',', Alone)),
                                T::Ident(Ident::new("link_section", c())),
                                T::Punct(Punct::new('=', Alone)),
                                T::Literal(Literal::string(".text.startup")),
                              ])
                            )),
                          ])
                        )),
                        T::Ident(Ident::new("unsafe", c())),
                        T::Ident(Ident::new("extern", c())),
                        T::Literal(Literal::string("C")),
                        T::Ident(Ident::new("fn", c())),
                        T::Punct(Punct::new('$', Alone)),
                        T::Ident(Ident::new("name", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::new()
                        )),
                        T::Punct(Punct::new('-', Joint)),
                        T::Punct(Punct::new('>', Alone)),
                        T::Ident(Ident::new("usize", c())),
                        T::Group(Group::new(Brace, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("do_atexit", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::from_iter([
                                T::Ident(Ident::new("__dtor", c())),
                              ])
                            )),
                            T::Punct(Punct::new(';', Alone)),
                            T::Literal(Literal::usize_unsuffixed(0)),
                          ])
                        )),
                        T::Punct(Punct::new('$', Alone)),
                        T::Ident(Ident::new("name", c())),
                      ])
                    )),
                    T::Punct(Punct::new(';', Alone)),
                  ])
                )),
                T::Punct(Punct::new(';', Alone)),
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("cfg", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("not", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("target_vendor", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("apple")),
                          ])
                        )),
                      ])
                    )),
                  ])
                )),
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("cfg_attr", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("any", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("linux")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("android")),
                          ])
                        )),
                        T::Punct(Punct::new(',', Alone)),
                        T::Ident(Ident::new("link_section", c())),
                        T::Punct(Punct::new('=', Alone)),
                        T::Literal(Literal::string(".text.exit")),
                      ])
                    )),
                  ])
                )),
                T::Ident(Ident::new("unsafe", c())),
                T::Ident(Ident::new("extern", c())),
                T::Literal(Literal::string("C")),
                T::Ident(Ident::new("fn", c())),
                T::Ident(Ident::new("__dtor", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::new()
                )),
                T::Group(Group::new(Brace, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("super", c())),
                    T::Punct(Punct::new(':', Joint)),
                    T::Punct(Punct::new(':', Alone)),
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("ident", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::new()
                    )),
                  ])
                )),
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("cfg", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("target_vendor", c())),
                        T::Punct(Punct::new('=', Alone)),
                        T::Literal(Literal::string("apple")),
                      ])
                    )),
                  ])
                )),
                T::Ident(Ident::new("unsafe", c())),
                T::Ident(Ident::new("extern", c())),
                T::Literal(Literal::string("C")),
                T::Ident(Ident::new("fn", c())),
                T::Ident(Ident::new("__dtor", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("_", c())),
                    T::Punct(Punct::new(':', Alone)),
                    T::Punct(Punct::new('*', Alone)),
                    T::Ident(Ident::new("const", c())),
                    T::Ident(Ident::new("u8", c())),
                  ])
                )),
                T::Group(Group::new(Brace, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("super", c())),
                    T::Punct(Punct::new(':', Joint)),
                    T::Punct(Punct::new(':', Alone)),
                    T::Punct(Punct::new('$', Alone)),
                    T::Ident(Ident::new("ident", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::new()
                    )),
                  ])
                )),
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("cfg", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("not", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("target_vendor", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("apple")),
                          ])
                        )),
                      ])
                    )),
                  ])
                )),
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("inline", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("always", c())),
                      ])
                    )),
                  ])
                )),
                T::Ident(Ident::new("pub", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("super", c())),
                  ])
                )),
                T::Ident(Ident::new("unsafe", c())),
                T::Ident(Ident::new("fn", c())),
                T::Ident(Ident::new("do_atexit", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("cb", c())),
                    T::Punct(Punct::new(':', Alone)),
                    T::Ident(Ident::new("unsafe", c())),
                    T::Ident(Ident::new("extern", c())),
                    T::Ident(Ident::new("fn", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::new()
                    )),
                  ])
                )),
                T::Group(Group::new(Brace, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("extern", c())),
                    T::Literal(Literal::string("C")),
                    T::Group(Group::new(Brace, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("fn", c())),
                        T::Ident(Ident::new("atexit", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("cb", c())),
                            T::Punct(Punct::new(':', Alone)),
                            T::Ident(Ident::new("unsafe", c())),
                            T::Ident(Ident::new("extern", c())),
                            T::Ident(Ident::new("fn", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::new()
                            )),
                          ])
                        )),
                        T::Punct(Punct::new(';', Alone)),
                      ])
                    )),
                    T::Ident(Ident::new("atexit", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("cb", c())),
                      ])
                    )),
                    T::Punct(Punct::new(';', Alone)),
                  ])
                )),
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("cfg", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("target_vendor", c())),
                        T::Punct(Punct::new('=', Alone)),
                        T::Literal(Literal::string("apple")),
                      ])
                    )),
                  ])
                )),
                T::Punct(Punct::new('#', Alone)),
                T::Group(Group::new(Bracket, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("inline", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("always", c())),
                      ])
                    )),
                  ])
                )),
                T::Ident(Ident::new("pub", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("super", c())),
                  ])
                )),
                T::Ident(Ident::new("unsafe", c())),
                T::Ident(Ident::new("fn", c())),
                T::Ident(Ident::new("do_atexit", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("cb", c())),
                    T::Punct(Punct::new(':', Alone)),
                    T::Ident(Ident::new("unsafe", c())),
                    T::Ident(Ident::new("extern", c())),
                    T::Ident(Ident::new("fn", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("_", c())),
                        T::Punct(Punct::new(':', Alone)),
                        T::Punct(Punct::new('*', Alone)),
                        T::Ident(Ident::new("const", c())),
                        T::Ident(Ident::new("u8", c())),
                      ])
                    )),
                  ])
                )),
                T::Group(Group::new(Brace, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("extern", c())),
                    T::Literal(Literal::string("C")),
                    T::Group(Group::new(Brace, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("static", c())),
                        T::Ident(Ident::new("__dso_handle", c())),
                        T::Punct(Punct::new(':', Alone)),
                        T::Punct(Punct::new('*', Alone)),
                        T::Ident(Ident::new("const", c())),
                        T::Ident(Ident::new("u8", c())),
                        T::Punct(Punct::new(';', Alone)),
                        T::Ident(Ident::new("fn", c())),
                        T::Ident(Ident::new("__cxa_atexit", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("cb", c())),
                            T::Punct(Punct::new(':', Alone)),
                            T::Ident(Ident::new("unsafe", c())),
                            T::Ident(Ident::new("extern", c())),
                            T::Ident(Ident::new("fn", c())),
                            T::Group(Group::new(Parenthesis, 
                              TokenStream::from_iter([
                                T::Ident(Ident::new("_", c())),
                                T::Punct(Punct::new(':', Alone)),
                                T::Punct(Punct::new('*', Alone)),
                                T::Ident(Ident::new("const", c())),
                                T::Ident(Ident::new("u8", c())),
                              ])
                            )),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("arg", c())),
                            T::Punct(Punct::new(':', Alone)),
                            T::Punct(Punct::new('*', Alone)),
                            T::Ident(Ident::new("const", c())),
                            T::Ident(Ident::new("u8", c())),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("dso_handle", c())),
                            T::Punct(Punct::new(':', Alone)),
                            T::Punct(Punct::new('*', Alone)),
                            T::Ident(Ident::new("const", c())),
                            T::Ident(Ident::new("u8", c())),
                          ])
                        )),
                        T::Punct(Punct::new(';', Alone)),
                      ])
                    )),
                    T::Ident(Ident::new("__cxa_atexit", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("cb", c())),
                        T::Punct(Punct::new(',', Alone)),
                        T::Ident(Ident::new("core", c())),
                        T::Punct(Punct::new(':', Joint)),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("ptr", c())),
                        T::Punct(Punct::new(':', Joint)),
                        T::Punct(Punct::new(':', Alone)),
                        T::Ident(Ident::new("null", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::new()
                        )),
                        T::Punct(Punct::new(',', Alone)),
                        T::Ident(Ident::new("__dso_handle", c())),
                      ])
                    )),
                    T::Punct(Punct::new(';', Alone)),
                  ])
                )),
              ])
            )),
          ])
        )),
        T::Punct(Punct::new(';', Alone)),
      ])
    )),
    T::Ident(Ident::new("pub", c())),
    T::Group(Group::new(Parenthesis, 
      TokenStream::from_iter([
        T::Ident(Ident::new("crate", c())),
      ])
    )),
    T::Ident(Ident::new("use", c())),
    T::Ident(Ident::new("ctor_impl", c())),
    T::Punct(Punct::new(';', Alone)),
  ])
}

pub fn ctor_raw() -> TokenStream {
  TokenStream::from_iter([
    T::Ident(Ident::new("macro_rules", c())),
    T::Punct(Punct::new('!', Alone)),
    T::Ident(Ident::new("ctor_raw", c())),
    T::Group(Group::new(Brace, 
      TokenStream::from_iter([
        T::Group(Group::new(Parenthesis, 
          TokenStream::from_iter([
            T::Punct(Punct::new('$', Alone)),
            T::Ident(Ident::new("used", c())),
            T::Punct(Punct::new(':', Alone)),
            T::Ident(Ident::new("meta", c())),
            T::Punct(Punct::new('$', Alone)),
            T::Group(Group::new(Parenthesis, 
              TokenStream::from_iter([
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("block", c())),
                T::Punct(Punct::new(':', Alone)),
                T::Ident(Ident::new("tt", c())),
              ])
            )),
            T::Punct(Punct::new('+', Alone)),
          ])
        )),
        T::Punct(Punct::new('=', Joint)),
        T::Punct(Punct::new('>', Alone)),
        T::Group(Group::new(Brace, 
          TokenStream::from_iter([
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("not", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("any", c())),
                        T::Group(Group::new(Parenthesis, 
                          TokenStream::from_iter([
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("linux")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("android")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("freebsd")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("netbsd")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("openbsd")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("dragonfly")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("illumos")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("target_os", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("haiku")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("target_vendor", c())),
                            T::Punct(Punct::new('=', Alone)),
                            T::Literal(Literal::string("apple")),
                            T::Punct(Punct::new(',', Alone)),
                            T::Ident(Ident::new("windows", c())),
                          ])
                        )),
                      ])
                    )),
                  ])
                )),
              ])
            )),
            T::Ident(Ident::new("compile_error", c())),
            T::Punct(Punct::new('!', Alone)),
            T::Group(Group::new(Parenthesis, 
              TokenStream::from_iter([
                T::Literal(Literal::string("#[ctor] is not supported on the current target")),
              ])
            )),
            T::Punct(Punct::new(';', Alone)),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("used", c())),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg_attr", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("any", c())),
                    T::Group(Group::new(Parenthesis, 
                      TokenStream::from_iter([
                        T::Ident(Ident::new("target_os", c())),
                        T::Punct(Punct::new('=', Alone)),
                        T::Literal(Literal::string("linux")),
                        T::Punct(Punct::new(',', Alone)),
                        T::Ident(Ident::new("target_os", c())),
                        T::Punct(Punct::new('=', Alone)),
                        T::Literal(Literal::string("android")),
                      ])
                    )),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("link_section", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string(".init_array")),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg_attr", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("target_os", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string("freebsd")),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("link_section", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string(".init_array")),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg_attr", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("target_os", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string("netbsd")),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("link_section", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string(".init_array")),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg_attr", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("target_os", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string("openbsd")),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("link_section", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string(".init_array")),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg_attr", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("target_os", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string("dragonfly")),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("link_section", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string(".init_array")),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg_attr", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("target_os", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string("illumos")),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("link_section", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string(".init_array")),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg_attr", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("target_os", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string("haiku")),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("link_section", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string(".init_array")),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg_attr", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("target_vendor", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string("apple")),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("link_section", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string("__DATA,__mod_init_func")),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('#', Alone)),
            T::Group(Group::new(Bracket, 
              TokenStream::from_iter([
                T::Ident(Ident::new("cfg_attr", c())),
                T::Group(Group::new(Parenthesis, 
                  TokenStream::from_iter([
                    T::Ident(Ident::new("windows", c())),
                    T::Punct(Punct::new(',', Alone)),
                    T::Ident(Ident::new("link_section", c())),
                    T::Punct(Punct::new('=', Alone)),
                    T::Literal(Literal::string(".CRT$XCU")),
                  ])
                )),
              ])
            )),
            T::Punct(Punct::new('$', Alone)),
            T::Group(Group::new(Parenthesis, 
              TokenStream::from_iter([
                T::Punct(Punct::new('$', Alone)),
                T::Ident(Ident::new("block", c())),
              ])
            )),
            T::Punct(Punct::new('+', Alone)),
          ])
        )),
      ])
    )),
    T::Ident(Ident::new("pub", c())),
    T::Group(Group::new(Parenthesis, 
      TokenStream::from_iter([
        T::Ident(Ident::new("crate", c())),
      ])
    )),
    T::Ident(Ident::new("use", c())),
    T::Ident(Ident::new("ctor_raw", c())),
    T::Punct(Punct::new(';', Alone)),
  ])
}

