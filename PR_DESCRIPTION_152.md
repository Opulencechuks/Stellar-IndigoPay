# PR Description: automated integration test suite for contract upgrade lifecycle and indexer compatibility

## Summary
This PR builds and integrates a robust integration test suite validating the full Soroban contract upgrade lifecycle from v1 to v2 in a network-fork/mock-host environment. It ensures state backward compatibility, event emission continuity, and backend indexer compatibility across upgrades.

## Details of Changes

### 1. Smart Contract & Workspace Configuration
- **V1 Feature Flag**: Introduced a `v1` feature flag in `contracts/indigopay-contract/Cargo.toml` to conditionally exclude the appended `paused: bool` field from the `Project` struct in `lib.rs`, modeling the exact structural transition during upgrade validation.
- **DataKey Enum Extension**: Appended `NewFeature` to the end of the `DataKey` enum to guarantee backward compatibility with existing storage variants.
- **Cargo Workspace Updates**: Added `indigopay-contract/tests/v2_fixture` to the workspace members in `contracts/Cargo.toml`.

### 2. Upgraded V2 Fixture
- Created `contracts/indigopay-contract/tests/v2_fixture.rs` (and its crate configuration under `tests/v2_fixture/Cargo.toml`) that wraps the main contract and implements new v2-specific functions (`new_v2_function` and `get_new_feature_val`) to simulate post-upgrade features.

### 3. Rust Upgrade Integration Test Suite
- Created `contracts/tests/upgrade_test.rs` which performs the full integration lifecycle:
  1. Deploys the V1 WASM fixture (without `paused` field).
  2. Seeds synthetic data (project registrations, donations, governance proposals, and votes).
  3. Proposes the upgrade and verifies cancellation/re-proposal flows.
  4. Asserts that `execute_upgrade` fails before the 34,560 ledger timelock delay.
  5. Advances the ledger past the timelock and executes the upgrade.
  6. Verifies state continuity (asserts raises, voter list, badges, and that `paused` correctly defaults to `false` on the pre-existing v1 project data layout).
  7. Invokes the newly introduced V2 functions.

### 4. Backend Indexer Compatibility Tests
- Created `backend/__tests__/contractUpgrade.test.js` to mock the RPC event stream and database pool:
  - Verifies that indexer processes a mix of V1, upgrade (`upg_exec`), and V2 events without cursor loss.
  - Verifies that new/unknown event types in V2 are handled gracefully and logged instead of interrupting the polling loop.
  - Verifies that parser failures correctly route to the Dead-Letter Queue (DLQ) while allowing the cursor to progress.

### 5. CI Integration & Documentation
- Added `upgrade-integration` job to `.github/workflows/contracts.yml` to compile the WASMs and run the integration tests.
- Updated `UPGRADE.md` with instructions on upgrade integration testing.
- Updated the `CHANGELOG.md` with release notes.

## Verification
- Built and ran Rust integration tests on CI:
  ```bash
  cargo test --package indigopay-contract --test upgrade_test --features testutils
  ```
- Executed Jest backend indexer compatibility tests:
  ```bash
  npm test backend/__tests__/contractUpgrade.test.js
  ```
