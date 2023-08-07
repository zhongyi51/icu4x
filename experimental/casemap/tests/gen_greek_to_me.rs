// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use databake::Bake;
use icu_casemap::greek_to_me::{self, GreekPrecomposedLetterData};
use icu_casemap::CaseMapper;
use icu_codepointtrie_builder::{CodePointTrieBuilder, CodePointTrieBuilderData};
use icu_collections::codepointtrie::{CodePointTrie, TrieType};
use icu_normalizer::DecomposingNormalizer;
use icu_properties::{maps, GeneralCategoryGroup, Script};
use std::fs;
use std::path::Path;

fn main() {
    let decomposer = DecomposingNormalizer::new_nfd();
    let script = maps::script();
    let gc = maps::general_category();
    let cm = CaseMapper::new();

    // Create a vector for all codepoints (including surrogates); as this is the format needed by CodePointTrieBuilder
    // push to it in a loop
    let size = u32::from(char::MAX);
    let mut vec = Vec::with_capacity(size as usize);
    for ch in 0..=size {
        let mut data = GreekPrecomposedLetterData::default();
        if let Ok(ch) = char::try_from(ch) {
            // Check for [:Grek:] and [:L:]
            if script.get(ch) != Script::Greek || !GeneralCategoryGroup::Letter.contains(gc.get(ch))
            {
                vec.push(data);
                continue;
            }

            let nfd = decomposer.normalize_utf8(ch.encode_utf8(&mut [0; 4]).as_bytes());

            for nfd_ch in nfd.chars() {
                match nfd_ch {
                    // accented: [:toNFD=/[\u0300\u0301\u0342\u0302\u0303\u0311]/:]&[:Grek:]&[:L:] (from the JSPs: toNFD is an extension).
                    greek_to_me::diacritics!(ACCENTS) => {
                        data.accented = true;
                    }
                    // dialytika: [:toNFD=/[\u0308]/:]&[:Grek:]&[:L:] (from the JSPs: toNFD is an extension).
                    greek_to_me::diacritics!(DIALYTIKA) => {
                        data.dialytika = true;
                    }
                    // precomposed_ypogegrammeni [:toNFD=/[\u0345]/:]&[:Grek:]&[:L:] (from the JSPs: toNFD is an extension).
                    greek_to_me::diacritics!(YPOGEGRAMMENI) => {
                        data.ypogegrammeni = true;
                    }
                    // Ignore all small letters
                    '\u{1D00}'..='\u{1DBF}' | '\u{AB65}' => (),
                    // caps: [[:Grek:]&[:L:]-[\u1D00-\u1DBF\uAB65]] . NFD, remove non-letters, uppercase
                    letter if GeneralCategoryGroup::Letter.contains(gc.get(letter)) => {
                        if data.uppercase.is_some() {
                            panic!("Found multiple letters within decomposition of {ch}");
                        }
                        let uppercased = cm.uppercase_to_string(
                            letter.encode_utf8(&mut [0; 4]),
                            &Default::default(),
                        );
                        let mut iter = uppercased.chars();
                        let uppercased = iter.next().unwrap();
                        assert!(
                            iter.next().is_none(),
                            "{letter} Should uppercase to a single letter char, instead uppercased to {uppercased:?}"
                        );
                        data.uppercase = Some(uppercased);
                    }
                    _ => (),
                }
            }
        }

        vec.push(data)
    }

    let trie: CodePointTrie<GreekPrecomposedLetterData> = CodePointTrieBuilder {
        data: CodePointTrieBuilderData::ValuesByCodePoint(&vec),
        default_value: Default::default(),
        error_value: Default::default(),
        trie_type: TrieType::Small,
    }
    .build();

    let output = format!(
        r#"// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This file is generated by running `cargo test --test gen_greek_to_me --features compiled_data,datagen
//
// Do not edit manually

use super::GreekPrecomposedLetterData;
use icu_collections::codepointtrie::CodePointTrie;

#[rustfmt::skip]
pub(crate) const GREEK_DATA_TRIE: CodePointTrie<'static, GreekPrecomposedLetterData> = {};
"#,
        trie.bake(&Default::default())
    );

    let local = Path::new(std::env!("CARGO_MANIFEST_DIR")).join("src/greek_to_me/data.rs");

    let local = fs::read_to_string(local).expect("src/greek_to_me/data.rs should be a UTF-8 file");

    // We cannot just check if the two are unequal because on Windows core.autocrlf
    // may have messed with the line endings on the file, or it may have not (either
    // due to a changed setting, or due to the code being in a cargo cache/vendor. We also
    // cannot fix this by passing `--config newline_style=unix` to rustfmt. We must
    // perform a `\r`-agnostic comparison.
    //
    // (technically this should only catch `\r\n` and not just `\r` but for a golden
    // test on rustfmt output it does not matter)
    if local
        .trim()
        .chars()
        .filter(|&x| x != '\r')
        .ne(output.trim().chars().filter(|&x| x != '\r'))
    {
        println!(
            r#"Please copy the following file to src/greek_to_me/data.rs:
========================================================
{output}
========================================================"#
        );

        panic!("Found mismatch between generated Greek specialcasing data and checked-in data. Please check in the updated file shown above.");
    }
}
