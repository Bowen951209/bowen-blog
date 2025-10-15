---
layout: post
title:  "The Condition For An Infinite Integral To Converge"
date:   2025-10-12 +0800
categories: math
---

This is the improper integral we want to look at:

$$
I = \int_a^\infty f(x) dx.
$$

I will show that for the integral to coverge (exist), $$f(x)$$ must decline *faster* than $$1/x$$ for large $$x$$. *(This only works for monotonically decaying functions. Functions like $$\frac{\sin x}{x}$$ or $$\sin(x^2)$$ won't apply. My note is very unrigorous.)*

We can split an integral into two parts:

$$
I = \int_a^b f(x) dx + \int_b^\infty f(x) dx.
$$

Because the first term is a finite integral, saying $$I$$ exists is equivalent as saying $$\int_b^\infty f(x) dx$$ exists, for any large $$b$$.

Notice that

$$
\int_b^\infty x^p dx =
\begin{cases}\begin{array}{lr}
\frac{x^{1+p}}{1+p}|_{b}^{\infty}&, p\neq -1\\
\ln(x)|_{b}^{\infty}&,p= -1
\end{array}
\end{cases}
.
$$

When $$p = -1,$$ the integral is $$\lim_{x \to \infty} \ln(x) - \ln(b),$$ which diverge.
If something declines *slower* than $$x^{-1}=1/x$$, it of course diverge, too!

$$
\text{If p $\geq$ -1, }
\int_b^\infty x^p dx \,\, \text{diverges}.
$$

When $$p<-1,$$

$$
\int_b^\infty x^p dx = \lim_{x \to \infty}\frac{ x^{1+p} - b^{1+p}}{1+p}.
$$

Since $$p<-1 \Leftrightarrow 1 + p < 0,$$

$$
\lim_{x \to \infty}x^{1+p} = 0 \implies\int_b^\infty x^p dx = -\frac{b^{1+p}}{1+p} = \text{constant}.
$$

We thus proved that, for an integral $$\int_a^\infty f(x) dx$$ to converge, plus $$f(x)$$ is monotonically decaying function, we must have

$$
f(x) \sim \frac{1}{x^a}, a > 1, \text{for large $x$}.
$$

### What one cannot conclude without extra hypotheses

One cannot in general deduce that "$$f(x)$$ decays faster than $$1/x$$" from the convergence of $$\int_a^{\infty} f(x) dx $$ alone. For example, if $$f$$ oscillates (e.g. $$f(x)=\sin x/x$$) or if $$f$$ has sparse narrow spikes whose areas sum to a finite total, the integral may converge while $$x f(x)$$ does not tend to zero.