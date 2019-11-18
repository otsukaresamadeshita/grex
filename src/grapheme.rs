/*
 * Copyright © 2019 Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fmt::{Display, Formatter, Result};
use unicode_segmentation::UnicodeSegmentation;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GraphemeCluster {
    graphemes: Vec<Grapheme>,
}

impl GraphemeCluster {
    pub(crate) fn from(s: &str) -> Self {
        Self {
            graphemes: UnicodeSegmentation::graphemes(s, true).map(|it| Grapheme::from(it)).collect_vec(),
        }
    }

    pub(crate) fn new(grapheme: Grapheme) -> Self {
        Self {
            graphemes: vec![grapheme]
        }
    }

    pub(crate) fn merge(first: &GraphemeCluster, second: &GraphemeCluster) -> Self {
        let mut graphemes = vec![];
        graphemes.extend_from_slice(&first.graphemes);
        graphemes.extend_from_slice(&second.graphemes);
        Self {
            graphemes
        }
    }

    pub(crate) fn graphemes(&self) -> &Vec<Grapheme> {
        &self.graphemes
    }

    pub(crate) fn graphemes_mut(&mut self) -> &mut Vec<Grapheme> {
        &mut self.graphemes
    }

    pub(crate) fn size(&self) -> usize {
        self.graphemes.len()
    }

    pub(crate) fn char_count(&self) -> usize {
        self.graphemes.iter().map(|it| it.char_count()).sum()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.graphemes.is_empty()
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Grapheme {
    value: String,
    min: u32,
    max: u32
}

impl Grapheme {
    pub(crate) fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
            min: 1,
            max: 1
        }
    }

    pub(crate) fn value(&self) -> &String {
        &self.value
    }

    pub(crate) fn char_count(&self) -> usize {
        self.value.chars().count()
    }
}

impl Display for GraphemeCluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unimplemented!()
    }
}

impl Display for Grapheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}
