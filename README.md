#### Usage:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_url() {
        applescript! {
            tell application "Safari" to open location "https://www.rust-lang.org"
        }
    }

    /* IMPORTANT: multiline requires each line end with ; */
    #[test]
    fn test_multiline_works() {
        applescript! {
            tell application "Safari";
                activate;
                open location "https://www.rust-lang.org";
                set bounds of front window to {100, 100, 1000, 800};
            end tell;
        }
    }

    #[test]
    fn test_list_notes() {
        applescript! {
            tell application "Notes";
                set noteList to "";
                    repeat with aNote in notes;
                        set noteList to noteList & name of aNote & linefeed;
                    end repeat;
                return noteList;
            end tell;
        }
    }
}
```
