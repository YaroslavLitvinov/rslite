//! SQLite on-disk record format encoder/decoder.
//!
//! Format: [header_size: varint] [serial_type: varint]... [data]...
//!
//! Serial types:
//!   0          → NULL
//!   1..=5      → 1/2/3/4/6-byte signed int (big-endian, sign-extended)
//!   6          → 8-byte signed int
//!   7          → 8-byte IEEE 754 big-endian float
//!   8          → integer constant 0 (zero bytes)
//!   9          → integer constant 1 (zero bytes)
//!   n even ≥12 → blob of (n−12)/2 bytes
//!   n odd  ≥13 → UTF-8 text of (n−13)/2 bytes

use super::exec::Register;

// ── Encoder ───────────────────────────────────────────────────────────────────

/// Serialize a slice of Register values into SQLite record format.
pub fn encode_record(cols: &[Register]) -> Vec<u8> {
    let mut serial_types: Vec<u64> = Vec::with_capacity(cols.len());
    let mut data: Vec<u8> = Vec::new();

    for col in cols {
        let st = match col {
            Register::Null => 0,
            Register::Integer(v) => smallest_int_serial(*v),
            Register::Real(_) => 7,
            Register::Text(s) => s.len() as u64 * 2 + 13,
            Register::Blob(b) => b.len() as u64 * 2 + 12,
        };
        serial_types.push(st);

        match col {
            Register::Null => {}
            Register::Integer(v) => push_int_bytes(&mut data, *v, st),
            Register::Real(v) => data.extend_from_slice(&v.to_bits().to_be_bytes()),
            Register::Text(s) => data.extend_from_slice(s.as_bytes()),
            Register::Blob(b) => data.extend_from_slice(b),
        }
    }

    // Build header bytes (serial types only; header_size varint prepended below).
    let mut header: Vec<u8> = Vec::new();
    for &st in &serial_types {
        push_varint(&mut header, st);
    }

    // Header size = varint(header_size_field) + header body.
    // The varint for `header_size` itself is usually 1 byte, but may be 2.
    let size_with_one = 1 + header.len();
    let hdr_size = if varint_len(size_with_one as u64) == 1 {
        size_with_one
    } else {
        2 + header.len()
    };

    let mut out = Vec::with_capacity(hdr_size + data.len());
    push_varint(&mut out, hdr_size as u64);
    out.extend_from_slice(&header);
    out.extend_from_slice(&data);
    out
}

// ── Decoder ───────────────────────────────────────────────────────────────────

/// Read column `col` (0-based) from a serialised SQLite record payload.
pub fn decode_column(payload: &[u8], col: usize) -> Result<Register, ()> {
    if payload.is_empty() {
        return Ok(Register::Null);
    }

    let (header_size, mut pos) = read_varint(payload, 0)?;
    let header_end = header_size as usize;
    if header_end > payload.len() {
        return Err(());
    }

    // Collect serial types.
    let mut serial_types: Vec<u64> = Vec::new();
    while pos < header_end {
        let (st, new_pos) = read_varint(payload, pos)?;
        serial_types.push(st);
        pos = new_pos;
    }

    if col >= serial_types.len() {
        return Ok(Register::Null);
    }

    // Compute byte offset for column `col`.
    let mut data_pos = header_end;
    for &st in &serial_types[..col] {
        data_pos += serial_type_content_len(st) as usize;
    }

    decode_value(payload, data_pos, serial_types[col])
}

