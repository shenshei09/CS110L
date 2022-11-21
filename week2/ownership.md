Example 1:
```
fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    s = String::from("goodbye");
    println!("{}", ref3.to_uppercase());
}
```

error[E0506]: cannot assign to `s` because it is borrowed

After correction:
```
fn main() {
    let mut s = String::from("hello");
    let ref1 = s.clone();
    let ref2 = &ref1;
    let ref3 = &ref2;
    s = String::from("goodbye");
    println!("{}", ref3.to_uppercase());
}
```




Example 2:
```
fn drip_drop() -> &String {
    let s = String::from("hello world!");
    return &s;
}
```

error[E0106]: missing lifetime specifier

After correction:
```
fn drip_drop() -> String {
    let s = String::from("hello world!");
    return s;
}
```




Example 3:
```
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0];
    println!("{}", s2);
}
```

error[E0507]: cannot move out of index of `Vec<String>`

After correction:
```
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0].clone();
    println!("{}", s2);
}
```