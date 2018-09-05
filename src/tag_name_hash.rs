// NOTE: unfortunately `static fn` is still unavaliable in stable,
// so we need to use manually precalculated values in this enum.
// Consistency between hashing algorithm and these values is guaranteed
// by the dedicated test.
#[repr(u64)]
#[derive(Debug)]
pub enum TagNameHash {
    Svg = 25452u64,
    Math = 596781u64,
    H1 = 416u64,
}

// NOTE: All standard tag names contain only ASCII alpha characters
// and digits from 1 to 6 (in numbered header tags, i.e. <h1> - <h6>).
// Considering that tag names are case insensitive we have only
// 26 + 6 = 32 characters. Thus, single character can be encoded in
// 5 bits and we can fit up to 64 / 5 ≈ 12 characters in a 64-bit
// integer. This is enough to encode all standard tag names, so
// we can just compare integers instead of expensive string
// comparison for tag names.
//
// The original idea of this tag hash-like thing belongs to Ingvar
// Stepanyan and was implemented in lazyhtml. So, kudos to him for
// comming up with this cool optimisation. This implementation differs
// from the original one as it adds ability to encode digits from 1
// to 6 which allows us to encode numbered header tags.
//
// In this implementation we reserve numbers from 0 to 5 for digits
// from 1 to 6 and numbers from 6 to 31 for ASCII alphas. Otherwise,
// if we use numbers from 0 to 25 for ASCII alphas we'll have an
// ambiguity for repetitative `a` characters: both `a`,
// `aaa` and even `aaaaa` will give us 0 as a hash. It's still a case
// for digits, but considering that tag name can't start from digit
// we are safe here, since we'll just get first character shifted left
// by zeroes as repetitave 1 digits get added to the hash.
#[inline]
pub fn update_tag_name_hash(hash: Option<u64>, ch: u8) -> Option<u64> {
    let mut hash = hash;

    if let Some(h) = hash {
        // NOTE: check if we still have space for yet another
        // character and if not then invalidate the hash.
        // Note, that we can't have `1` (which is encoded as 0b00000) as
        // a first character of a tag name, so it's safe to perform
        // check this way.
        hash = if h >> (64 - 5) == 0 {
            match ch {
                // NOTE: apply 0x1F mask on ASCII alpha to convert it to the
                // number from 1 to 26 (character case is controlled by one of
                // upper bits which we eliminate with the mask). Then add
                // 5, since numbers from 0 to 5 are reserved for digits.
                // Aftwerards put result as 5 lower bits of the hash.
                b'a'...b'z' | b'A'...b'Z' => Some((h << 5) | (ch as u64 & 0x1F) + 5),

                // NOTE: apply 0x0F on ASCII digit to convert it to number
                // from 1 to 6. Then substract 1 to make it zero-based.
                // Afterwards, put result as lower bits of the hash.
                b'1'...b'6' => Some((h << 5) | (ch as u64 & 0x0F) - 1),

                // NOTE: for any other characters hash function is not
                // applicable, so we completely invalidate the hash.
                _ => None,
            }
        } else {
            None
        };
    }

    hash
}

pub fn get_tag_name_hash(name: &str) -> Option<u64> {
    let mut hash = Some(0);

    for ch in name.bytes() {
        hash = update_tag_name_hash(hash, ch);
    }

    hash
}