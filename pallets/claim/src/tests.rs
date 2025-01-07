use crate::mock;
use crate::mock::RuntimeEvent as TestEvent;
use crate::mock::*;
use crate::*;
use frame_support::{assert_err, assert_ok};
use frame_system::{EventRecord, Phase};
use sp_runtime::traits::BadOrigin;

pub fn assert_evt(event: Event<Test>, context: &str) {
    assert_evt_gen(true, event, context);
}

pub fn assert_not_evt(event: Event<Test>, context: &str) {
    assert_evt_gen(false, event, context);
}

fn assert_evt_gen(contains: bool, event: Event<Test>, context: &str) {
    let message = match contains {
        true => format!("{context} - CANNOT FIND {:?}", event),
        false => format!("{context} - FOUND {:?}", event),
    };
    assert_eq!(
        contains,
        mock::System::events().contains(&EventRecord {
            phase: Phase::Initialization,
            event: TestEvent::Claim(event),
            topics: vec![],
        }),
        "{message}"
    )
}

#[test]
fn genesis_default_build() {
    test().execute_with(|| {
        assert!(Beneficiaries::<Test>::iter().next().is_none());
        assert_eq!(TotalClaimable::<Test>::get(), BalanceOf::<Test>::zero());
        assert!(!AirdropActive::<Test>::get());
        assert!(AirdropId::<Test>::get().is_none());
        assert_eq!(
            Balances::free_balance(ClaimAccount::get()),
            EXISTENTIAL_DEPOSIT
        );
        assert_eq!(Claim::pot(), 0);
    })
}

#[test]
#[should_panic(expected = "NotEnoughFunds")]
fn genesis_build_insufficient_balance() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Insufficient,
    )
    .execute_with(|| {});
}

#[test]
fn genesis_build_sufficient_balance() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_eq!(
            Beneficiaries::<Test>::iter().collect::<BTreeMap<_, _>>(),
            GENESIS_BENEFICIARIES_MAP.clone()
        );
        assert_eq!(TotalClaimable::<Test>::get(), SUFFICIENT_GENESIS_BALANCE);
        assert!(AirdropActive::<Test>::get());
        assert_eq!(AirdropId::<Test>::get(), Some(0));
        assert_eq!(
            Balances::free_balance(ClaimAccount::get()),
            SUFFICIENT_GENESIS_BALANCE + EXISTENTIAL_DEPOSIT
        );
        assert_eq!(Claim::pot(), SUFFICIENT_GENESIS_BALANCE);
    });
}

#[test]
fn new_airdrop() {
    test().execute_with(|| {
        assert_ok!(Claim::begin_airdrop(
            Origin::Signed(MANAGER_USER).into(),
            None
        ));
        assert_evt(Event::AirdropStarted { airdrop_id: 0 }, "New airdrop");
        assert!(Beneficiaries::<Test>::iter().next().is_none());
        assert_eq!(TotalClaimable::<Test>::get(), BalanceOf::<Test>::zero());
        assert!(AirdropActive::<Test>::get());
        assert_eq!(AirdropId::<Test>::get().unwrap(), 0);
        assert_eq!(
            Balances::free_balance(ClaimAccount::get()),
            EXISTENTIAL_DEPOSIT
        );
        assert_eq!(Claim::pot(), 0);
    })
}

#[test]
fn new_airdrop_wrong_origin() {
    test().execute_with(|| {
        assert_err!(
            Claim::begin_airdrop(Origin::Signed(USER_1).into(), None),
            BadOrigin
        );
        assert_not_evt(Event::AirdropStarted { airdrop_id: 0 }, "No new airdrop");
    })
}

#[test]
fn new_airdrop_sufficient_funds() {
    test_with_configs(
        WithGenesisBeneficiaries::No,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_ok!(Claim::begin_airdrop(
            Origin::Signed(MANAGER_USER).into(),
            Some(GENESIS_BENEFICIARIES_MAP.clone())
        ));
        assert_evt(Event::AirdropStarted { airdrop_id: 0 }, "New airdrop");
        assert_eq!(
            Beneficiaries::<Test>::iter().collect::<BTreeMap<_, _>>(),
            GENESIS_BENEFICIARIES_MAP.clone()
        );
        assert_eq!(TotalClaimable::<Test>::get(), SUFFICIENT_GENESIS_BALANCE);
        assert!(AirdropActive::<Test>::get());
        assert_eq!(AirdropId::<Test>::get(), Some(0));
    })
}

