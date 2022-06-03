// #include "logging.c"
// #include <criterion/criterion.h>

// Test(logging, sanity) {
//   uint8_t instruction_data[] = {10, 11, 12, 13, 14};
//   GthPubkey program_id = {.x = {
//                               1,
//                           }};
//   GthPubkey key = {.x = {
//                        2,
//                    }};
//   uint64_t weis = 1;
//   uint8_t data[] = {0, 0, 0, 0};
//   GthAccountInfo accounts[] = {};
//   GthParameters params = {accounts, sizeof(accounts) /
//   sizeof(GthAccountInfo), instruction_data,
//                           sizeof(instruction_data), &program_id};

//   cr_assert(SUCCESS == logging(&params));
// }
