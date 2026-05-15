# Styling Considerations for Clickease

This document outlines the design and implementation strategy for the visual layer of Clickease, focusing on Neumorphic aesthetics, themeability, and performance-oriented animations for a desktop utility.

## 1. Neumorphic Design Implementation

Neumorphism (Soft UI) creates depth using contrasting shadows on elements that share the same background color as their parent.

### 1.1 Core Principles

- **Monochromatic base:** The element color and background color must be identical or very close.
- **Dual Shadows:**
  - **Light Shadow:** Top-left, lighter than the background.
  - **Dark Shadow:** Bottom-right, darker than the background.
- **Softness:** Large blur radii and subtle offsets.
- **Radius:** Large border-radii (e.g., `12px` to `24px`) to enhance the "molded" look.

### 1.2 Tailwind CSS Configuration

We will extend Tailwind to support Neumorphic utilities.

```javascript
// tailwind.config.js (conceptual)
module.exports = {
  theme: {
    extend: {
      boxShadow: {
        // Flat/Raised
        "nm-flat":
          "var(--nm-offset) var(--nm-offset) var(--nm-blur) var(--nm-shadow-dark), " +
          "calc(var(--nm-offset) * -1) calc(var(--nm-offset) * -1) var(--nm-blur) var(--nm-shadow-light)",
        // Inset/Pressed
        "nm-inset":
          "inset var(--nm-offset) var(--nm-offset) var(--nm-blur) var(--nm-shadow-dark), " +
          "inset calc(var(--nm-offset) * -1) calc(var(--nm-offset) * -1) var(--nm-blur) var(--nm-shadow-light)",
      },
      borderRadius: {
        nm: "20px",
      },
    },
  },
};
```

### 1.3 Accessibility Enhancements

Since Neumorphism relies on low-contrast shadows, we will:

- Add a subtle 1px border (`border-opacity-10`) for better edge definition.
- Ensure high-contrast typography and icons within Neumorphic containers.
- Provide a "High Contrast" toggle in settings that adds traditional borders/outlines.

---

## 2. Customizable Colors & Presets (OKLCH System)

To support dynamic presets while maintaining visual consistency, we will use the **OKLCH** color space. OKLCH allows us to adjust lightness and chroma predictably across different hues.

### 2.1 CSS Variable Strategy

```css
:root {
  /* Base OKLCH values for the theme */
  --theme-h: 250; /* Hue */
  --theme-c: 0.05; /* Chroma (low for soft look) */
  --theme-l: 95%; /* Lightness */

  /* Derived Colors */
  --bg-color: oklch(var(--theme-l) var(--theme-c) var(--theme-h));

  /* Neumorphic Shadow Calculations */
  --nm-shadow-light: oklch(
    calc(var(--theme-l) + 5%) var(--theme-c) var(--theme-h)
  );
  --nm-shadow-dark: oklch(
    calc(var(--theme-l) - 10%) var(--theme-c) var(--theme-h)
  );

  /* Offsets and Blurs */
  --nm-offset: 6px;
  --nm-blur: 12px;
}
```

### 2.2 Presets System

Presets will simply override the `--theme-h`, `--theme-c`, and `--theme-l` variables. This ensures that a "Blue" preset and a "Green" preset have the same perceived "softness" and depth.

---

## 3. Default Themes (System-Aware)

Clickease will automatically switch between themes based on system preferences using the `prefers-color-scheme` media query or a manual toggle.

### 3.1 Dark Mode: Dracula

Based on the popular Dracula palette, adapted for Neumorphism.

- **Background:** `#282a36` (OKLCH: `25% 0.04 268`)
- **Accent:** `#bd93f9` (Purple)
- **Shadow Dark:** `#191a21`
- **Shadow Light:** `#373a4b`

### 3.2 Light Mode: Catppuccin (Latte)

A soft, warm light theme.

- **Background:** `#eff1f5` (OKLCH: `95% 0.01 250`)
- **Accent:** `#1e66f5` (Blue)
- **Shadow Dark:** `#dce0e8`
- **Shadow Light:** `#ffffff`

---

## 4. Scaling, Transitions & Animations

For a desktop utility, animations must be snappy and provide immediate feedback.

### 4.1 Native CSS Transitions

We will use native CSS transitions and standard Web APIs to handle layout transitions and state changes, keeping the application lightweight.

- **Neumorphic Feedback:** When a button is clicked, it should transition from `shadow-nm-flat` to `shadow-nm-inset` using a `transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1)` property.
- **State Changes:** CSS classes (e.g., `.is-pressed`, `.is-active`) will be toggled via Vanilla TypeScript to trigger animations.
- **Layout Morphing:** Use CSS Grid and Flexbox with `transition` on properties like `grid-template-columns` or `opacity/transform` where supported.

### 4.2 Component Scaling

- **Hover States:** Subtle scaling (`scale(1.02)`) and shadow intensity increase using CSS `:hover` pseudo-classes.
- **Micro-interactions:** Icons should have slight rotations or color shifts when their parent card is hovered, implemented via CSS transitions on the `transform` property.

---

## 5. UI Impact: Resizing & System Tray

### 5.1 Responsive Strategy

Tauri windows can be resized by the user. We will implement:

- **Breakpoint-less Fluidity:** Using `clamp()` for font sizes and padding instead of strictly relying on `sm:`, `md:`, `lg:` breakpoints.
- **Compact Mode:** If the window width is < 400px, labels will hide, leaving only icons (side-bar becomes a bottom-bar or thin side-strip).

### 5.2 System Tray & Minimization

- **Mini-view:** When "minimized" to tray, clicking the icon should open a small, non-resizable "Quick Actions" pop-up near the tray area (using Tauri's `tauri-plugin-positioner`).
- **State Preservation:** The UI should immediately reflect the current active simulation state (e.g., a "Recording" glow) even in the mini-view.

### 5.3 Window Transitions

- **Fade-in/out:** Smooth opacity transitions when the window is shown/hidden.
- **Slide-up:** For the tray pop-up to mimic native OS feel (especially on macOS and Windows).
