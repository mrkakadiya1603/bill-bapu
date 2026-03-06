# RestroSync — Design System

## Design Direction: "Warm Industrial"

A design language that blends **warm, appetizing tones** with **clean industrial precision**. Think of a modern Indian bistro — exposed brick warmth meets clean steel counters. The UI should feel like a tool built by craftsmen for craftsmen: warm enough to invite, sharp enough to perform.

---

## 1. Design Philosophy

### Core Principles

1. **Speed is Visual** — UI should *feel* fast. Micro-animations are snappy (150ms max), transitions are purposeful, loading states are skeletal. No spinners, no blocking modals for routine operations.

2. **Glanceable** — A cashier mid-rush should understand the screen state in under 1 second. Use color-coded status, large touch targets, and clear visual hierarchy.

3. **Warm, Not Sterile** — Restaurant software shouldn't look like hospital software. We use warm neutrals and accent colors inspired by Indian spices and cuisine.

4. **Density Without Clutter** — Restaurant staff need information-dense screens (orders, tables, bills) but the layout must breathe. Use cards, subtle dividers, and whitespace strategically.

5. **Device-Adaptive** — The same design language scales from a 5" waiter phone to a 24" cashier monitor. Components adapt, not just shrink.

---

## 2. Color Palette

### Brand Colors

```
Primary:        #E85D2C  (Tandoori Orange — warm, energetic, food-associated)
Primary Dark:   #C44A1E  (Roasted Orange — hover/active states)
Primary Light:  #FFF0EB  (Peach Whisper — subtle backgrounds, highlights)
```

### Secondary Colors

```
Secondary:      #1B2D45  (Deep Navy — authority, trust, used for headers/nav)
Secondary Light:#2A4365  (Slate Blue — secondary buttons, less emphasis)
Accent:         #F4A623  (Turmeric Gold — highlights, badges, premium features)
Accent Alt:     #2D9F6F  (Mint Chutney — success, positive states, profit indicators)
```

### Neutral Palette

```
Background:     #FAFAF8  (Warm White — main canvas, not pure white)
Surface:        #FFFFFF  (Pure White — cards, modals, elevated surfaces)
Surface Alt:    #F5F3EF  (Warm Gray — alternating rows, secondary backgrounds)
Border:         #E8E4DD  (Sand — subtle borders, dividers)
Border Strong:  #C9C3B8  (Stone — input borders, stronger dividers)

Text Primary:   #1A1A18  (Near Black — primary text, warm undertone)
Text Secondary: #6B6560  (Warm Gray — labels, secondary info)
Text Tertiary:  #9C9590  (Light Warm Gray — placeholders, disabled)
Text Inverse:   #FAFAF8  (Warm White — text on dark backgrounds)
```

### Semantic Colors (Status)

```
Success:        #2D9F6F  (Mint Chutney — paid, completed, in-stock)
Success Bg:     #EDFAF3  (Mint Wash)

Warning:        #E8A817  (Mustard — pending, low stock, attention needed)
Warning Bg:     #FFF8E6  (Cream)

Error:          #D93B3B  (Red Chili — failed, overdue, out of stock)
Error Bg:       #FEF0F0  (Rose Wash)

Info:           #3B82C4  (Sky — informational, neutral alerts)
Info Bg:        #EFF6FF  (Ice)
```

### Order/Table Status Colors (Specific to Restaurant Context)

```
Table Available:    #2D9F6F  (Green)
Table Occupied:     #E85D2C  (Orange)
Table Reserved:     #3B82C4  (Blue)
Table Billing:      #E8A817  (Yellow)

KOT New:            #E85D2C  (Orange — needs attention)
KOT Preparing:      #E8A817  (Yellow — in progress)
KOT Ready:          #2D9F6F  (Green — ready to serve)
KOT Delayed:        #D93B3B  (Red — overdue)
```

### Dark Mode (Phase 2 — Kitchen Display Optimized)

```
Background:     #121212
Surface:        #1E1E1E
Surface Alt:    #2A2A2A
Border:         #3A3A3A
Text Primary:   #E8E4DD
Text Secondary: #9C9590
Primary:        #F07040  (Slightly lighter orange for dark backgrounds)
```

---

## 3. Typography

### Font Stack

```
Primary:        "Inter" (UI text — clean, legible, excellent at small sizes)
Monospace:      "JetBrains Mono" (bill numbers, prices, order IDs, receipts)
Display:        "Plus Jakarta Sans" (headings, brand moments — slightly more personality)
```

