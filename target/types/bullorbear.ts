/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/bullorbear.json`.
 */
export type Bullorbear = {
  "address": "F4Cu5nYYQYJU9qdqyDcZsMbadcNeADDZTqD9AnN12DFK",
  "metadata": {
    "name": "bullorbear",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "bet",
      "discriminator": [
        94,
        203,
        166,
        126,
        20,
        243,
        169,
        82
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "bet",
          "writable": true
        },
        {
          "name": "round",
          "writable": true
        },
        {
          "name": "treasury",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  114,
                  101,
                  97,
                  115,
                  117,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "epoch",
          "type": "u64"
        },
        {
          "name": "position",
          "type": {
            "defined": {
              "name": "position"
            }
          }
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "closeBet",
      "discriminator": [
        185,
        206,
        13,
        184,
        176,
        108,
        140,
        107
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "bet",
          "writable": true
        },
        {
          "name": "user",
          "writable": true
        },
        {
          "name": "round",
          "writable": true
        },
        {
          "name": "treasury",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  114,
                  101,
                  97,
                  115,
                  117,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "closeConfig",
      "discriminator": [
        145,
        9,
        72,
        157,
        95,
        125,
        61,
        85
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "destination",
          "writable": true,
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "closeRound",
      "discriminator": [
        149,
        14,
        81,
        88,
        230,
        226,
        234,
        37
      ],
      "accounts": [
        {
          "name": "destination",
          "writable": true,
          "signer": true
        },
        {
          "name": "round",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "epoch",
          "type": "u64"
        }
      ]
    },
    {
      "name": "configInitialize",
      "discriminator": [
        129,
        48,
        207,
        45,
        143,
        130,
        95,
        127
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "interval",
          "type": "u16"
        },
        {
          "name": "minBetAmount",
          "type": "u64"
        },
        {
          "name": "treasuryFee",
          "type": "u16"
        }
      ]
    },
    {
      "name": "genesisExecute",
      "discriminator": [
        185,
        155,
        201,
        180,
        98,
        39,
        158,
        118
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "round",
          "writable": true
        },
        {
          "name": "nextRound",
          "writable": true
        },
        {
          "name": "futureRound",
          "writable": true
        },
        {
          "name": "priceUpdate"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "genesisLock",
      "discriminator": [
        58,
        128,
        180,
        184,
        194,
        218,
        56,
        100
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "round",
          "writable": true
        },
        {
          "name": "nextRound",
          "writable": true
        },
        {
          "name": "futureRound",
          "writable": true
        },
        {
          "name": "priceUpdate"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "round",
          "writable": true
        },
        {
          "name": "nextRound",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "interval",
          "type": "u16"
        },
        {
          "name": "epoch",
          "type": "u64"
        }
      ]
    },
    {
      "name": "pause",
      "discriminator": [
        211,
        22,
        221,
        251,
        74,
        121,
        193,
        47
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "unpause",
      "discriminator": [
        169,
        144,
        4,
        38,
        10,
        141,
        188,
        255
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "withdrawTreasury",
      "discriminator": [
        40,
        63,
        122,
        158,
        144,
        216,
        83,
        96
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "treasury",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  114,
                  101,
                  97,
                  115,
                  117,
                  114,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "bet",
      "discriminator": [
        147,
        23,
        35,
        59,
        15,
        75,
        155,
        32
      ]
    },
    {
      "name": "config",
      "discriminator": [
        155,
        12,
        170,
        224,
        30,
        250,
        204,
        130
      ]
    },
    {
      "name": "priceUpdateV2",
      "discriminator": [
        34,
        241,
        35,
        99,
        157,
        126,
        244,
        205
      ]
    },
    {
      "name": "round",
      "discriminator": [
        87,
        127,
        165,
        51,
        73,
        78,
        116,
        174
      ]
    }
  ],
  "events": [
    {
      "name": "betEvent",
      "discriminator": [
        123,
        52,
        73,
        185,
        152,
        169,
        40,
        128
      ]
    },
    {
      "name": "claim",
      "discriminator": [
        133,
        98,
        9,
        238,
        133,
        207,
        191,
        113
      ]
    },
    {
      "name": "genesisInitialized",
      "discriminator": [
        41,
        79,
        39,
        224,
        64,
        195,
        0,
        233
      ]
    },
    {
      "name": "pause",
      "discriminator": [
        194,
        251,
        232,
        196,
        118,
        95,
        111,
        219
      ]
    },
    {
      "name": "rewardsCalculated",
      "discriminator": [
        189,
        53,
        68,
        150,
        159,
        153,
        89,
        115
      ]
    },
    {
      "name": "roundClosed",
      "discriminator": [
        45,
        243,
        28,
        22,
        132,
        70,
        175,
        226
      ]
    },
    {
      "name": "roundInitialized",
      "discriminator": [
        238,
        116,
        151,
        217,
        19,
        157,
        254,
        83
      ]
    },
    {
      "name": "roundStarted",
      "discriminator": [
        180,
        209,
        2,
        244,
        238,
        48,
        170,
        120
      ]
    },
    {
      "name": "unpause",
      "discriminator": [
        241,
        149,
        104,
        90,
        199,
        136,
        219,
        146
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "genesisLockRoundOnce",
      "msg": "Can run genesis_lock_round only once"
    },
    {
      "code": 6001,
      "name": "invalidEpoch",
      "msg": "Invalid round epoch"
    },
    {
      "code": 6002,
      "name": "roundNotBettable",
      "msg": "Round not bettable"
    },
    {
      "code": 6003,
      "name": "betTooSmall",
      "msg": "Bet amount below minimum"
    },
    {
      "code": 6004,
      "name": "alreadyClaimed",
      "msg": "Already claimed"
    },
    {
      "code": 6005,
      "name": "roundNotEnded",
      "msg": "Round not ended"
    },
    {
      "code": 6006,
      "name": "unauthorizedOperator",
      "msg": "Unauthorized operator"
    },
    {
      "code": 6007,
      "name": "overflow",
      "msg": "Arithmetic overflow occurred"
    },
    {
      "code": 6008,
      "name": "invalidFee",
      "msg": "Invalid fee percentage"
    },
    {
      "code": 6009,
      "name": "invalidPriceFeed",
      "msg": "The price feed account is invalid or not BTC/USD."
    },
    {
      "code": 6010,
      "name": "stalePrice",
      "msg": "The price feed is stale or not currently trading."
    },
    {
      "code": 6011,
      "name": "mathOverflow",
      "msg": "Math overflow"
    },
    {
      "code": 6012,
      "name": "bettingClosed",
      "msg": "Betting is closed for this round"
    },
    {
      "code": 6013,
      "name": "unresolvedBetsExist",
      "msg": "Unresolved bets exist for this round"
    },
    {
      "code": 6014,
      "name": "insufficientTreasuryFunds",
      "msg": "Insufficient treasury funds"
    }
  ],
  "types": [
    {
      "name": "bet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "epoch",
            "type": "u64"
          },
          {
            "name": "position",
            "type": {
              "defined": {
                "name": "position"
              }
            }
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "claimed",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "betEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epoch",
            "type": "u64"
          },
          {
            "name": "position",
            "type": {
              "defined": {
                "name": "position"
              }
            }
          },
          {
            "name": "betAmount",
            "type": "u64"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "bullAmount",
            "type": "u64"
          },
          {
            "name": "bearAmount",
            "type": "u64"
          },
          {
            "name": "bullTotalBets",
            "type": "u32"
          },
          {
            "name": "bearTotalBets",
            "type": "u32"
          },
          {
            "name": "user",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "claim",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "epoch",
            "type": "u64"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "winningPosition",
            "type": {
              "defined": {
                "name": "position"
              }
            }
          },
          {
            "name": "payoutRatio",
            "type": "f64"
          }
        ]
      }
    },
    {
      "name": "config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "operator",
            "type": "pubkey"
          },
          {
            "name": "genesisLockOnce",
            "type": "bool"
          },
          {
            "name": "genesisInitialized",
            "type": "bool"
          },
          {
            "name": "intervalSeconds",
            "type": "u16"
          },
          {
            "name": "minBetAmount",
            "type": "u64"
          },
          {
            "name": "treasuryFee",
            "type": "u16"
          },
          {
            "name": "treasuryAmount",
            "type": "u64"
          },
          {
            "name": "currentEpoch",
            "type": "u64"
          },
          {
            "name": "lastAvailableEpoch",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "genesisInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "interval",
            "type": "u16"
          },
          {
            "name": "epoch",
            "type": "u64"
          },
          {
            "name": "lockTs",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "pause",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epoch",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "position",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "bull"
          },
          {
            "name": "bear"
          }
        ]
      }
    },
    {
      "name": "priceFeedMessage",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feedId",
            "docs": [
              "`FeedId` but avoid the type alias because of compatibility issues with Anchor's `idl-build` feature."
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "price",
            "type": "i64"
          },
          {
            "name": "conf",
            "type": "u64"
          },
          {
            "name": "exponent",
            "type": "i32"
          },
          {
            "name": "publishTime",
            "docs": [
              "The timestamp of this price update in seconds"
            ],
            "type": "i64"
          },
          {
            "name": "prevPublishTime",
            "docs": [
              "The timestamp of the previous price update. This field is intended to allow users to",
              "identify the single unique price update for any moment in time:",
              "for any time t, the unique update is the one such that prev_publish_time < t <= publish_time.",
              "",
              "Note that there may not be such an update while we are migrating to the new message-sending logic,",
              "as some price updates on pythnet may not be sent to other chains (because the message-sending",
              "logic may not have triggered). We can solve this problem by making the message-sending mandatory",
              "(which we can do once publishers have migrated over).",
              "",
              "Additionally, this field may be equal to publish_time if the message is sent on a slot where",
              "where the aggregation was unsuccesful. This problem will go away once all publishers have",
              "migrated over to a recent version of pyth-agent."
            ],
            "type": "i64"
          },
          {
            "name": "emaPrice",
            "type": "i64"
          },
          {
            "name": "emaConf",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "priceUpdateV2",
      "docs": [
        "A price update account. This account is used by the Pyth Receiver program to store a verified price update from a Pyth price feed.",
        "It contains:",
        "- `write_authority`: The write authority for this account. This authority can close this account to reclaim rent or update the account to contain a different price update.",
        "- `verification_level`: The [`VerificationLevel`] of this price update. This represents how many Wormhole guardian signatures have been verified for this price update.",
        "- `price_message`: The actual price update.",
        "- `posted_slot`: The slot at which this price update was posted."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "writeAuthority",
            "type": "pubkey"
          },
          {
            "name": "verificationLevel",
            "type": {
              "defined": {
                "name": "verificationLevel"
              }
            }
          },
          {
            "name": "priceMessage",
            "type": {
              "defined": {
                "name": "priceFeedMessage"
              }
            }
          },
          {
            "name": "postedSlot",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "rewardsCalculated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epoch",
            "type": "u64"
          },
          {
            "name": "rewardBase",
            "type": "u64"
          },
          {
            "name": "rewardAmount",
            "type": "u64"
          },
          {
            "name": "treasuryAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "round",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epoch",
            "type": "u64"
          },
          {
            "name": "startTs",
            "type": "i64"
          },
          {
            "name": "lockTs",
            "type": "i64"
          },
          {
            "name": "closeTs",
            "type": "i64"
          },
          {
            "name": "lockPrice",
            "type": "i64"
          },
          {
            "name": "lockPriceExponent",
            "type": "i32"
          },
          {
            "name": "closePrice",
            "type": "i64"
          },
          {
            "name": "closePriceExponent",
            "type": "i32"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "bullAmount",
            "type": "u64"
          },
          {
            "name": "bearAmount",
            "type": "u64"
          },
          {
            "name": "bullTotalBets",
            "type": "u32"
          },
          {
            "name": "bearTotalBets",
            "type": "u32"
          },
          {
            "name": "rewardBase",
            "type": "u64"
          },
          {
            "name": "rewardAmount",
            "type": "u64"
          },
          {
            "name": "unresolvedBetsCount",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "roundClosed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": "pubkey"
          },
          {
            "name": "epoch",
            "type": "u64"
          },
          {
            "name": "closeTs",
            "type": "i64"
          },
          {
            "name": "closePrice",
            "type": "i64"
          },
          {
            "name": "closePriceExponent",
            "type": "i32"
          }
        ]
      }
    },
    {
      "name": "roundInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epoch",
            "type": "u64"
          },
          {
            "name": "startTs",
            "type": "i64"
          },
          {
            "name": "lockTs",
            "type": "i64"
          },
          {
            "name": "closeTs",
            "type": "i64"
          },
          {
            "name": "key",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "roundStarted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": "pubkey"
          },
          {
            "name": "epoch",
            "type": "u64"
          },
          {
            "name": "closeTs",
            "type": "i64"
          },
          {
            "name": "lockPrice",
            "type": "i64"
          },
          {
            "name": "lockPriceExponent",
            "type": "i32"
          }
        ]
      }
    },
    {
      "name": "unpause",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epoch",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "verificationLevel",
      "docs": [
        "Pyth price updates are bridged to all blockchains via Wormhole.",
        "Using the price updates on another chain requires verifying the signatures of the Wormhole guardians.",
        "The usual process is to check the signatures for two thirds of the total number of guardians, but this can be cumbersome on Solana because of the transaction size limits,",
        "so we also allow for partial verification.",
        "",
        "This enum represents how much a price update has been verified:",
        "- If `Full`, we have verified the signatures for two thirds of the current guardians.",
        "- If `Partial`, only `num_signatures` guardian signatures have been checked.",
        "",
        "# Warning",
        "Using partially verified price updates is dangerous, as it lowers the threshold of guardians that need to collude to produce a malicious price update."
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "partial",
            "fields": [
              {
                "name": "numSignatures",
                "type": "u8"
              }
            ]
          },
          {
            "name": "full"
          }
        ]
      }
    }
  ]
};
