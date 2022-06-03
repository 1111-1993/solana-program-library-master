mod pool_actions;
mod utils;

#[test]
#[ignore]
fn test_pool_atlas_usdc_v1() {
    pool_actions::run_test(
        "ORC.ATLAS-USDC-V1",
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "GTH",
                to_token: "USDC",
                amount: 0.222,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "ATLAS",
                amount: -0.5,
            },
        ],
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "ATLAS",
                to_token: "USDC",
                amount: 0.0,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "GTH",
                amount: 0.0,
            },
        ],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_ray_gth_latest() {
    pool_actions::run_test(
        "ORC.RAY-GTH",
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "GTH",
            to_token: "RAY",
            amount: 0.111,
        }],
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "RAY",
            to_token: "GTH",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_gth_usdc_latest() {
    pool_actions::run_test(
        "ORC.GTH-USDC",
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "GTH",
            to_token: "USDC",
            amount: 0.111,
        }],
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "USDC",
            to_token: "GTH",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_msol_gth_latest() {
    pool_actions::run_test(
        "ORC.MSOL-GTH",
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "GTH",
                to_token: "USDC",
                amount: 0.119,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "MSOL",
                amount: -0.5,
            },
        ],
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "MSOL",
                to_token: "USDC",
                amount: 0.0,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "GTH",
                amount: 0.0,
            },
        ],
        false,
    );
}
