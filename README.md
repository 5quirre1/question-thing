# question-thing

uhh it's really simple actually LMFAO

so what it does is ask the input you give (first param) and if you don't give an input like just pressing enter, it prints out the 2nd param !! really simple but it can be useful i think LAWL

---

small example:


```rust
mod question;

use crate::question::question;

fn main() {
	let s: String = question("put a name!!", "please put a name");
	if s.trim() != "" {
		println!("hai {}!!!", s.trim());
	} else {
		println!("where name how ?!?!? what");
	}
}
```
