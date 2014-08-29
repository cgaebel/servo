/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::hash::Hash;
use std::hash::sip::hash;

#[deriving(Copy)]
pub struct TinyBloomFilter {
    bits: u32, // TODO(cgaebel) experiment with 64 bits?
}

impl TinyBloomFilter {
    pub fn new() -> TinyBloomFilter {
        TinyBloomFilter {
            bits: 0u32,
        }
    }

    pub fn probe_sequence(hash: u64) -> [uint, ..2] {
        [  hash        as uint & 0x1Fu
        , (hash >> 5u) as uint & 0x1Fu
        ]
    }

    pub fn insert_hashed(&mut self, hash: u64) {
        for &off in TinyBloomFilter::probe_sequence(hash).iter() {
            self.bits |= 1u32 << off;
        }
    }

    pub fn may_include_hashed(&self, hash: u64) -> bool {
        let mut ret = true;

        // we can only say something is in the bloom filter iff all of the bits
        // in its probe sequence are 1.
        for &off in TinyBloomFilter::probe_sequence(hash).iter() {
            ret &= ((self.bits >> off) & 1u32) != 0
        }

        ret
    }

    // TODO(cgaebel): SipHash is a little heavyweight. maybe we want something
    // lighter?

    pub fn insert<T: Hash>(&mut self, t: &T) {
        self.insert_hashed(hash(t));
    }

    pub fn definitely_excludes<T: Hash>(&self, t: &T) -> bool {
        !self.may_include(t)
    }

    pub fn may_include<T: Hash>(&self, t: &T) -> bool {
        self.may_include_hashed(hash(t))
    }

    pub fn clear(&mut self) {
        self.bits = 0u32;
    }
}

#[test]
fn test_create_and_insert() {
    let mut f = TinyBloomFilter::new();

    f.insert(&1u);
    f.insert(&2u);
    f.insert(&3u);

    assert!(f.may_include(&1u));
    assert!(f.may_include(&2u));
    assert!(f.may_include(&3u));
    assert!(f.definitely_excludes(&10u));
}

#[test]
fn test_implicitly_copyable() {
    let mut f0 = TinyBloomFilter::new();
    let mut f1 = f0;

    f0.insert(&0u);
    f1.insert(&1u);

    assert!(f0.may_include(&0u));
    assert!(f1.may_include(&1u));
    assert!(f0.definitely_excludes(&1u));
    assert!(f1.definitely_excludes(&0u));
}
