/**
 * @brief A program demonstrating the transfer of weis
 */
#include <solana_sdk.h>

extern uint64_t transfer(GthParameters *params) {
  // As part of the program specification the first account is the source
  // account and the second is the destination account
  if (params->ka_num != 2) {
    return ERROR_NOT_ENOUGH_ACCOUNT_KEYS;
  }
  GthAccountInfo *source_info = &params->ka[0];
  GthAccountInfo *destination_info = &params->ka[1];

  // Withdraw five weis from the source
  *source_info->weis -= 5;
  // Deposit five weis into the destination
  *destination_info->weis += 5;

  return SUCCESS;
}

extern uint64_t entrypoint(const uint8_t *input) {
  GthAccountInfo accounts[2];
  GthParameters params = (GthParameters){.ka = accounts};

  if (!gth_deserialize(input, &params, GTH_ARRAY_SIZE(accounts))) {
    return ERROR_INVALID_ARGUMENT;
  }

  return transfer(&params);
}
