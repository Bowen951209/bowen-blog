---
layout: post
title:  "Integration By Parts 3D"
date:   2025-10-12 +0800
categories: math
---

$$\iiint_V f \left(\nabla \cdot \vec A\right) dV = ?$$

Integration by parts is derived from the product rule for derivatives:

$$
\begin{align}  
&\frac{d}{dx} (fg) = f \left(\frac{dg}{dx} \right) + g \left(\frac{df}{dx} \right) \\
&\Leftrightarrow
\int \frac{d}{dx} (fg) dx = \int f \left(\frac{dg}{dx} \right) dx + \int g \left(\frac{df}{dx} \right) dx \\
&\Leftrightarrow
\int f \left(\frac{dg}{dx} \right) dx = \int \frac{d}{dx} (fg) dx - \int g \left(\frac{df}{dx} \right) dx
= fg - \int g \left(\frac{df}{dx} \right) dx.
\end{align}
$$

We can do the similar to multivariables. Let $$f$$ be a scalar field, and $$\vec A$$ be a vector field,

$$
\begin{align}
\nabla \cdot (f \vec A)
&= \left(\frac{\partial f}{\partial x} A_x + f \frac{\partial A_x}{\partial x}\right)
+ \left(\frac{\partial f}{\partial y} A_y + f \frac{\partial A_y}{\partial y}\right)
+ \left(\frac{\partial f}{\partial z} A_z + f \frac{\partial A_z}{\partial z}\right)\\
&= \nabla f \cdot \vec A + f \left(\nabla \cdot \vec A\right).
\end{align}
$$

Integrate both sides over a volume $$V$$ and use the divergence theorem, we get

$$
\begin{align}
&\oiint_{\partial V} f \vec A \cdot d \vec S
= \iiint_V \nabla \cdot (f \vec A) dV
= \iiint_V \nabla f \cdot \vec A dV + \iiint_V f \left(\nabla \cdot \vec A\right) dV\\
&\Leftrightarrow
\iiint_V f \left(\nabla \cdot \vec A\right) dV
= \oiint_{\partial V} f \vec A \cdot d \vec S - \iiint_V \nabla f \cdot \vec A dV.
\end{align}
$$