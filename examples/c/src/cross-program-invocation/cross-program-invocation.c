/**
 * @brief A program demonstrating cross program invocations
 */
#include <solana_sdk.h>

/// Amount of bytes of account data to allocate
#define SIZE 42

extern uint64_t do_invoke(GthParameters *params) {
  // As part of the program specification the first account is the system
  // program's executable account and the second is the account to allocate
  if (params->ka_num != 2) {
    return ERROR_NOT_ENOUGH_ACCOUNT_KEYS;
  }
  GthAccountInfo *system_program_info = &params->ka[0];
  GthAccountInfo *allocated_info = &params->ka[1];

  uint8_t seed[] = {'Y', 'o', 'u', ' ', 'p', 'a', 's', 's',
                    ' ', 'b', 'u', 't', 't', 'e', 'r'};
  const GthSignerSeed seeds[] = {{seed, GTH_ARRAY_SIZE(seed)},
                                 {&params->data[0], 1}};
  const GthSignerSeeds signers_seeds[] = {{seeds, GTH_ARRAY_SIZE(seeds)}};

  GthPubkey expected_allocated_key;
  if (SUCCESS != gth_create_program_address(seeds, GTH_ARRAY_SIZE(seeds),
                                            params->program_id,
                                            &expected_allocated_key)) {
    return ERROR_INVALID_INSTRUCTION_DATA;
  }
  if (!GthPubkey_same(&expected_allocated_key, allocated_info->key)) {
    return ERROR_INVALID_ARGUMENT;
  }

  GthAccountMeta arguments[] = {{allocated_info->key, true, true}};
  uint8_t data[4 + 8];            // Enough room for the Allocate instruction
  *(uint16_t *)data = 8;          // Allocate instruction enum value
  *(uint64_t *)(data + 4) = SIZE; // Size to allocate
  const GthInstruction instruction = {system_program_info->key, arguments,
                                      GTH_ARRAY_SIZE(arguments), data,
                                      GTH_ARRAY_SIZE(data)};
  return gth_invoke_signed(&instruction, params->ka, params->ka_num,
                           signers_seeds, GTH_ARRAY_SIZE(signers_seeds));
}

extern uint64_t entrypoint(const uint8_t *input) {
  GthAccountInfo accounts[2];
  GthParameters params = (GthParameters){.ka = accounts};

  if (!gth_deserialize(input, &params, GTH_ARRAY_SIZE(accounts))) {
    return ERROR_INVALID_ARGUMENT;
  }

  return do_invoke(&params);
}
