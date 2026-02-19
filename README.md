# unit repl

一个命令行单位转换器

## 复合单位

```
expr      := number unit_expr "to" unit_expr EOF

unit_expr := unit_term (("*" | "/") unit_term)*

unit_term := unit_atom ("^" signed_int)?

unit_atom := ident

ident     := alpha (alnum | ".")*

signed_int:= ("+"|"-")? digit+

number    := ("+"|"-")? digit+ ("." digit*)? (("e"|"E") ("+"|"-")? digit+)?   // 数字后必须是空白或EOF
```