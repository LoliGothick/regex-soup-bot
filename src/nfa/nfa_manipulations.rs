/*
 * ISC License
 *
 * Copyright (c) 2021 Mitama Lab
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 */

use automata::{nfa::Nfa, Alphabet};

/// The NFA that only accepts the empty word.
pub fn epsilon<A: Alphabet>() -> Nfa<A> {
    todo!()
}

/// An NFA that only accepts the single-letter word containing the given alphabet.
pub fn literal<A: Alphabet>(_alphabet: A) -> Nfa<A> {
    todo!()
}

#[allow(dead_code)]
/// An NFA that recognizes `L(left)^L(right)` where `^` is the concatenation of languages.
pub fn concat<A: Alphabet>(_left: &Nfa<A>, _right: &Nfa<A>) -> Nfa<A> {
    todo!()
}

/// An NFA that accepts concatenations of words each of which is recognized by NFAs in the slice.
pub fn concat_all<A: Alphabet>(_nfas: &[Nfa<A>]) -> Nfa<A> {
    todo!()
}

/// An NFA that recognizes union of languages of NFAs in the given slice.
pub fn union_all<A: Alphabet>(_nfas: &[Nfa<A>]) -> Nfa<A> {
    todo!()
}

/// An NFA that recognizes zero or more repetition of words recognized by the given NFA.
pub fn star<A: Alphabet>(_nfa: &Nfa<A>) -> Nfa<A> {
    todo!()
}
