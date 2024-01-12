This library implements case conversion methods for the String type.  
Credit : the "uppercase first letter" function comes from [this post](https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust)

```
use case_convert::CaseConvert;
let s = String::from("test");
assert_eq!(s.uppercase_first(), "Test");
```

0.1.0 : Converts to uppercase the first letter of a string