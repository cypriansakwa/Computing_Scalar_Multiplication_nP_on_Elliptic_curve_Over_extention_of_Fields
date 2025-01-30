# Elliptic Curve Arithmetic over Finite Field Extensions $\mathbb{F}_p^k$

This project implements elliptic curve arithmetic over a finite field extension $\mathbb{F}_p^k$, where `p` is a prime and `k` is the extension degree. The implementation is generic and allows changing `p` and `k`. The default example uses **F₅²**.

## Features

- Finite field arithmetic in $\mathbb{F}_p^k$ (addition, subtraction, multiplication, division, inversion)
- Definition of elliptic curve points over $\mathbb{F}_p^k$
- Elliptic curve point addition and doubling
- Scalar multiplication using the double-and-add method

## Finite Field Arithmetic

Elements of $\mathbb{F}_p^k$ are represented as:

$a + bt$, where $t^k$ is reduced modulo an irreducible polynomial


The reduction polynomial determines the field structure. The default implementation uses:

$t² \equiv 3 \mod 5$ (for $\mathbb{F}_5^2$)


### Operations:

- **Addition:**  $(a + bt) + (c + dt) = (a + c) + (b + d)t$
- **Subtraction:**  $(a + bt)-(c + dt)=(a - c) + (b - d)t$
- **Multiplication:**  $(a + bt) \cdot(c + dt) = ac + (ad + bc)t + bd \cdot \text{reduction factor}$
- **Division:** Uses field inversion and multiplication
- **Inversion:**  $(a + bt)^{-1} = (a -bt) / (a^2-\text{reduction factor} \cdot b^2)$
- **Modular Inverse:** Computed using the extended Euclidean algorithm

## Elliptic Curve Arithmetic

We consider the elliptic curve equation:

$y^2 = x^3 + ax + b$


Operations:

- **Point Addition:** Implements standard EC addition rules
- **Point Doubling:** Uses the derivative formula for doubling
- **Scalar Multiplication:** Implements double-and-add method
- **Infinity Point Handling:** The point at infinity is properly handled

## Customization

- The field $\mathbb{F}_p^k$ can be changed by modifying `p`, `k`, and the reduction polynomial.
- The elliptic curve parameters `a` and `b` can be set to define different curves.

## Usage

### Compile and Run

```sh
cargo run
```
### Example Output
$[3]P = \text{Point at Infinity}$

