# YouTube Transcript Tool - UI Redesign Design Document

**Date**: 2026-03-16
**Status**: Approved
**Target**: YouTube Transcript Web Application

## Design Philosophy

"Invisible Tools" — The interface should let users focus on content, not the UI itself. Inspired by Linear and Vercel's design language.

## Target Audience

- Developers/Technical users — Need fast, efficient data extraction
- Content creators/Researchers — Use repeatedly, value readability
- General users — Occasional use, need clear guidance

## Visual System

### Color Palette

```css
/* Primary - Blue Sky */
--primary: #0EA5E9 (Sky 500)
--primary-hover: #0284C7 (Sky 600)

/* Accent - Violet */
--accent: #8B5CF6 (Violet 500)
--accent-hover: #7C3AED (Violet 600)

/* Background */
--bg-light: #FFFFFF
--bg-dark: #0A0A0A

/* Text */
--text-primary: #0A0A0A
--text-secondary: #737373 (Neutral 500)
--text-light: #E5E5E5

/* Border */
--border-light: #E5E5E5
--border-dark: #262626
```

### Typography

| Usage | Font | Characteristics |
|-------|------|-----------------|
| Headings | Inter Tight | Tighter tracking, more modern |
| Body | Inter | Readable, neutral |
| Code/Time | JetBrains Mono | Monospace, technical |

### Spacing & Radius

- **Border Radius**: 6px (sharper, not rounded 8px)
- **Base Unit**: 4px
- **Scale**: 4, 8, 12, 16, 24, 32, 48, 64px

### Shadows

Remove decorative shadows. Use only depth-essential subtle shadows.

---

## Layout Restructuring

### Hero Section

**Current Issues**:
- Centered alignment feels template-y
- Emoji decoration unprofessional
- Generic spacing

**Changes**:
- Left-align everything
- Large, tight-spacing heading
- Single-line description in gray
- Remove all decorations

```tsx
// Before
<div className="text-center space-y-4">
  <h2 className="text-3xl font-bold">{t("header.title")}</h2>
  <p className="text-muted-foreground max-w-2xl mx-auto">{t("meta.description")}</p>
</div>

// After
<div className="space-y-2">
  <h1 className="text-4xl font-semibold tracking-tight">
    YouTube Transcript
  </h1>
  <p className="text-sm text-neutral-500 max-w-lg">
    Extract, translate, and summarize video transcripts instantly.
  </p>
</div>
```

### Input Area

**Changes**:
- Full-width (remove max-width constraint)
- Remove decorative icon inside input
- Cleaner placeholder text
- Subtle border on focus

### Card System

**Current Issues**:
- Equal-sized grid feels generic
- Icons before every heading
- Too much padding

**Changes**:
- Transcript card: full width
- Summary/Translation: stacked vertically
- Remove all heading icons
- Reduce padding
- Use borders instead of shadows for separation

---

## Interaction Enhancements

### Loading State - Skeleton Screen

```tsx
<SkeletonCard>
  <SkeletonHeader />
  <SkeletonLines count={5} />
</SkeletonCard>
```

- Subtle pulse animation
- Matches actual content structure

### Copy Feedback

**Current Problem**: Single `copied` state conflicts across buttons

**Solution**:
```tsx
const [copiedId, setCopiedId] = useState<string | null>(null)

<Button onClick={() => handleCopy(id)}>
  {copiedId === id ? <CheckIcon /> : <CopyIcon />}
</Button>
```

- Icon swap only (no text change)
- Prevents layout shift
- Independent state per button

### Transitions

| Element | Transition | Duration |
|---------|-----------|----------|
| Page enter | Fade + Slide up | 300ms |
| Card expand | Height | 200ms |
| Button hover | Background | 100ms |
| Input focus | Border + Ring | 150ms |

Stagger children animations by 50ms for polished feel.

---

## Feature Completions

### Empty State

Before user enters URL, show:

```
┌────────────────────────────────────────────────────┐
│                                                    │
│   Try an example video:                            │
│                                                    │
│   ┌──────────────────────────────────────────┐    │
│   │ https://youtube.com/watch?v=dQw4w9WgXcQ  │    │
│   └──────────────────────────────────────────┘    │
│                                                    │
│            [ Click to fill ]                       │
│                                                    │
└────────────────────────────────────────────────────┘
```

### Error Messages

- Red border on input
- Error message below in small red text
- Hint message in smaller gray text

### Mobile Optimizations

- Touch targets: minimum 44x44px
- Language switch: underline link style (larger hit area)
- Responsive spacing: reduce padding on small screens
- Stack all cards vertically on mobile

---

## Component Changes

| Component | Current | After |
|-----------|---------|-------|
| Logo | 📺 + text | SVG icon or text only |
| Lang Switch | Rounded pills | Underline links |
| Transcript List | Flex items | Table layout |
| Copy Button | Text change | Icon swap + tooltip |
| Icons | Every heading | Removed |

---

## Accessibility Fixes

### Critical

1. **Language Switcher** - Add `aria-label` and `aria-current`
2. **Decorative Icons** - Add `aria-hidden="true"`
3. **Error Messages** - Add `role="alert"` and `aria-live`

### High Priority

4. **Touch Targets** - Ensure 44x44px minimum
5. **Focus Indicators** - Maintain visible ring on all interactive elements
6. **Error Handling** - Add try-catch with UI feedback

---

## Implementation Priority

1. Visual System (colors, typography, spacing)
2. Layout (hero, input, cards)
3. Interactions (skeleton, copy, transitions)
4. Features (empty state, errors, mobile)
5. Accessibility (ARIA labels, contrast)

---

## Success Criteria

- [ ] No emoji in production code
- [ ] All ARIA labels present
- [ ] Touch targets ≥ 44x44px
- [ ] Empty state implemented
- [ ] Skeleton loading implemented
- [ ] Independent copy button states
- [ ] Mobile responsive at 375px breakpoint
- [ ] WCAG AA contrast ratios met
