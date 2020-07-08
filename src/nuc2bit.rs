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
    unsafe{ *LOOKUP.get_unchecked(nuc as usize) }
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
}
