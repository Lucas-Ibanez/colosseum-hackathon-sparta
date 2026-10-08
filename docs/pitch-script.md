# Hive pitch script (up to 3 min)

Script for the **pitch video** of the Colosseum submission: a startup pitch, not a demo
(team, problem, who it is for, validation and vision). The technical demo is a separate
video, cut from the continuous take ([`docs/demo-script.md`](demo-script.md#demo-técnica-corte-de-23-min)).
Format: slides with a voiceover. Decision: "Decisões humanas depois do D12a" in
[`docs/decisions.md`](decisions.md).

## Rules

- Spoken claims about what Hive does are **only** the ratified phrases (F1–F8,
  [`README.md`](../README.md#frozen-phrases-english-ratified-r-ui)), quoted as they are.
  F1 is said only while the slide shows the settlement link.
- Everything not built is said as future: "next", "we plan to", "roadmap". The vision slide
  is titled "Roadmap (not built yet)".
- Nothing about the team, users, validation, partners, numbers or business model that the
  team has not supplied. Every gap is a `TODO(…)` until the team fills it; an unfilled
  `TODO(validation)` is spoken as "no user validation yet", never skipped silently.
- Do not say the terms of the "Do not say" list of the `README.md`, nor "secure", "safe",
  "guaranteed", "certified", "verified agent", "reputation", "score", "trust level" or
  "approved" as a verdict.
- The official slogan is not rewritten or translated by us: `TODO(brand)` until the brand
  team supplies the English version.

## Script (target 2:55, about 350–400 spoken words)

| Part | Time | Slide |
| --- | ---: | --- |
| 1. Team | 0:00–0:15 | 1 |
| 2. Problem | 0:15–0:45 | 2 |
| 3. Who it is for | 0:45–1:05 | 3 |
| 4. What Hive does today | 1:05–2:00 | 4, 5 |
| 5. Validation | 2:00–2:15 | 6 |
| 6. Business model | 2:15–2:30 | 6 |
| 7. Vision | 2:30–2:50 | 7 |
| 8. Close | 2:50–2:55 | 7 |

**1. Team (0:15).**
> `TODO(team)`: names, roles and relevant background, in two or three sentences.

**2. Problem (0:30).**
> "AI coding agents are starting to take on delegated development tasks. When an agent
> delivers work, who decides whether it gets paid? Today, someone reviews the delivery by
> hand, or both sides rely on the platform in the middle to decide. The payment depends on
> a judgment that is hard to check afterwards."

**3. Who it is for (0:20).**
> "We are building for tech leads and partners at AI-native software houses that hand
> tasks to agents, and for operators of job platforms that pay for delegated work. They
> have bounded tasks, acceptance criteria that can be automated, and distinct parties on
> each side."

Source: ICP of `HIVE_MVP_UI_GUIDE.md` §2.

**4. What Hive does today (0:55).**
> "Hive fixes the acceptance criteria before any work starts and keeps the payment in an
> escrow on Solana devnet. A previously committed deterministic evaluator ran on the
> artifact delivered in the Job and produced the published verdict. The program moves the
> Test USDC only after checking the binding between journal, Job and delivery and
> verifying the proof."
>
> (slide 4, with the settlement link on screen) "Hive's Groth16 receipt is verified on
> devnet via CPI to the immutable Groth16 verifier of risc0-solana v3.0.0, in the same
> instruction that releases or refunds the Job's Test USDC."
>
> (slide 5) "The proof attests to the execution of the fixed rule on this artifact, not to
> the quality of a piece of software. The v1 rule is trivial (output = 2 × input) and
> serves to demonstrate the flow. Devnet and Test USDC only; none of this exists on
> mainnet."

These are F2, F1, F3 and F5, unchanged.

**5. Validation (0:15).**
> `TODO(validation)`: only real feedback from users or design partners, supplied by the
> team. If there is none by recording time, say: "We have not run user validation yet.
> Next, we plan to put this flow in front of design partners."

**6. Business model (0:15).**
> `TODO(business)`: how Hive plans to make money, written by the team. Say it as a plan
> ("we plan to…"), with no numbers that the team cannot back.

**7. Vision (0:20).**
> "Next, we plan to replace the trivial v1 rule with deterministic evaluators for real,
> bounded tasks, still on restricted artifacts. We also plan pilots that compare this
> zkVM evidence with simpler mechanisms, such as signed CI and human review, to learn how
> much evidence each kind of task needs."

Source: product guide, `docs/context/guia-mvp-agentes-de-codigo.md` §1 (product
learning) and §3 (v1 scope).

**8. Close (0:05).**
> "Hive." `TODO(brand)`: official English slogan.

## Slides (7)

Visual identity from [`DESIGN.md`](../DESIGN.md): Manrope for text, IBM Plex Mono only for
identifiers (signature, program ID, image ID); aubergine for structure, amber only for
brand details, never as a result color; flat surfaces. Logo from `brand/` exactly as it is
([`brand/MANIFEST.md`](../brand/MANIFEST.md)): `brand/logo/hive-horizontal-berinjela.svg` on
a light background, `brand/logo/hive-horizontal-branco.svg` on aubergine. These SVGs are
provisional (`TODO(brand)` until the vector master exists). No mascot (no file exists),
seal, shield, halo or honeycomb pattern.

| # | Title | Content |
| --- | --- | --- |
| 1 | Hive | horizontal logo; team names and roles (`TODO(team)`) |
| 2 | When an AI agent delivers work, who decides whether it gets paid? | the problem in two lines |
| 3 | Who it is for | AI-native software houses (tech leads, partners); job platform operators |
| 4 | How it works today | four steps: create and fund in one transaction; local proof (RISC Zero, then Groth16); `deliver` and settlement in one transaction with a CPI to the verifier; refund after the deadline. F1 in full next to the PASS settlement link [`2gB9djwM…`](https://explorer.solana.com/tx/2gB9djwM3WpApHLXYf5riF974mTK5W4UW7HCv1xJamLPu3NEwnPcCY4oT6zAwvbMkaKRcATf1LCQPyXP8rmpj3o?cluster=devnet) (99,541 CU in the verifier) |
| 5 | What the proof does not show | F3; F5; F4 ("The escrow and the verifier are immutable (upgrade authority `none`). No one, not even the project, changes the rules or decides the payment; there is also no e-stop.") |
| 6 | Validation and business model | `TODO(validation)`; `TODO(business)` |
| 7 | Roadmap (not built yet) | the two "next" items of part 7; close with the logo and `TODO(brand)` |

## Before recording

- Fill every `TODO(…)` above, or say the gap as written in the rules.
- Check the time limit and the format required by the current edition of the hackathon
  (`TODO(form)` in [`docs/submission.md`](submission.md)).
- Open the settlement link of slide 4 once and confirm it loads on devnet.