### Type Scale

```
Display:        32px / 40px line-height / 700 weight    — Page titles (rare)
Heading 1:      24px / 32px line-height / 600 weight    — Section headers
Heading 2:      20px / 28px line-height / 600 weight    — Card titles
Heading 3:      16px / 24px line-height / 600 weight    — Sub-sections
Body:           14px / 22px line-height / 400 weight    — Default text
Body Small:     13px / 20px line-height / 400 weight    — Dense tables
Caption:        12px / 18px line-height / 400 weight    — Labels, timestamps
Overline:       11px / 16px line-height / 600 weight    — Category labels (uppercase, tracked)

Price/Amount:   Use monospace, 600 weight, slightly larger than surrounding text
Bill Total:     24px–32px monospace, 700 weight
```

### Typography Rules

- Never go below 12px — restaurant staff read screens from arm's length
- Prices and amounts always use monospace (alignment matters)
- Use sentence case for UI labels, not Title Case or ALL CAPS (except overlines)
- Line lengths should not exceed 65 characters for readability
- Bold (600) for emphasis, not italic (italic is hard to read at speed)

---

## 4. Spacing & Layout

### Spacing Scale (Base 4px)

```
4xs:    2px     — Tight internal gaps
3xs:    4px     — Icon-to-text gaps, tight padding
2xs:    8px     — Compact element spacing
xs:     12px    — Default element gaps
sm:     16px    — Card padding, section gaps
md:     24px    — Between content blocks
lg:     32px    — Major section separation
xl:     48px    — Page-level spacing
2xl:    64px    — Hero/display spacing
```

### Layout System

```
Grid:           12-column grid on desktop, 4-column on mobile
Gutter:         16px (mobile), 24px (desktop)
Max Content:    1440px (desktop app is flexible, web app is constrained)
Sidebar Width:  240px collapsed-capable to 64px (icon-only)
```

### Breakpoints

```
Mobile:         < 640px     (Waiter phone)
Tablet:         640–1024px  (Kitchen display, small tablets)
Desktop:        1024–1440px (Cashier PC)
Wide:           > 1440px    (Large monitors, multi-panel views)
```

---

## 5. Components

### 5.1 Buttons

**Variants:**

| Variant | Use Case | Style |
|---|---|---|
| Primary | Main actions (Place Order, Save, Pay) | Filled, `#E85D2C`, white text |
| Secondary | Secondary actions (Cancel, Back) | Outlined, `#1B2D45` border |
| Ghost | Tertiary actions (filters, toggles) | Text only, no border |
| Danger | Destructive actions (Delete, Void) | Filled, `#D93B3B` |
| Success | Positive confirmations (Settle, Confirm) | Filled, `#2D9F6F` |

**Sizes:**

| Size | Height | Padding | Font | Use Case |
|---|---|---|---|---|
| Large | 48px | 24px horizontal | 16px/600 | Touch-primary (waiter, kitchen) |
| Default | 40px | 16px horizontal | 14px/500 | Desktop default |
| Small | 32px | 12px horizontal | 13px/500 | Dense UI, table cells |
| Icon | 36px x 36px | — | — | Icon-only actions |

**States:**
- Default → Hover (darken 8%) → Active (darken 15%) → Disabled (40% opacity)
- Focus ring: 2px offset, `#E85D2C` with 30% opacity glow
- All buttons have `border-radius: 8px`

### 5.2 Cards

The primary container for content blocks (orders, menu items, table tiles, reports).

```
Background:     #FFFFFF
Border:         1px solid #E8E4DD
Border Radius:  12px
Shadow:         0 1px 3px rgba(26, 26, 24, 0.06)
Shadow Hover:   0 4px 12px rgba(26, 26, 24, 0.1)
Padding:        16px (default), 12px (compact/mobile)
```

**Card Variants:**
- **Flat Card** — No shadow, border only. For dense lists.
- **Elevated Card** — Shadow + border. For standalone content (order summary, table tile).
- **Interactive Card** — Hover shadow + subtle scale (1.01). For clickable items (menu items, tables).
- **Status Card** — Left border accent (4px) colored by status. For orders, KOTs.

### 5.3 Inputs

```
Height:         40px (default), 48px (touch)
Border:         1px solid #C9C3B8
Border Radius:  8px
Focus Border:   #E85D2C
Focus Ring:     0 0 0 3px rgba(232, 93, 44, 0.15)
Background:     #FFFFFF
Padding:        12px horizontal
Font:           14px/400
Label:          12px/600, #6B6560, positioned above with 4px gap
Error:          12px, #D93B3B, below input with 4px gap
```