fn decode_value(payload: &[u8], pos: usize, serial_type: u64) -> Result<Register, ()> {
    match serial_type {
        0 | 10 | 11 => Ok(Register::Null),
        8 => Ok(Register::Integer(0)),
        9 => Ok(Register::Integer(1)),
        7 => {
            if pos + 8 > payload.len() {
                return Err(());
            }
            let bits = u64::from_be_bytes(payload[pos..pos + 8].try_into().unwrap());
            Ok(Register::Real(f64::from_bits(bits)))
        }
        1..=6 => {
            let len = match serial_type {
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 4,
                5 => 6,
                6 => 8,
                _ => unreachable!(),
            };
            if pos + len > payload.len() {
                return Err(());
            }
            Ok(Register::Integer(read_signed_be(&payload[pos..pos + len])))
        }
        n if n >= 12 && n % 2 == 0 => {
            let len = ((n - 12) / 2) as usize;
            if pos + len > payload.len() {
                return Err(());
            }
            Ok(Register::Blob(payload[pos..pos + len].to_vec()))
        }
        n if n >= 13 => {
            let len = ((n - 13) / 2) as usize;
            if pos + len > payload.len() {
                return Err(());
            }
            let s = std::str::from_utf8(&payload[pos..pos + len]).map_err(|_| ())?;
            Ok(Register::Text(s.to_string()))
        }
        _ => Ok(Register::Null),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn serial_type_content_len(st: u64) -> u64 {
    match st {
        0 | 8 | 9 | 10 | 11 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 6,
        6 | 7 => 8,
        n if n >= 12 && n % 2 == 0 => (n - 12) / 2,
        n if n >= 13 => (n - 13) / 2,
        _ => 0,
    }
}

fn smallest_int_serial(v: i64) -> u64 {
    match v {
        0 => 8,
        1 => 9,
        -128..=127 => 1,
        -32768..=32767 => 2,
        -8388608..=8388607 => 3,
        -2147483648..=2147483647 => 4,
        -140737488355328..=140737488355327 => 5,
        _ => 6,
    }
}

fn push_int_bytes(buf: &mut Vec<u8>, v: i64, serial_type: u64) {
    let n = match serial_type {
        1 => 1usize,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 6,
        6 => 8,
        8 | 9 => return, // zero bytes
        _ => return,
    };
    let be = v.to_be_bytes();
    buf.extend_from_slice(&be[8 - n..]);
}

/// SQLite varint: big-endian, 7 bits per byte, MSB = continuation.
/// The 9th byte (if needed) uses all 8 bits.
pub fn push_varint(buf: &mut Vec<u8>, v: u64) {
    if v <= 0x7f {
        buf.push(v as u8);
        return;
    }
    // Collect 7-bit chunks from LSB; then reverse for big-endian output.
    let mut tmp = [0u8; 9];
    let mut n = 0usize;
    let mut v = v;
    loop {
        tmp[n] = (v as u8 & 0x7f) | 0x80;
        n += 1;
        v >>= 7;
        if v == 0 {
            break;
        }
    }
    tmp[0] &= 0x7f; // clear continuation bit on the least-significant group (last byte output)
    for i in (0..n).rev() {
        buf.push(tmp[i]);
    }
}

fn varint_len(v: u64) -> usize {
    let mut tmp = Vec::new();
    push_varint(&mut tmp, v);
    tmp.len()
}

/// Decode a varint from `buf` starting at `pos`. Returns (value, new_pos).
pub fn read_varint(buf: &[u8], mut pos: usize) -> Result<(u64, usize), ()> {
    if pos >= buf.len() {
        return Err(());
    }
    let b = buf[pos];
    if b & 0x80 == 0 {
        return Ok((b as u64, pos + 1));
    }

    let mut val: u64 = 0;
    for i in 0..9 {
        if pos >= buf.len() {
            return Err(());
        }
        let b = buf[pos];
        pos += 1;
        if i == 8 {
            val = (val << 8) | b as u64;
            break;
        }
        val = (val << 7) | (b & 0x7f) as u64;
        if b & 0x80 == 0 {
            break;
        }
    }
    Ok((val, pos))
}

fn read_signed_be(bytes: &[u8]) -> i64 {
    let n = bytes.len();
    if n == 0 {
        return 0;
    }
    let sign_extend = bytes[0] & 0x80 != 0;
    let mut val: i64 = 0;
    for &b in bytes {
        val = (val << 8) | b as i64;
    }
    if sign_extend && n < 8 {
        let shift = (8 - n) * 8;
        val = (val << shift) >> shift;
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(cols: &[Register]) {
        let encoded = encode_record(cols);
        for (i, expected) in cols.iter().enumerate() {
            let got = decode_column(&encoded, i).expect("decode_column failed");
            assert_eq!(&got, expected, "col {i} mismatch");
        }
    }

    #[test]
    fn test_basic_types() {
        roundtrip(&[
            Register::Null,
            Register::Integer(42),
            Register::Integer(-1),
            Register::Integer(0),
            Register::Integer(1),
            Register::Real(3.14),
            Register::Text("hello".into()),
            Register::Blob(vec![0x01, 0x02, 0x03]),
        ]);
    }

    #[test]
    fn test_large_integers() {
        roundtrip(&[
            Register::Integer(i64::MAX),
            Register::Integer(i64::MIN),
            Register::Integer(128),
            Register::Integer(-129),
            Register::Integer(32768),
        ]);
    }
}
