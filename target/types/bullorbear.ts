/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/bullorbear.json`.
 */
export type Bullorbear = {
  "address": "DLEQCwxqxavJUK93bpdqXGkqJSmwJTmL2vnXRTPYNUau",
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
      "name": "betRefund",
      "discriminator": [
        50,
        253,
        176,
        135,
        68,
        62,
        177,
        197
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
      "args": []
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
        },
        {
          "name": "config",
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
          "name": "programData"
        },
        {
          "name": "program",
          "address": "DLEQCwxqxavJUK93bpdqXGkqJSmwJTmL2vnXRTPYNUau"
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
        },
        {
          "name": "admin",
          "type": "pubkey"
        },
        {
          "name": "operator",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "configUpdate",
      "discriminator": [
        80,
        37,
        109,
        136,
        82,
        135,
        89,
        241
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
        }
      ],
      "args": [
        {
          "name": "admin",
          "type": {
            "option": "pubkey"
          }
        },
        {
          "name": "operator",
          "type": {
            "option": "pubkey"
          }
        },
        {
          "name": "intervalSeconds",
          "type": {
            "option": "u16"
          }
        },
        {
          "name": "minBetAmount",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "treasuryFee",
          "type": {
            "option": "u16"
          }
        }
      ]
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
      "args": []
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
      "name": "roundAddFuture",
      "discriminator": [
        73,
        13,
        63,
        100,
        105,
        106,
        124,
        23
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
          "name": "lastAvailableRound",
          "writable": true
        },
        {
          "name": "futureRound",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "roundExecute",
      "discriminator": [
        45,
        21,
        191,
        6,
        192,
        5,
        11,
        134
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
          "name": "lastAvailableRound",
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
      "name": "roundLock",
      "discriminator": [
        220,
        60,
        236,
        217,
        40,
        15,
        28,
        10
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
          "name": "lastAvailableRound",
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
      "name": "initialized",
      "discriminator": [
        208,
        213,
        115,
        98,
        115,
        82,
        201,
        209
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
    },
    {
      "code": 6015,
      "name": "gamePaused",
      "msg": "Game is paused"
    },
    {
      "code": 6016,
      "name": "refundNotYetAvailable",
      "msg": "Refund not yet available"
    },
    {
      "code": 6017,
      "name": "priceTimestampMismatch",
      "msg": "Price timestamp not within 1 second window"
    },
    {
      "code": 6018,
      "name": "invalidAdminOrOperator",
      "msg": "Invalid admin or operator configuration"
    },
    {
      "code": 6019,
      "name": "invalidAccountOwner",
      "msg": "Invalid account owner"
    },
    {
      "code": 6020,
      "name": "invalidAccountSize",
      "msg": "Invalid account size"
    },
    {
      "code": 6021,
      "name": "insufficientAccountData",
      "msg": "Insufficient account data"
    },
    {
      "code": 6022,
      "name": "betSerializationFailed",
      "msg": "Bet serialization failed"
    },
    {
      "code": 6023,
      "name": "betVerificationFailed",
      "msg": "Bet verification failed"
    },
    {
      "code": 6024,
      "name": "invalidDiscriminator",
      "msg": "Invalid discriminator"
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
            "name": "payoutRatioBps",
            "type": "u64"
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
            "name": "lockedOnce",
            "type": "bool"
          },
          {
            "name": "paused",
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
            "name": "pendingBetAmount",
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
      "name": "initialized",
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
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "lockPriceExponent",
            "type": {
              "option": "i32"
            }
          },
          {
            "name": "closePrice",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "closePriceExponent",
            "type": {
              "option": "i32"
            }
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
