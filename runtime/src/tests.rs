// This are integration tests for pallets (eg pallet-settlement-fflonk)
// This are unit tests for runtime

use super::*;

use frame_support::traits::{fungible::Inspect, Currency, ExistenceRequirement, WithdrawReasons};

mod testsfixtures;

// Function used for creating the environment for the test.
// It must return a sp_io::TestExternalities, and the actual test will execute this one before running.
fn new_test_ext() -> sp_io::TestExternalities {
    // This builds the initial genesis storage for this test
    let mut t = frame_system::GenesisConfig::<super::Runtime>::default()
        .build_storage()
        .unwrap();

    // Create four users by calling the fixture function
    let sample_users = testsfixtures::get_sample_users();

    // Incorporate the pairs account / starting balance into the Genesis config
    pallet_balances::GenesisConfig::<super::Runtime> {
        balances: sample_users
            .into_iter()
            .map(|user| (user.raw_account.into(), user.starting_balance))
            .collect(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::from(t);

    ext.execute_with(|| System::set_block_number(1));

    // Return the test externalities
    ext
}

// Test definition and execution. Test body must be written in the execute_with closure.
#[test]
fn check_starting_balances_and_existential_limit() {
    new_test_ext().execute_with(|| {
        // This creates a few public keys used to be converted to AccountId
        let sample_users = testsfixtures::get_sample_users();

        for sample_user in &sample_users {
            assert_eq!(
                Balances::balance(&sample_user.raw_account.into()),
                sample_user.starting_balance
            );
        }

        // Now perform a withdraw on the fourth account, leaving its balance under the EXISTENTIAL_DEPOSIT limit
        // This should kill the account, when executed with the ExistenceRequirement::AllowDeath option
        let _id_3_withdraw = Balances::withdraw(
            &sample_users[3].raw_account.into(),
            testsfixtures::EXISTENTIAL_DEPOSIT_REMINDER, // Withdrawing more th
            WithdrawReasons::TIP,
            ExistenceRequirement::AllowDeath,
        );

        // Verify that the fourth account balance is now 0
        assert_eq!(Balances::balance(&sample_users[3].raw_account.into()), 0);
    });
}

// Test definition and execution. Test body must be written in the execute_with closure.
#[test]
fn pallet_fflonk_availability() {
    new_test_ext().execute_with(|| {
        let dummy_origin = sp_runtime::AccountId32::new([0; 32]);
        let dummy_raw_proof = [0; 25 * 32];
        assert!(SettlementFFlonkPallet::submit_proof(
            RuntimeOrigin::signed(dummy_origin),
            dummy_raw_proof
        ).is_err());
        // just checking code builds, hence the pallet is available to the runtime
    });
}