#include "transfer-weis.c"
#include <criterion/criterion.h>

Test(transfer, sanity) {
  uint8_t instruction_data[] = {};
  GthPubkey program_id = {.x = {
                              1,
                          }};
  GthPubkey source_key = {.x = {
                              2,
                          }};
  uint64_t source_weis = 5;
  uint8_t source_data[] = {};
  GthPubkey destination_program_id = {.x = {
                                          3,
                                      }};
  GthPubkey destination_key = {.x = {
                                   4,
                               }};
  uint64_t destination_weis = 0;
  uint8_t destination_data[] = {};
  GthAccountInfo accounts[] = {{
                                   &source_key,
                                   &source_weis,
                                   sizeof(source_data),
                                   source_data,
                                   &program_id,
                                   0,
                                   true,
                                   true,
                                   false,
                               },
                               {
                                   &destination_key,
                                   &destination_weis,
                                   sizeof(destination_data),
                                   destination_data,
                                   &program_id,
                                   0,
                                   true,
                                   true,
                                   false,
                               }};
  GthParameters params = {accounts, sizeof(accounts) / sizeof(GthAccountInfo),
                          instruction_data, sizeof(instruction_data),
                          &program_id};

  cr_assert(SUCCESS == transfer(&params));
  cr_assert(0 == *accounts[0].weis);
  cr_assert(5 == *accounts[1].weis);
}
