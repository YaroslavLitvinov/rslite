/// Stable Rust formatting utilities - replaces C variadics
/// Uses std::fmt::Write for type-safe, efficient string building

use std::fmt::Write;

/// Adapter to write to JsonString using Rust's fmt::Write trait
pub struct JsonStringWriter<'a> {
    buf: &'a mut [u8],
    written: usize,
}

impl<'a> JsonStringWriter<'a> {
    /// Create a new writer for the given buffer
    pub fn new(buf: &'a mut [u8]) -> Self {
        JsonStringWriter {
            buf,
            written: 0,
        }
    }

    /// Get number of bytes written
    pub fn written(&self) -> usize {
        self.written
    }
}

impl<'a> Write for JsonStringWriter<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if self.written + len > self.buf.len() {
            return Err(std::fmt::Error);
        }

        self.buf[self.written..self.written + len]
            .copy_from_slice(bytes);

        self.written += len;
        Ok(())
    }
}

/// Format arguments into a JsonString
///
/// # Safety
/// Caller must ensure:
/// - zBuf and nAlloc are valid
/// - sufficient capacity exists (use jsonStringGrow if needed)
pub unsafe fn json_printf(
    p: *mut crate::src::src::json::JsonString,
    args: std::fmt::Arguments,
) -> std::fmt::Result {
    if p.is_null() {
        return Err(std::fmt::Error);
    }

    let p_ref = &mut *p;

    // Get mutable slice of the buffer
    let buf = std::slice::from_raw_parts_mut(
        p_ref.zBuf as *mut u8,
        p_ref.nAlloc as usize,
    );

    // Create writer starting at current position
    let offset = p_ref.nUsed as usize;
    if offset > buf.len() {
        return Err(std::fmt::Error);
    }

    let tail = &mut buf[offset..];
    let mut writer = JsonStringWriter::new(tail);

    // Write formatted arguments
    write!(writer, "{}", args)?;

    // Update used count
    p_ref.nUsed += writer.written() as u64;

    Ok(())
}

/// Format float using SQLite's %!0.15g format
/// Formats with 15 significant digits and strips trailing zeros after decimal
pub fn format_g15(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_positive() { "9.0e+999".to_string() } else { "-9.0e+999".to_string() };
    }

    if v == 0.0 {
        return "0".to_string();
    }

    // Use exponential notation if exponent is outside [-4, 15) range
    let log10_abs = v.abs().log10();
    let use_exp = log10_abs < -4.0 || log10_abs >= 15.0;

    let mut result = if use_exp {
        // Exponential format with 14 decimal places = 15 significant figures
        format!("{:.14e}", v)
    } else {
        // For fixed notation, calculate the number of decimal places needed
        // to represent 15 significant figures
        let int_digits = if v.abs() >= 1.0 {
            (v.abs().log10().floor() as i32 + 1) as usize
        } else {
            1
        };

        let decimal_places = if int_digits < 15 {
            15 - int_digits
        } else {
            0
        };

        let formatted = format!("{:.prec$}", v, prec = decimal_places);
        formatted
    };

    // Strip trailing zeros after decimal point (but not in exponential notation)
    if result.contains('.') && !result.contains('e') && !result.contains('E') {
        while result.ends_with('0') {
            result.pop();
        }
        if result.ends_with('.') {
            result.pop();
        }
    }

    result
}