#[test]
fn new_aidrop_insufficient_funds() {
    test_with_configs(
        WithGenesisBeneficiaries::No,
        GenesisClaimBalance::Insufficient,
    )
    .execute_with(|| {
        assert_err!(
            Claim::begin_airdrop(
                Origin::Signed(MANAGER_USER).into(),
                Some(GENESIS_BENEFICIARIES_MAP.clone())
            ),
            Error::<Test>::NotEnoughFunds
        );
        assert_not_evt(Event::AirdropStarted { airdrop_id: 0 }, "New airdrop");

        // Adding beneficiaries must be atomic: if balance was insufficient none should've been added
        assert!(Beneficiaries::<Test>::iter().next().is_none());
        assert_eq!(TotalClaimable::<Test>::get(), BalanceOf::<Test>::zero());
        assert!(!AirdropActive::<Test>::get());
        assert!(AirdropId::<Test>::get().is_none());
        assert_eq!(
            Balances::free_balance(ClaimAccount::get()),
            INSUFFICIENT_GENESIS_BALANCE + EXISTENTIAL_DEPOSIT
        );
        assert_eq!(Claim::pot(), INSUFFICIENT_GENESIS_BALANCE);
    })
}

#[test]
fn cannot_start_new_aidrop_if_one_already_in_progress() {
    test().execute_with(|| {
        assert_ok!(Claim::begin_airdrop(
            Origin::Signed(MANAGER_USER).into(),
            None
        ));
        assert_evt(Event::AirdropStarted { airdrop_id: 0 }, "New airdrop");
        assert_err!(
            Claim::begin_airdrop(
                Origin::Signed(MANAGER_USER).into(),
                Some(GENESIS_BENEFICIARIES_MAP.clone())
            ),
            Error::<Test>::AlreadyStarted
        );
        assert_not_evt(Event::AirdropStarted { airdrop_id: 1 }, "No new airdrop");
    })
}

#[test]
fn claim() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_ok!(Claim::claim(Origin::Signed(USER_1).into(), None));
        assert_evt(
            Event::Claimed {
                beneficiary: USER_1,
                amount: USER_1_AMOUNT,
                payment_id: (),
            },
            "Successfull claim",
        );
        assert_eq!(Claim::pot(), SUFFICIENT_GENESIS_BALANCE - USER_1_AMOUNT);
        assert_eq!(
            TotalClaimable::<Test>::get(),
            SUFFICIENT_GENESIS_BALANCE - USER_1_AMOUNT
        );
        assert_eq!(Balances::free_balance(USER_1), USER_1_AMOUNT);
        assert!(Beneficiaries::<Test>::get(USER_1).is_none());
    });
}

#[test]
fn claim_with_opt_dest() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_ok!(Claim::claim(Origin::Signed(USER_1).into(), Some(USER_2)));
        assert_evt(
            Event::Claimed {
                beneficiary: USER_2,
                amount: USER_1_AMOUNT,
                payment_id: (),
            },
            "Successfull claim for a different dest",
        );
        assert_eq!(Claim::pot(), SUFFICIENT_GENESIS_BALANCE - USER_1_AMOUNT);
        assert_eq!(
            TotalClaimable::<Test>::get(),
            SUFFICIENT_GENESIS_BALANCE - USER_1_AMOUNT
        );
        assert_eq!(Balances::free_balance(USER_1), 0);
        assert_eq!(Balances::free_balance(USER_2), USER_1_AMOUNT);
        assert!(Beneficiaries::<Test>::get(USER_1).is_none());
    });
}

#[test]
fn claim_wrong_beneficiary() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_err!(
            Claim::claim(Origin::Signed(NON_BENEFICIARY).into(), None),
            Error::<Test>::NotEligible
        );
        assert_not_evt(
            Event::Claimed {
                beneficiary: NON_BENEFICIARY,
                amount: 0,
                payment_id: (),
            },
            "Cannot claim if not a beneficiary",
        );
        assert_eq!(Claim::pot(), SUFFICIENT_GENESIS_BALANCE);
        assert_eq!(TotalClaimable::<Test>::get(), SUFFICIENT_GENESIS_BALANCE);
        assert_eq!(
            Beneficiaries::<Test>::iter().collect::<BTreeMap<_, _>>(),
            GENESIS_BENEFICIARIES_MAP.clone()
        );
    });
}

