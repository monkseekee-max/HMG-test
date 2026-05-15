# Design System - HMG

## Product Context

- **What this is:** Official website for HMG, a branch-aware memory graph for autonomous coding agents.
- **Who it is for:** Developers and teams wiring long-running agents into real engineering workflows.
- **Project type:** Technical marketing site with product-like proof.

## Visual Thesis

HMG should feel like a trustworthy engineering instrument: dark memory graph, crisp type, restrained color, and visible proof of how the system thinks.

## Content Plan

- **Hero:** Brand, promise, CTA, and a full-bleed memory graph scene.
- **Language:** Chinese by default, with an explicit Chinese / English toggle in the header.
- **Problem:** Agents lose judgment, not just context.
- **Model:** Interactive memory lifecycle with agent brief, recall, correction, and handoff.
- **Trust:** Scope, repair, and audit as first-class product promises.
- **Quickstart:** Show the API loop directly.
- **Final CTA:** Push the user toward wiring HMG into an agent loop.

## Aesthetic Direction

- **Direction:** Industrial / utilitarian with editorial clarity.
- **Decoration level:** Intentional. The memory graph scene is the visual anchor; routine sections stay quiet.
- **Mood:** Serious, legible, and mechanism-forward. No generic AI glow.

## Typography

- **Display and body:** Geist, loaded from Google Fonts.
- **Code and metadata:** JetBrains Mono, loaded from Google Fonts.
- **Scale:** Large brand type in the hero, compact monospace labels for system state, readable paragraph text around 1rem to 1.2rem.

## Color

- **Approach:** Restrained base with operational accents.
- **Ink:** `#15161A`
- **Surface:** `#F7F8F5`
- **Strong surface:** `#EEF1EA`
- **Dark:** `#111318`
- **Primary green:** `#0E7C66`
- **Bright green:** `#15A37F`
- **Correction coral:** `#FF5A3D`
- **Link / active blue:** `#265CFF`
- **Warning gold:** `#F2C14E`

## Layout

- **Approach:** Full-bleed hero, then grid-disciplined sections.
- **Max content width:** `1180px`
- **Radius:** Small, mostly 4px to 8px. Large rounded SaaS chrome is avoided.
- **Cards:** Avoided except for repeated trust items and tool-like product surfaces.

## Motion

- **Approach:** Minimal-functional.
- **Current motions:** Hero graph scan animation, hover lift on buttons, tab state transition in the memory explorer.
- **Rule:** Motion should clarify product state or affordance. Decorative motion is removed.

## Decisions Log

| Date | Decision | Rationale |
| --- | --- | --- |
| 2026-05-15 | Use Leptos CSR with Trunk | Official Leptos docs recommend CSR + Trunk for snappy sites and static deployment. |
| 2026-05-15 | Use a full-bleed memory graph hero | HMG needs product mechanism as first impression, not an abstract AI illustration. |
| 2026-05-15 | Keep dependencies to Leptos only | First prototype should stay small and easy to reason about. |
| 2026-05-15 | Default to Chinese with English toggle | The site should serve Chinese readers first while preserving the original English positioning. |
