[TOML 教程 - 可能是目前最好的配置文件格式
](https://zhuanlan.zhihu.com/p/50412485)
特点 嵌套方式不通过缩进(yaml)或者括号(json)
通过显式声明[x.x]
例如
```python
[d]
x = "d.x"
y = "d.y"

[[e]]
x = "e[0].x"
y = "e[0].y"

[[e]]
x = "e[1].x"
y = "e[1].y"

[f.A]
x.y = "f.A.x.y"

[f.B]
x.y = """
f.
  B.
    x.
      y
"""
```