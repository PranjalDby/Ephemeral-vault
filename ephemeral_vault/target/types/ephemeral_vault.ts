/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/ephemeral_vault.json`.
 */
export type EphemeralVault = {
  "address": "2Y2AseLPmKvaGRXsU4yB3hjjMgXyhh9Y4LVgsgkSzCoT",
  "metadata": {
    "name": "ephemeralVault",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "createEphemeralVault",
      "discriminator": [
        97,
        118,
        149,
        230,
        158,
        6,
        238,
        26
      ],
      "accounts": [
        {
          "name": "parentWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "ephemeralWallet"
        },
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "const",
                "value": [
                  118,
                  50
                ]
              },
              {
                "kind": "account",
                "path": "parentWallet"
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
          "name": "sessionDuration",
          "type": "i64"
        }
      ]
    },
    {
      "name": "depositSol",
      "discriminator": [
        108,
        81,
        78,
        117,
        125,
        155,
        56,
        200
      ],
      "accounts": [
        {
          "name": "parentWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "const",
                "value": [
                  118,
                  50
                ]
              },
              {
                "kind": "account",
                "path": "parentWallet"
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
    },
    {
      "name": "placeTrade",
      "docs": [
        "Open or update a perpetual futures position using the ephemeral wallet.",
        "No SOL transfers needed here — trade is purely risk accounting."
      ],
      "discriminator": [
        102,
        39,
        166,
        38,
        98,
        171,
        190,
        242
      ],
      "accounts": [
        {
          "name": "parentWallet",
          "writable": true
        },
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "const",
                "value": [
                  118,
                  50
                ]
              },
              {
                "kind": "account",
                "path": "parentWallet"
              }
            ]
          }
        },
        {
          "name": "ephemeralWallet",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "size",
          "type": "i64"
        },
        {
          "name": "price",
          "type": "i64"
        }
      ]
    },
    {
      "name": "revokeSession",
      "docs": [
        "Parent wallet can revoke the trading session before expiry.",
        "This makes the ephemeral wallet lose authority immediately."
      ],
      "discriminator": [
        86,
        92,
        198,
        120,
        144,
        2,
        7,
        194
      ],
      "accounts": [
        {
          "name": "parentWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "const",
                "value": [
                  118,
                  50
                ]
              },
              {
                "kind": "account",
                "path": "parentWallet"
              }
            ]
          }
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "vaultAccount",
      "discriminator": [
        230,
        251,
        241,
        83,
        139,
        202,
        93,
        28
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "sessionExpired",
      "msg": "Session expired"
    }
  ],
  "types": [
    {
      "name": "vaultAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "parentWallet",
            "type": "pubkey"
          },
          {
            "name": "ephemeralWallet",
            "type": "pubkey"
          },
          {
            "name": "sessionExpiresAt",
            "type": "i64"
          },
          {
            "name": "positionSize",
            "type": "i64"
          },
          {
            "name": "entryPrice",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
