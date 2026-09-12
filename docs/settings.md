---
layout: default
title: Settings & Configuration
---

# MathCanvas Settings Architecture

As a commercial-grade workspace, MathCanvas offers deep customizability across all calculation, rendering, and interaction vectors. 

## 1. Math & Calculation Engine
- **Angle Unit**: Radians (`rad`), Degrees (`deg`), Gradians (`grad`)
- **Calculation Mode**: 
  - **Exact/Algebraic**: Prioritizes fractions, surds, and CAS symbolic outputs.
  - **Numeric/Approximate**: Fast floating-point execution.
- **Complex Mode**: 
  - **Real Only**: Roots of negative numbers yield errors.
  - **Complex Auto**: Enables complex domains.
  - **Imaginary Unit Symbol**: `i` (Math Standard) vs `j` (Engineering/Electrical Standard).
- **Precision & Rounding**:
  - Significant Digits (e.g., 3-15)
  - Fixed Decimals
  - Rounding Mode (Nearest, Truncate, Ceil, Floor, Half-Even)
  - **Arbitrary Precision (BigNum)**: Bit limits for internal representation.

## 2. Pen, Input & Stroke Settings
- **Pen Mode**:
  - **Stylus Only**: Finger handles pan/zoom exclusively.
  - **Hybrid**: Finger drawing allowed.
- **Sensitivity & Palm Rejection**: Low, Medium, High, Off.
- **Handwriting Recognition Trigger**:
  - **Real-Time**: Converts automatically after pen lift + delay.
  - **Explicit**: Triggers only on `=` or lasso gesture.
  - **Manual**: Lasso + "Convert" button.
- **Stroke Smoothing**: None, Weak, Strong (Bezier curve optimization).
- **Auto-Shape Recognition**: Cleans up freehand circles, lines, rectangles, and arrows.

## 3. Graph & Rendering Engine
- **Quality & Sampling**:
  - Point count (500 to 5000+ points).
  - **Adaptive Sampling**: Dynamically subdivides dense/discontinuous intervals.
- **Hardware Acceleration**: Enable/Disable `wgpu`/Vulkan rendering.
- **Coordinate System**: Cartesian, Polar, Logarithmic. Grid snapping toggles.
- **Singularity Detection**: Automatic rendering of vertical asymptotes as dashed lines.

## 4. Localization & Display
- **Decimal Separator**: `1,234.56` (US/Japan) vs `1.234,56` (Europe/ISO).
- **Multiplication Symbol**: $\times$, $\cdot$, or implicit.
- **Division Symbol**: $\div$, $/$, or fractions ($\frac{a}{b}$).
- **Typography**: Select fonts for Math and UI (e.g., Noto Sans JP, BIZ UDPGothic).

## 5. Notebook & Canvas Behavior
- **Reactive DAG Mode**:
  - **Immediate**: Background recalc on every change.
  - **Manual**: Batch execution for heavy notebooks.
- **Auto-Save & History**: Save interval (Immediate, 30s, 1m). Undo stack limit (50-500).
- **Crash Recovery**: Snapshot interval.

## 6. Platform Specific
- **Windows**: Windows Ink API toggle, GPU preference (Discrete vs Integrated).
- **Android**: Refresh rate sync (60Hz, 90Hz, 120Hz), Stylus side-button mappings (Eraser, Lasso, Undo).
