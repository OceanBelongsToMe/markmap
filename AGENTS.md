# AGENTS.md

## Purpose

Agents in this repository optimize for:

- Understandability
- Change isolation
- Local reasoning

Rule compliance is secondary.

## Core Principle

Single Responsibility Principle (SRP):

A module should have only one reason to change.

All other principles are derived consequences.

## Principle Derivation Order

SRP → Cohesion/Separation → Layering → ISP → DIP → OCP → LSP  
(KISS / YAGNI act as brakes)

## Conflict Resolution Priority

1. Human understandability
2. Change isolation
3. Structural cost
4. Principle purity

Violating a principle is acceptable if intentional and justified.

## Agent Expectations

- Cite concrete evidence
- Prefer minimal, reversible refactors
- Explain trade-offs
