# Here are some syntax cases of the developing DSL for zkSNARKs

### Sum
```python
# snark - it's a special sort of functions,
# it returns nothing, private and public are specifiers for input variables
snark sum(private x, y; public s):
    # constraint - it's a keyword to ensure the equity,
    # it can be used in snarks and gadgets as well
    constraint x + y == s
```

### Inequity
```python
snark gt(private x; public a):
    # Inequities are allowed in constraint,
    # but it's necessary to remember they generate a bunch of constraints in R1CS,
    # you should avoid them as much as possible
    constraint x > a
```

### Square equation
```python
snark square_equation(private x; public c[3]):  # We can use arrays with the syntax like a[10]
    constraint c[0] + c[1] * x + c[2] * x**2 == 0  # There can be long expressions in constraint
```

### Polynomial equation
```python
# A constant
const N = 5

snark polinomial_equation(private x, public c[N]):
    y = c[0]
    xn = 1
    for i in [1:N]:  # Only loops with fixed number of iterations are allowed
        xn *= x
        y += xn * c[i]
    constraint y == 0
```

### Mean with sum gadget
```python
const N = 10

# A gadget example
gadget sum(a[N]):
    s = 0
    for i in [0:N-1]:
        s += a[i]
    return s

snark mean(private x[N]; public m):
    s = sum(x)  # Calling the gadget
    constraint m == s / N
```

### Median (approximate implementation)
```python
const N = 10

snark median(private x[N]; public m):
    u = 0
    d = 0
    for i in [0:N-1]:
        if x[i] >= m:  # if-elif-else statements are allowed
            u += 1
        if x[i] <= m:
            d += 1
    constraint u == d
```
