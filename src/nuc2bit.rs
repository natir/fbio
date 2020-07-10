/*
Copyright (c) 2020 Pierre Marijon <pmarijon@hhu.de>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

pub fn move_mask(nuc: u8) -> u8 {
    nuc >> 1 & 0b11
}

pub fn move_move(nuc: u8) -> u8 {
    (nuc << 5) >> 6
}

pub fn test_match(nuc: u8) -> u8 {
    match nuc {
        b'A' | b'a' => 0,
        b'C' | b'c' => 1,
        b'T' | b't' => 2,
        b'G' | b'g' => 3,
        _ => 0,
    }
}

pub fn test_if(nuc: u8) -> u8 {
    if nuc == b'A' || nuc == b'a' {
        0
    } else if nuc == b'C' || nuc == b'c' {
        1
    } else if nuc == b'T' || nuc == b't' {
        2
    } else if nuc == b'G' || nuc == b'g' {
        3
    } else {
        0
    }
}

pub fn test_match_upper(mut nuc: u8) -> u8 {
    nuc = nuc.to_ascii_uppercase();

    match nuc {
        b'A' => 0,
        b'C' => 1,
        b'T' => 2,
        b'G' => 3,
        _ => 0,
    }
}

pub fn test_if_upper(mut nuc: u8) -> u8 {
    nuc = nuc.to_ascii_uppercase();

    if nuc == b'A' {
        0
    } else if nuc == b'C' {
        1
    } else if nuc == b'T' {
        2
    } else if nuc == b'G' {
        3
    } else {
        0
    }
}

const LOOKUP: [u8; 256] = {
    let mut lookup = [0; 256];

    lookup[b'A' as usize] = 0;
    lookup[b'C' as usize] = 1;
    lookup[b'G' as usize] = 3;
    lookup[b'T' as usize] = 2;
    lookup[b'a' as usize] = 0;
    lookup[b'c' as usize] = 1;
    lookup[b'g' as usize] = 3;
    lookup[b't' as usize] = 2;

    lookup
};

pub fn lookup(nuc: u8) -> u8 {
    LOOKUP[nuc as usize]
}

pub fn lookup_nocheck(nuc: u8) -> u8 {
    unsafe { *LOOKUP.get_unchecked(nuc as usize) }
}

pub struct GroupVec<'a, const N: usize> {
    seq: &'a [u8],
    cache: [u8; N],
    index_seq: usize,
    index_cache: usize,
}

impl<'a> GroupVec<'a, 16> {
    pub fn new(seq: &'a [u8]) -> Self {
        if seq.len() >= 16 {
            let mut me = Self {
                seq,
                cache: [0; 16],
                index_seq: 0,
                index_cache: 0,
            };

            me.generate_cache();

            me.index_seq = 16;

            me
        } else {
            Self {
                seq,
                cache: [0; 16],
                index_seq: 0,
                index_cache: 16,
            }
        }
    }

    pub fn generate_cache(&mut self) {
        unsafe {
            let mut concat = std::mem::transmute::<[u8; 16], core::arch::x86_64::__m128i>(
                *(&self.seq[self.index_seq..] as *const [u8] as *const [u8; 16]),
            );

            concat = core::arch::x86_64::_mm_srli_epi16(concat, 1); // remove first bit of u16
            concat =
                core::arch::x86_64::_mm_and_si128(concat, core::arch::x86_64::_mm_set1_epi8(3)); // bit and

            self.cache = std::mem::transmute::<core::arch::x86_64::__m128i, [u8; 16]>(concat);
        }
    }
}

impl<'a> GroupVec<'a, 32> {
    pub fn new(seq: &'a [u8]) -> Self {
        if seq.len() >= 32 {
            let mut me = Self {
                seq,
                cache: [0; 32],
                index_seq: 0,
                index_cache: 0,
            };

            me.generate_cache();

            me.index_seq = 32;

            me
        } else {
            Self {
                seq,
                cache: [0; 32],
                index_seq: 0,
                index_cache: 32,
            }
        }
    }

    pub fn generate_cache(&mut self) {
        unsafe {
            let mut concat = std::mem::transmute::<[u8; 32], core::arch::x86_64::__m256i>(
                *(&self.seq[self.index_seq..] as *const [u8] as *const [u8; 32]),
            );

            concat = core::arch::x86_64::_mm256_srli_epi16(concat, 1); // remove first bit of u16
            concat = core::arch::x86_64::_mm256_and_si256(
                concat,
                core::arch::x86_64::_mm256_set1_epi8(3),
            ); // bit and

            self.cache = std::mem::transmute::<core::arch::x86_64::__m256i, [u8; 32]>(concat);
        }
    }
}

macro_rules! group_vec_iter_imp {
    ($size:tt) => {
        impl<'a> Iterator for GroupVec<'a, $size> {
            type Item = u8;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index_cache < $size {
                    self.index_cache += 1;

                    Some(self.cache[self.index_cache - 1])
                } else {
                    if self.index_seq + $size <= self.seq.len() {
                        self.generate_cache();

                        self.index_seq += $size;
                        self.index_cache = 1;

                        Some(self.cache[0])
                    } else if self.index_seq < self.seq.len() {
                        self.index_seq += 1;

                        Some(move_mask(self.seq[self.index_seq - 1]))
                    } else {
                        None
                    }
                }
            }
        }
    };
}

group_vec_iter_imp!(16);
group_vec_iter_imp!(32);

pub struct GroupPhf<'a> {
    seq: &'a [u8],
    cache: &'a [u8; 4],
    index_seq: usize,
    index_cache: usize,
}

impl<'a> GroupPhf<'a> {
    pub fn new(seq: &'a [u8]) -> Self {
        if seq.len() >= 4 {
            let mut me = Self {
                seq,
                cache: &[0; 4],
                index_seq: 0,
                index_cache: 0,
            };

            me.generate_cache();

            me.index_seq = 4;

            me
        } else {
            Self {
                seq,
                cache: &[0; 4],
                index_seq: 0,
                index_cache: 4,
            }
        }
    }

    pub fn generate_cache(&mut self) {
        unsafe {
            self.cache = crate::nuc2bit_phf::LOOKUP_GROUP_PHF
                .get(&std::mem::transmute::<[u8; 4], u32>(
                    *(&self.seq[self.index_seq..] as *const [u8] as *const [u8; 4]),
                ))
                .unwrap();
        }
    }
}

impl<'a> Iterator for GroupPhf<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_cache < 4 {
            self.index_cache += 1;

            Some(self.cache[self.index_cache - 1])
        } else {
            if self.index_seq + 4 <= self.seq.len() {
                self.generate_cache();

                self.index_seq += 4;
                self.index_cache = 1;

                Some(self.cache[0])
            } else if self.index_seq < self.seq.len() {
                self.index_seq += 1;

                Some(move_mask(self.seq[self.index_seq - 1]))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn move_mask() {
        assert_eq!(0, super::move_mask(b'A'));
        assert_eq!(1, super::move_mask(b'C'));
        assert_eq!(2, super::move_mask(b'T'));
        assert_eq!(3, super::move_mask(b'G'));
        assert_eq!(0, super::move_mask(b'a'));
        assert_eq!(1, super::move_mask(b'c'));
        assert_eq!(2, super::move_mask(b't'));
        assert_eq!(3, super::move_mask(b'g'));

        assert_eq!(3, super::move_mask(b'N'));
    }

    #[test]
    fn move_move() {
        for i in 0..=255 {
            assert_eq!(super::move_mask(i), super::move_move(i));
        }
    }

    #[test]
    fn test_match() {
        assert_eq!(0, super::test_match(b'A'));
        assert_eq!(1, super::test_match(b'C'));
        assert_eq!(2, super::test_match(b'T'));
        assert_eq!(3, super::test_match(b'G'));
        assert_eq!(0, super::test_match(b'a'));
        assert_eq!(1, super::test_match(b'c'));
        assert_eq!(2, super::test_match(b't'));
        assert_eq!(3, super::test_match(b'g'));

        assert_eq!(0, super::test_match(b'N'));
    }

    #[test]
    fn test_if() {
        for i in 0..=255 {
            assert_eq!(super::test_match(i), super::test_if(i));
        }
    }

    #[test]
    fn test_match_upper() {
        for i in 0..=255 {
            assert_eq!(super::test_match(i), super::test_match_upper(i));
        }
    }

    #[test]
    fn test_if_upper() {
        for i in 0..=255 {
            assert_eq!(super::test_match(i), super::test_if_upper(i));
        }
    }

    #[test]
    fn lookup() {
        for i in 0..=255 {
            assert_eq!(super::test_match(i), super::lookup(i));
        }
    }

    #[test]
    fn lookup_nocheck() {
        for i in 0..=255 {
            assert_eq!(super::test_match(i), super::lookup_nocheck(i));
        }
    }

    #[test]
    fn group_vector16() {
        assert_eq!(
            super::GroupVec::<16>::new(b"ACTG").collect::<Vec<u8>>(),
            vec![0, 1, 2, 3]
        );

        assert_eq!(
            super::GroupVec::<16>::new(b"ACTGactg").collect::<Vec<u8>>(),
            vec![0, 1, 2, 3, 0, 1, 2, 3]
        );

        assert_eq!(
            super::GroupVec::<16>::new(b"ACTGactgACTGact").collect::<Vec<u8>>(),
            vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2]
        );

        assert_eq!(
            super::GroupVec::<16>::new(b"ACTGactgACTGactg")
                .into_iter()
                .collect::<Vec<u8>>(),
            vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]
        );

        assert_eq!(
            super::GroupVec::<16>::new(b"ACTGactgACTGactgNN")
                .into_iter()
                .collect::<Vec<u8>>(),
            vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 3, 3]
        );

        assert_eq!(
            super::GroupVec::<16>::new(b"CAGAACCCCAATAAACCNCAGAACCCCAATAAACC")
                .into_iter()
                .collect::<Vec<u8>>(),
            vec![
                1, 0, 3, 0, 0, 1, 1, 1, 1, 0, 0, 2, 0, 0, 0, 1, 1, 3, 1, 0, 3, 0, 0, 1, 1, 1, 1, 0,
                0, 2, 0, 0, 0, 1, 1
            ]
        );
    }

    #[test]
    fn group_vector32() {
        assert_eq!(
            super::GroupVec::<32>::new(b"ACTGactgACTGactgACTGactgACTGactg")
                .into_iter()
                .collect::<Vec<u8>>(),
            vec![
                0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3,
                0, 1, 2, 3
            ]
        );

        assert_eq!(
            super::GroupVec::<32>::new(b"ACTGactgACTGactgACTGactgACTGact")
                .into_iter()
                .collect::<Vec<u8>>(),
            vec![
                0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3,
                0, 1, 2
            ]
        );

        assert_eq!(
            super::GroupVec::<32>::new(b"ACTGactgACTGactgACTGactgACTGactgN")
                .into_iter()
                .collect::<Vec<u8>>(),
            vec![
                0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3,
                0, 1, 2, 3, 3
            ]
        );
    }

    #[test]
    fn group_phf() {
        assert_eq!(
            super::GroupPhf::new(b"ACT").collect::<Vec<u8>>(),
            vec![0, 1, 2]
        );

        assert_eq!(
            super::GroupPhf::new(b"ACTGA").collect::<Vec<u8>>(),
            vec![0, 1, 2, 3, 0]
        );
    }
}
