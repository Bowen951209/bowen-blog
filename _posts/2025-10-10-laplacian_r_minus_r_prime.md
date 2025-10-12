---
layout: post
title:  "The laplacian of 1 / |r - r'|"
date:   2025-10-10 +0800
categories: math
---

I want to know 

$$
\nabla^2 (\frac{1}{|\vec r - \vec r'|}) = \nabla \cdot \left( \nabla (\frac{1}{|\vec r - \vec r'|}) \right).
$$


# Method 1 --- Cartesian Coordinates

Let's first look at 
$$\nabla (\frac{1}{|\vec r - \vec r'|})$$. Let

$$
|\vec r - \vec r'| = \sqrt{(x-x')^2 + (y-y')^2 + (z-z')^2},
$$

and so

$$
\frac{\partial}{\partial x}\left( \frac{1}{|\vec r - \vec r'|}\right)
= \frac{x'-x}{|\vec r - \vec r'|^3}, \\
\frac{\partial}{\partial y}\left( \frac{1}{|\vec r - \vec r'|}\right)
= \frac{y'-y}{|\vec r - \vec r'|^3}, \\
\frac{\partial}{\partial z}\left( \frac{1}{|\vec r - \vec r'|}\right)
= \frac{z'-z}{|\vec r - \vec r'|^3}.
$$

Thus, 

$$
\nabla (\frac{1}{|\vec r - \vec r'|}) 
= \frac{(x'-x)\ihat+(y'-y)\jhat+(z'-z)\khat}{|\vec r - \vec r'|^3}
= \frac{\vec r' - \vec r}{|\vec r - \vec r'|^3}.
$$

Then, we want
$$\nabla^2 (\frac{1}{|\vec r - \vec r'|}) = \nabla \cdot \left( \frac{\vec r' - \vec r}{|\vec r - \vec r'|^3} \right).$$
Notice that (after verbose but straightforward calculation),

$$
\frac{\partial}{\partial x} \frac{x'-x}{|\vec r - \vec r'|^3} = \frac{-|\vec r - \vec r'|^3 + 3|\vec r - \vec r'|(x'-x)^2}{|\vec r - \vec r'|^6},\\
\frac{\partial}{\partial y} \frac{y'-y}{|\vec r - \vec r'|^3} = \frac{-|\vec r - \vec r'|^3 + 3|\vec r - \vec r'|(y'-y)^2}{|\vec r - \vec r'|^6},\\
\frac{\partial}{\partial z} \frac{z'-z}{|\vec r - \vec r'|^3} = \frac{-|\vec r - \vec r'|^3 + 3|\vec r - \vec r'|(z'-z)^2}{|\vec r - \vec r'|^6}.
$$

The divergence is to adding them up, which is

$$
\begin{align}
&\frac{-|\vec r - \vec r'|^3 + 3|\vec r - \vec r'|\left((x'-x)^2+(y'-y)^2+(z'-z)^2\right)}{|\vec r - \vec r'|^6} \\
&= \frac{-|\vec r - \vec r'|^3 + 3|\vec r - \vec r'|\cdot |\vec r - \vec r'|^2}{|\vec r - \vec r'|^6}\\
&= \frac{-|\vec r - \vec r'|^3 + 3|\vec r - \vec r'|^3}{|\vec r - \vec r'|^6} \\
&= 0. \quad \text{(For $|\vec r - \vec r'| \neq 0$)}
\end{align}
$$

We thus get for 
$$|\vec r - \vec r'| \neq 0,$$

$$
\nabla^2 (\frac{1}{|\vec r - \vec r'|}) = 0.
$$

# Method 2 --- Spherical Coordinates

It is best to calculate this in spherical coordinate system.
Here's the result for the laplacian operator in spherical coordinate system.

$$
\nabla^2 = \frac{1}{r^2}\frac{\partial}{\partial r}\left(r^2 \frac{\partial}{\partial r}\right) + \frac{1}{r^2\sin\theta}\frac{\partial}{\partial \theta}\left(\sin\theta \frac{\partial}{\partial \theta}\right) + \frac{1}{r^2\sin^2\theta}\frac{\partial^2}{\partial \phi^2}.
$$

Our function has nothing to do with $$\theta$$ and $$\phi$$, so it simplifies to

$$
\nabla^2 \frac{1}{|\vec r|} = \frac{1}{r^2}\frac{\partial}{\partial r}\left(r^2 \frac{\partial}{\partial r}\left(\frac{1}{r}\right)\right)
= 0.
$$

You just offset the thing and get the form in $$\vec r - \vec r'$$.

# What Happens At The Origin?

Because the divergence is zero everywhere except origin,
by a consequence of Divergence Theorem, any shape around the origin will have the same flux.
Let's just pick $$V$$ to be an unit sphere for simplicity.

$$
\oiint_{\partial S} \frac{\vec r}{|\vec r|^3}\cdot d\vec A
= \oiint_{\partial S} dA
= 4\pi \\
= \iiint_V \nabla\cdot\left(\frac{\vec r}{|\vec r|^3}\right)\,dV
\implies \nabla\cdot\left(\frac{\vec r}{|\vec r|^3}\right) = 4\pi\,\delta^3(\vec r).
$$

Thus we conclude

$$
\nabla^2 \frac{1}{|\vec r|} = -4\pi\,\delta^3(\vec r),
$$

And the offset version

$$
\nabla^2 \frac{1}{|\vec r - \vec r'|} = -4\pi\,\delta^3(\vec r - \vec r').
$$


$$
\tdplotsetmaincoords{60}{120}
\begin{tikzpicture}[tdplot_main_coords,scale=2]

\shade[ball color = gray!40, opacity = 0.3] (0,0) circle (1cm);
  \draw (0,0) circle (1cm);

% Place arrows throughout a grid (excluding origin and outside sphere)
\foreach \x in {-1,-0.5,0,0.5,1}
  \foreach \y in {-1,-0.5,0,0.5,1}
    \foreach \z in {-1,-0.5,0,0.5,1}
      {
        \pgfmathsetmacro{\r}{sqrt(\x*\x+\y*\y+\z*\z)}
        \ifdim\r pt > 0.2pt
          \ifdim\r pt < 1.1pt
            % Arrow length scaled for illustration
            \pgfmathsetmacro{\len}{0.25/(\r*\r)}
            % Arrow endpoint
            \pgfmathsetmacro{\xe}{\x*\len}
            \pgfmathsetmacro{\ye}{\y*\len}
            \pgfmathsetmacro{\ze}{\z*\len}

            \draw[->,line width=0.7pt, color=cyan] 
              (\x,\y,\z) -- ++(\xe,\ye,\ze);
          \fi
        \fi
      }

    % Draw axes
\draw[->,thick] (0,0,0) -- (1.5,0,0) node[anchor=north east]{$x$};
\draw[->,thick] (0,0,0) -- (0,1.5,0) node[anchor=west]{$y$};
\draw[->,thick] (0,0,0) -- (0,0,1.5) node[anchor=south]{$z$};

\shade[ball color = red] (0,0) circle (0.05cm);
  \draw (0,0) circle (0.05cm);

\end{tikzpicture}
$$
