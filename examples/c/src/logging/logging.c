/**
 * @brief A program demonstrating logging
 */
#include <solana_sdk.h>

extern uint64_t logging(GthParameters *params) {
  // Log a string
  gth_log("static string");

  // Log 5 numbers as u64s in hexadecimal format
  gth_log_64(params->data[0], params->data[1], params->data[2], params->data[3],
             params->data[4]);

  // Log a slice
  gth_log_array(params->data, params->data_len);

  // Log a public key
  gth_log_pubkey(params->program_id);

  // Log all the program's input parameters
  gth_log_params(params);

  // Log the number of compute units remaining that the program can consume.
  gth_log_compute_units();

  return SUCCESS;
}

extern uint64_t entrypoint(const uint8_t *input) {
  GthAccountInfo accounts[1];
  GthParameters params = (GthParameters){.ka = accounts};

  if (!gth_deserialize(input, &params, GTH_ARRAY_SIZE(accounts))) {
    return ERROR_INVALID_ARGUMENT;
  }

  return logging(&params);
}
