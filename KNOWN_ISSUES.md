# Extensions can no longer call sqlite3_mprintf through the extension API
We doesn't populate function pointer anymore, in `loadext.rs`
```rust
  mprintf: Some(                                                  
      crate::src::src::printf::sqlite3_mprintf
          as unsafe extern "C" fn(
              *const ::core::ffi::c_char,
              //...
          ) -> *mut ::core::ffi::c_char,                                                                                                                                
  )
```
Before: Extensions could call sqlite3_mprintf via the extension API
After: Extensions cannot - they'd need to either:
  1. Not use mprintf at all
  2. Use alternative SQLite string formatting functions (like sqlite3_snprintf)
  3. Link against sqlite3_mprintf some other way (but that would probably also fail if c_variadic is disabled)


