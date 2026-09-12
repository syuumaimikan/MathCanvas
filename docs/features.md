---
layout: default
title: Killer Features
---

# MathCanvas: Next-Generation Killer Features

MathCanvas differentiates itself from existing solutions (Apple Math Notes, GeoGebra, Desmos) through deep integration of handwriting, symbolic logic, and reactive visualization.

## 1. Dynamic Interactions (Canvas-Math Fusion)
- **Smart Scrubber (Variable Scrubber)**: Drag numbers inside equations (e.g., $y = ax^2$, scrubbing $a$) to instantly warp linked graphs and results. Includes inline mini-sliders `[ a: 0 ━●━ 10 ]`.
- **Visual Linking (Color Tracking)**: Terms in equations (e.g., $+3x$ in red) automatically match colors with corresponding tangents, slopes, or vector fields on graphs.

## 2. Geometry & Physics Simulations
- **Dynamic Geometry**: Draw rough shapes (triangles/circles) that convert to exact objects. Dragging vertices live-updates angles, areas, and centroid formulas.
- **Interactive Vector Fields & Topology**: Input differential equations ($\frac{dy}{dx} = x-y$) to project a vector field. Tapping the canvas dynamically traces solution curves.
- **Physics Sandbox**: Draw rigid bodies, springs, and pendulums. Link gravity $g$ and friction $\mu$ directly to formulas to execute visual simulations.

## 3. Educational & Curriculum Assist (Japan Specialized)
- **High School & University Presets**:
  - Math I/A: Discriminants, Cosine Rule, Modular Arithmetic.
  - Math II/B: Logarithm laws, Recurrence relations, Vectors.
  - Math III/C: Limits, Integration by parts, Complex plane mapping, Matrices.
- **Step-by-Step Solver**: Accordion-style logical breakdowns explaining factorizations and quadratic formulas in natural Japanese.
- **Math Linter (Error Detection)**: Underlines logic breaks in real-time (e.g., squaring both sides losing equivalence, potential zero divisions).

## 4. Engineering & Research Utilities
- **Code Generation & Export**: Convert equations instantly to Rust (nalgebra/ndarray), Python (NumPy/SymPy), C/GLSL, or LaTeX.
- **Real-Time Data Regression**: Paste CSVs to auto-plot scatter graphs and apply Least-Squares fitting across custom nonlinear models.
- **Boolean Algebra & Discrete Math**: Build truth tables, Karnaugh maps, and run Quine-McCluskey simplifications via logic expressions (AND, OR, XOR, $\to$).
- **Dimensional Analysis**: Instantly red-squiggles inconsistent unit additions (e.g., $[m] + [s]$).
- **Error Propagation**: Auto-calculates standard uncertainty combining derivatives.

## 5. Advanced Handwriting UX
- **Gesture Erase**: Scribble out terms to neatly delete them from the AST.
- **Lasso Quick-Actions**: Circle an area to pop up: "Copy to LaTeX", "Graph it", "Integrate", "Factorize".
- **Auto-Solve Trigger ("=")**: Writing `=` at the end of a stroke auto-computes and faintly renders the simplified answer; tap to lock it in.
- **Strike-through Simplification**: Slashing matching terms in numerators/denominators instantly crosses them out and spawns the simplified fraction.
- **Smart Annotations**: Drawing an arrow $\to$ to a graph vertex automatically pops out extreme values and derivative meanings.
- **Stroke Time-Lapse**: Replay thought processes and derivation steps via slider or export as MP4/GIF.

## 6. Advanced Visualization
- **Implicit Curve Shading**: High-performance translucent shading for inequalities (e.g., $x^2 + y^2 \le 4$).
- **Parametric Curve Tracing**: Press "Play" on time $t$ to watch particles move along $x=f(t), y=g(t)$ with velocity/acceleration vectors extending in real time.
- **3D Cross-Section Slicer**: Slide a plane across a 3D surface $z=f(x,y)$ to extract live 2D contours.
- **Complex Conformal Mapping**: Map input grids across complex functions $w=f(z)$ to see topological distortion.
- **Calculus Visualizations**: Dynamically render Riemann Sums (left, right, midpoint, trapezoid) and Taylor Series animations clinging to parent curves as $N$ increases.

## 7. Collaborative & Productivity Features
- **Split-View Scratchpad**: Slide out a calculation sheet, solve an intermediate value, and drag only the final answer into the main document.
- **Mask Mode for Worksheets**: Click terms to blank them out for student test-generation (exports to PDF).
- **Math Diff**: Ghost-overlay previous graphs/results when tweaking parameters to see exactly what changed.
- **Markdown Hybridization**: Seamlessly mix rich text (headers, lists, callouts) with display math and freehand canvases.