**Input Variants:**
- Text Input (default)
- Number Input (with +/- stepper for quantities)
- Search Input (with search icon, clearable)
- Select / Dropdown
- Date Picker
- Amount Input (monospace, right-aligned, currency prefix)

### 5.4 Tables (Data)

```
Header:         Background #F5F3EF, font 12px/600 uppercase, text #6B6560
Row Height:     44px (default), 36px (compact)
Row Hover:      Background #FAFAF8
Row Alternate:  Background #FAFAF8 (if enabled)
Border:         Bottom border only, 1px #E8E4DD
Cell Padding:   12px horizontal
```

- Numeric columns right-aligned
- Status columns use colored badges
- Action columns pinned to the right
- Sortable columns show arrow indicators

### 5.5 Badges & Tags

```
Border Radius:  6px
Padding:        4px 8px
Font:           12px/600
```

| Badge Type | Background | Text |
|---|---|---|
| Success | `#EDFAF3` | `#1A7F54` |
| Warning | `#FFF8E6` | `#9A6F00` |
| Error | `#FEF0F0` | `#B92B2B` |
| Info | `#EFF6FF` | `#2563A8` |
| Neutral | `#F5F3EF` | `#6B6560` |
| Primary | `#FFF0EB` | `#C44A1E` |

### 5.6 Navigation

**Sidebar (Desktop — Cashier PC):**
```
Width:          240px (expanded), 64px (collapsed)
Background:     #1B2D45 (Deep Navy)
Text:           #C8D6E5 (inactive), #FFFFFF (active)
Active Item:    Background rgba(232, 93, 44, 0.15), left border 3px #E85D2C
Hover:          Background rgba(255, 255, 255, 0.06)
Icon Size:      20px
Item Height:    44px
Section Label:  11px/600, uppercase, #7B8FA3
```

**Bottom Tab Bar (Waiter Phone):**
```
Height:         56px + safe area
Background:     #FFFFFF
Border Top:     1px solid #E8E4DD
Active:         #E85D2C icon + label
Inactive:       #9C9590 icon + label
Icon Size:      24px
Label:          11px/500
```

### 5.7 Modals & Overlays

```
Overlay:        rgba(26, 26, 24, 0.5), backdrop-blur: 4px
Modal:          Background #FFFFFF, border-radius 16px
                Shadow: 0 20px 60px rgba(26, 26, 24, 0.2)
                Max width: 480px (small), 640px (medium), 960px (large)
                Padding: 24px
Header:         20px/600 title, close button top-right
Footer:         Right-aligned buttons, separated by 12px gap
```

### 5.8 Toast Notifications

```
Position:       Top-right on desktop, top-center on mobile
Border Radius:  10px
Shadow:         0 8px 24px rgba(26, 26, 24, 0.12)
Padding:        12px 16px
Max Width:      400px
Duration:       3s (info), 5s (error), persistent (action required)
Animation:      Slide in from right (desktop) / top (mobile), 200ms ease-out
```

### 5.9 Table Floor Map Tiles

Special component for the Table Management screen.

```
Tile Size:      Variable (snaps to grid), min 80x80px
Border Radius:  12px
Font:           Table number in 18px/700 centered
Status:         Full background fill with status color (subtle, 15% opacity) + status dot
Hover:          Elevate with shadow
Touch Target:   Entire tile is tappable
```

---

## 6. Iconography

### Style
- **Line icons** as default (1.5px stroke, rounded caps)
- **Filled icons** for active/selected states in navigation
- Consistent 24px canvas, 20px default display size
- Use a single icon library: **Lucide Icons** (open source, consistent, React-ready)

### Custom Icons (if needed)
- KOT (Kitchen Order Ticket)
- Table layouts
- Connectivity status (Cloud/WiFi/Standalone)
- Indian Rupee (₹) symbol

---

## 7. Motion & Animation

### Principles
- Animations serve function, not decoration
- Maximum duration: 200ms for micro-interactions, 300ms for transitions
- Easing: `ease-out` for entrances, `ease-in` for exits, `ease-in-out` for morphs

### Specific Animations

