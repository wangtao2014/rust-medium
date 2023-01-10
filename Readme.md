# 所有权
## 值删除的顺序
    - 变量（包括函数的参数）按相反的顺序进行删除
    - 嵌套的值按照源代码的顺序进行删除

    - 闭包是匿名函数，但可以捕获外部环境的变量
    - 函数实现了 Fn trait
    - 闭包类型 需要实现 Fn trait

- Rust 一切皆类型，并由 trait 掌握类型的行为逻辑
- 所以 Fn、FnOnce、FnMut 描述的都是闭包将以何种方式去捕获外部变量。
- 它和 Fn 一样都只能获取外部变量的引用，但 Fn 在使用的时候只能拿到不可变引用，而 FnMut 还可以获取可变引用。

    - 如果你需要转移外部变量的所有权，使用 FnOnce，并且闭包只能调用一次；
    - 如果你不需要转移所有权，并且也不需要修改外部变量，那么使用 Fn；
    - 如果你不需要转移所有权，但是需要修改外部变量，那么使用 FnMut；

- 所以问号表达式等价于不会 panic 的 unwrap
    - 调用方和被调用方的返回值都要是 Result 枚举类型