#[test]
fn claim_insufficient_balance() {
    // Should never happen
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        Beneficiaries::<Test>::insert(NON_BENEFICIARY, SUFFICIENT_GENESIS_BALANCE + 1);
        assert_err!(
            Claim::claim(Origin::Signed(NON_BENEFICIARY).into(), None),
            Error::<Test>::PayoutError
        );
        assert_not_evt(
            Event::Claimed {
                beneficiary: NON_BENEFICIARY,
                amount: SUFFICIENT_GENESIS_BALANCE + 1,
                payment_id: (),
            },
            "Cannot claim if money not available",
        );
    })
}

#[test]
fn cannot_claim_while_airdrop_inactive() {
    test().execute_with(|| {
        assert_err!(
            Claim::claim(Origin::Signed(USER_1).into(), None),
            Error::<Test>::AlreadyEnded
        );
    })
}

#[test]
fn claim_for() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_ok!(Claim::claim_for(Origin::None.into(), USER_1));
        assert_evt(
            Event::Claimed {
                beneficiary: USER_1,
                amount: USER_1_AMOUNT,
                payment_id: (),
            },
            "Successfull claim for another beneficiary",
        );
        assert_eq!(Claim::pot(), SUFFICIENT_GENESIS_BALANCE - USER_1_AMOUNT);
        assert_eq!(
            TotalClaimable::<Test>::get(),
            SUFFICIENT_GENESIS_BALANCE - USER_1_AMOUNT
        );
        assert_eq!(Balances::free_balance(USER_1), USER_1_AMOUNT);
        assert!(Beneficiaries::<Test>::get(USER_1).is_none());
    });
}

#[test]
fn claim_for_insufficient_balance() {
    // Should never happen
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        Beneficiaries::<Test>::insert(NON_BENEFICIARY, SUFFICIENT_GENESIS_BALANCE + 1);
        assert_err!(
            Claim::claim_for(Origin::None.into(), NON_BENEFICIARY),
            Error::<Test>::PayoutError
        );
        assert_not_evt(
            Event::Claimed {
                beneficiary: NON_BENEFICIARY,
                amount: SUFFICIENT_GENESIS_BALANCE + 1,
                payment_id: (),
            },
            "Cannot claim for other if money not available",
        );
    })
}

#[test]
fn claim_for_wrong_beneficiary() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_err!(
            Claim::claim_for(Origin::None.into(), NON_BENEFICIARY),
            Error::<Test>::NotEligible
        );
        assert_not_evt(
            Event::Claimed {
                beneficiary: NON_BENEFICIARY,
                amount: 0,
                payment_id: (),
            },
            "Cannot claim if not a beneficiary",
        );
        assert_eq!(Claim::pot(), SUFFICIENT_GENESIS_BALANCE);
        assert_eq!(TotalClaimable::<Test>::get(), SUFFICIENT_GENESIS_BALANCE);
        assert_eq!(
            Beneficiaries::<Test>::iter().collect::<BTreeMap<_, _>>(),
            GENESIS_BENEFICIARIES_MAP.clone()
        );
    });
}

#[test]
fn cannot_claim_for_while_airdrop_inactive() {
    test().execute_with(|| {
        assert_err!(
            Claim::claim_for(Origin::None.into(), USER_1),
            Error::<Test>::AlreadyEnded
        );
    })
}

#[test]
fn add_beneficiaries_wrong_origin() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_err!(
            Claim::add_beneficiaries(Origin::Signed(USER_1).into(), NEW_BENEFICIARIES_MAP.clone()),
            BadOrigin
        );
    })
}

#[test]
fn cannot_add_beneficiaries_while_airdrop_inactive() {
    test().execute_with(|| {
        assert_err!(
            Claim::add_beneficiaries(
                Origin::Signed(MANAGER_USER).into(),
                NEW_BENEFICIARIES_MAP.clone()
            ),
            Error::<Test>::AlreadyEnded
        );
    })
}