| Element | Animation | Duration | Easing |
|---|---|---|---|
| Page transitions | Fade + subtle slide (8px) | 200ms | ease-out |
| Modal entrance | Scale from 0.95 + fade | 200ms | ease-out |
| Card hover | Shadow expand + translateY(-1px) | 150ms | ease-out |
| Toast enter | Slide from edge + fade | 200ms | ease-out |
| Toast exit | Fade out | 150ms | ease-in |
| Sidebar collapse | Width morph | 200ms | ease-in-out |
| Button press | Scale to 0.97 | 100ms | ease-out |
| Status change | Background color crossfade | 300ms | ease-in-out |
| Loading skeleton | Shimmer pulse | 1.5s loop | ease-in-out |
| New order flash | Brief highlight pulse (orange glow) | 600ms | ease-out |

---

## 8. Screen-Specific Design Notes

### 8.1 Cashier / Billing Screen (Desktop — Primary Screen)
- **Split layout**: Left 60% = order items list, Right 40% = bill summary + payment
- Dense but readable — every pixel matters
- Quick-access numpad for quantities
- Large "Settle Bill" button (always visible, bottom-right)
- Running total always visible in a sticky header/footer

### 8.2 Table Floor Map (Desktop + Tablet)
- Grid of table tiles, color-coded by status
- Drag-to-arrange in edit mode
- Click/tap a table → opens order panel (slide-in from right)
- Real-time status updates via WebSocket (tile color change with pulse animation)

### 8.3 Kitchen Display (Tablet/Monitor — Dark Mode Preferred)
- **Large cards** with high contrast — readable from 6 feet away
- Order cards flow left-to-right or top-to-bottom
- Prominent timer on each KOT showing elapsed time
- Color transitions: Orange → Yellow → Green (or Red if delayed)
- Sound/vibration alert for new orders

### 8.4 Waiter Phone (PWA — Mobile-First)
- Bottom tab navigation: Tables | Orders | Menu | Notifications
- Table grid as home screen — tap to take order
- Menu browsing with search + category filter
- Large touch targets (min 44px), thumb-friendly bottom actions
- Offline indicator banner (subtle, not blocking)

### 8.5 Reports / Analytics (Desktop + Web)
- Clean data visualization with the brand color palette
- Card-based KPI tiles at the top (revenue, orders, avg ticket)
- Charts use the spice palette: Orange, Gold, Mint, Navy
- Print-optimized layouts for daily summary reports

---

## 9. Responsive Strategy

| Device | Primary Layout | Navigation | Input Mode |
|---|---|---|---|
| Cashier PC (1024px+) | Multi-panel, sidebar nav | Sidebar + keyboard shortcuts | Keyboard + mouse |
| Kitchen Tablet (640–1024px) | Single column, cards | Minimal top bar | Touch |
| Waiter Phone (<640px) | Single column, stacked | Bottom tab bar | Touch |
| Owner Web (1024px+) | Dashboard layout, sidebar | Sidebar | Mouse |

---

## 10. Accessibility

- All interactive elements: min 44px touch target on touch devices
- Color contrast ratio: minimum 4.5:1 for text, 3:1 for UI elements
- Never use color alone to convey status — always pair with icon or text label
- Keyboard navigable: all actions reachable via Tab + Enter/Space
- Focus indicators always visible (never `outline: none` without replacement)
- Support system font-size scaling up to 150%

---

## 11. Design Tokens (Implementation Reference)

All values above should be implemented as design tokens (CSS custom properties / Tailwind config / theme object) so they can be:
- Referenced consistently across Desktop (Tauri), Web (Next.js), and PWA
- Swapped for dark mode
- Adjusted for white-label / restaurant branding (future Phase 3 feature)

### Token Naming Convention

```
--rs-color-primary
--rs-color-primary-dark
--rs-color-bg
--rs-color-surface
--rs-color-text-primary
--rs-color-status-success
--rs-spacing-sm
--rs-radius-card
--rs-shadow-card
--rs-font-body
--rs-font-display
```

Prefix: `rs-` (RestroSync)

---

## 12. File / Asset Organization

```
packages/ui/
  src/
    tokens/
      colors.ts
      typography.ts
      spacing.ts
      shadows.ts
    components/
      Button/
      Card/
      Input/
      Badge/
      Modal/
      Toast/
      Table/
      Navigation/
    layouts/
      CashierLayout.tsx
      WaiterLayout.tsx
      KitchenLayout.tsx
      DashboardLayout.tsx
    hooks/
      useTheme.ts
      useBreakpoint.ts
    styles/
      global.css
      reset.css
```

This shared `packages/ui` package will be consumed by both the Tauri desktop app and the Next.js web app, ensuring visual consistency across all surfaces.
