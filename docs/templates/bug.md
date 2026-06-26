# Bug Task

**Owner:** Software Engineer (or Designer if purely visual) · **Branch:** `fix(<module>)/<summary>` · **Commit:** `fix(<module>): ...`

> Reading this cold? `ARCHITECTURE.md` tells you which side owns what. Most behavior bugs are
> server (`tick.js`); most visual bugs are a sprite `draw()`.

## Report
- **Symptom:** _what the player sees_
- **Repro steps:** _1… 2… 3… (exact, two tabs if multiplayer)_
- **Expected vs actual:**
- **First seen / suspected area:** _e.g. after `reset`, bullets from the previous round persist_

## Investigate
- Reproduce **first** and write down the exact steps.
- Behavior/desync bug → server is authoritative; check `tick.js` purity and the event list.
- Visual-only bug → the relevant `objects/*.rs` `draw()`; no logic change needed.
- Timing/position drift → check `clock_offset` and the position formulas (client mirrors server).

## Definition of done
- [ ] Root cause identified (one line in the PR), not just symptom patched.
- [ ] Bug no longer reproduces with the original steps — described in the PR.
- [ ] No regression to: scoring, reset, win-at-5, object interactions, heartbeat resync.
- [ ] Relevant formatter/lint clean (`cargo fmt` / `pnpm exec biome check`).
- [ ] Regression note in the PR (and a test once a harness exists).