#[test]
fn add_beneficiaries_sufficient_funds() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        let _ = Balances::mint_into(&Claim::account_id(), NEW_SUFFICIENT_BALANCE).unwrap();
        assert_ok!(Claim::add_beneficiaries(
            Origin::Signed(MANAGER_USER).into(),
            NEW_BENEFICIARIES_MAP.clone()
        ));
        assert_eq!(
            Claim::pot(),
            SUFFICIENT_GENESIS_BALANCE + NEW_SUFFICIENT_BALANCE
        );
        assert_eq!(
            TotalClaimable::<Test>::get(),
            SUFFICIENT_GENESIS_BALANCE + NEW_SUFFICIENT_BALANCE
        );
        assert_eq!(
            Beneficiaries::<Test>::iter().collect::<BTreeMap<_, _>>(),
            GENESIS_BENEFICIARIES_MAP
                .clone()
                .into_iter()
                .chain(NEW_BENEFICIARIES_MAP.clone().into_iter())
                .collect::<BTreeMap<_, _>>()
        );
    })
}

#[test]
fn add_beneficiaries_insufficient_funds() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        // Just add enough funds to cover for first insertions but not all
        let _ = Balances::mint_into(&Claim::account_id(), USER_4_AMOUNT + USER_5_AMOUNT).unwrap();

        assert_err!(
            Claim::add_beneficiaries(
                Origin::Signed(MANAGER_USER).into(),
                NEW_BENEFICIARIES_MAP.clone()
            ),
            Error::<Test>::NotEnoughFunds
        );

        // Atomic operation: state is untouched
        assert_eq!(
            Claim::pot(),
            SUFFICIENT_GENESIS_BALANCE + USER_4_AMOUNT + USER_5_AMOUNT
        );
        assert_eq!(TotalClaimable::<Test>::get(), SUFFICIENT_GENESIS_BALANCE);
        assert_eq!(
            Beneficiaries::<Test>::iter().collect::<BTreeMap<_, _>>(),
            GENESIS_BENEFICIARIES_MAP.clone()
        );
    })
}

#[test]
fn add_new_mixed_beneficiaries_modify_total_claimable_success() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        let _ = Balances::mint_into(&Claim::account_id(), MODIFIED_SUFFICIENT_BALANCE).unwrap();
        assert_ok!(Claim::add_beneficiaries(
            Origin::Signed(MANAGER_USER).into(),
            MODIFIED_BENEFICIARIES_MAP.clone()
        ));
        assert_eq!(
            Claim::pot(),
            SUFFICIENT_GENESIS_BALANCE + MODIFIED_SUFFICIENT_BALANCE
        );
        assert_eq!(
            TotalClaimable::<Test>::get(),
            SUFFICIENT_GENESIS_BALANCE + MODIFIED_SUFFICIENT_BALANCE
        );
        assert_eq!(
            Beneficiaries::<Test>::get(USER_1),
            Some(USER_1_MODIFIED_AMOUNT)
        );
        assert_eq!(
            Beneficiaries::<Test>::get(USER_3),
            Some(USER_3_MODIFIED_AMOUNT)
        );
        assert_eq!(Beneficiaries::<Test>::get(USER_6), Some(USER_6_AMOUNT));
    })
}

#[test]
fn add_new_mixed_beneficiaries_modify_total_claimable_failure() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        let _ = Balances::mint_into(&Claim::account_id(), USER_6_AMOUNT).unwrap();
        assert_err!(
            Claim::add_beneficiaries(
                Origin::Signed(MANAGER_USER).into(),
                MODIFIED_BENEFICIARIES_MAP.clone()
            ),
            Error::<Test>::NotEnoughFunds
        );
        // Atomic operation: state is untouched
        assert_eq!(Claim::pot(), SUFFICIENT_GENESIS_BALANCE + USER_6_AMOUNT);
        assert_eq!(TotalClaimable::<Test>::get(), SUFFICIENT_GENESIS_BALANCE);
        assert_eq!(
            Beneficiaries::<Test>::iter().collect::<BTreeMap<_, _>>(),
            GENESIS_BENEFICIARIES_MAP.clone()
        );
    })
}

#[test]
fn remove_beneficiaries_wrong_origin() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_err!(
            Claim::remove_beneficiaries(
                Origin::Signed(USER_1).into(),
                GENESIS_BENEFICIARIES_SET.clone()
            ),
            BadOrigin
        );
    })
}

#[test]
fn cannot_remove_beneficiaries_while_airdrop_inactive() {
    test().execute_with(|| {
        assert_err!(
            Claim::remove_beneficiaries(
                Origin::Signed(MANAGER_USER).into(),
                GENESIS_BENEFICIARIES_SET.clone()
            ),
            Error::<Test>::AlreadyEnded
        );
    })
}

#[test]
fn remove_beneficiaries_modify_total_claimable() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        let mut beneficiaries_to_remove = GENESIS_BENEFICIARIES_SET.clone();
        let _ = beneficiaries_to_remove.take(&USER_3);

        assert_ok!(Claim::remove_beneficiaries(
            Origin::Signed(MANAGER_USER).into(),
            beneficiaries_to_remove
        ));

        assert_eq!(Claim::pot(), SUFFICIENT_GENESIS_BALANCE);
        assert_eq!(
            TotalClaimable::<Test>::get(),
            SUFFICIENT_GENESIS_BALANCE - USER_1_AMOUNT - USER_2_AMOUNT
        );
        assert_eq!(
            Beneficiaries::<Test>::iter().collect::<Vec<(_, _)>>(),
            vec![(USER_3, USER_3_AMOUNT)]
        );
    })
}

#[test]
fn remove_beneficiaries_not_existing_is_ok() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        let mut beneficiaries_to_remove = GENESIS_BENEFICIARIES_SET.clone();
        beneficiaries_to_remove.insert(USER_4);
        assert_ok!(Claim::remove_beneficiaries(
            Origin::Signed(MANAGER_USER).into(),
            beneficiaries_to_remove
        ));
        assert!(Beneficiaries::<Test>::iter().next().is_none());
    })
}

#[test]
fn end_airdrop() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        // Give other balance. Now Self::pot() == SUFFICIENT_GENESIS_BALANCE * 2
        let _ = Balances::mint_into(&Claim::account_id(), SUFFICIENT_GENESIS_BALANCE).unwrap();
        assert_ok!(Claim::end_airdrop(Origin::Signed(MANAGER_USER).into()));
        assert!(!AirdropActive::<Test>::get());
        assert!(Beneficiaries::<Test>::iter().next().is_none());
        assert_eq!(Claim::pot(), 0);
        assert_eq!(
            Balances::free_balance(ClaimAccount::get()),
            EXISTENTIAL_DEPOSIT
        );
        assert_eq!(TotalClaimable::<Test>::get(), 0);
        assert_eq!(
            Balances::reducible_balance(
                &UnclaimedDestinationMockAccount::get(),
                Preservation::Expendable,
                Fortitude::Polite,
            ),
            SUFFICIENT_GENESIS_BALANCE * 2
        );
    });
}

#[test]
fn end_airdrop_wrong_origin() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_err!(Claim::end_airdrop(Origin::Signed(USER_1).into()), BadOrigin);
        assert_not_evt(Event::AirdropEnded { airdrop_id: 0 }, "No end airdrop");
    })
}

#[test]
fn double_end_airdrop() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_ok!(Claim::end_airdrop(Origin::Signed(MANAGER_USER).into()));
        assert_err!(
            Claim::end_airdrop(Origin::Signed(MANAGER_USER).into()),
            Error::<Test>::AlreadyEnded
        );
    });
}

#[test]
fn end_airdrop_new_airdrop() {
    test_with_configs(
        WithGenesisBeneficiaries::Yes,
        GenesisClaimBalance::Sufficient,
    )
    .execute_with(|| {
        assert_ok!(Claim::end_airdrop(Origin::Signed(MANAGER_USER).into()));
        assert_ok!(Claim::begin_airdrop(
            Origin::Signed(MANAGER_USER).into(),
            None
        ));
        assert_evt(Event::AirdropStarted { airdrop_id: 1 }, "New airdrop");
        assert!(Beneficiaries::<Test>::iter().next().is_none());
        assert_eq!(TotalClaimable::<Test>::get(), BalanceOf::<Test>::zero());
        assert!(AirdropActive::<Test>::get());
        assert_eq!(AirdropId::<Test>::get(), Some(1));
        assert_eq!(
            Balances::free_balance(ClaimAccount::get()),
            EXISTENTIAL_DEPOSIT
        );
        assert_eq!(Claim::pot(), 0);
    });
}
