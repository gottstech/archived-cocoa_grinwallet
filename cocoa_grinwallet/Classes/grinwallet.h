// Copyright 2019 Ivan Sorokin.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void cstr_free(const char *s);

const char*  grin_check_password(
    const char* json_cfg,
    const char* password,
    uint8_t *error
);

const char* grin_init_wallet_seed(uint8_t *error);

const char* grin_wallet_init(
     const char* json_cfg,
     const char* password,
     bool is_12_phrases,
     uint8_t *error
);

const char* grin_wallet_init_recover(
    const char* json_cfg,
    const char* mnemonic,
    uint8_t *error
);

const char* grin_wallet_restore(
    const char* json_cfg,
    uint64_t start_index,
    uint64_t batch_size,
    uint8_t *error
);

const char* grin_wallet_check(
    const char* json_cfg,
    uint64_t start_index,
    uint64_t batch_size,
    bool update_outputs,
    uint8_t *error
);

const char* grin_get_wallet_mnemonic(
    const char* json_cfg,
    uint8_t *error
);

const char* grin_get_balance(
    const char* json_cfg,
    uint8_t *error
);

const char* grin_tx_retrieve(
    const char* json_cfg,
    const char* tx_slate_id,
    uint8_t *error
);

const char* grin_txs_retrieve(
    const char* json_cfg,
    uint8_t *error
);

const char* grin_output_retrieve(
    const char* json_cfg,
    uint32_t id,
    uint8_t *error
);

const char* grin_outputs_retrieve(
    const char* json_cfg,
    uint8_t *error
);

const char* grin_listen(
    const char* json_cfg,
    uint8_t *error
);

const char* grin_init_tx(
    const char* json_cfg,
    uint64_t amount,
    const char* selection_strategy,
    int16_t target_slate_version,
    const char* message,
    uint8_t *error
);

const char* grin_send_tx(
    const char* json_cfg,
    uint64_t amount,
    const char* receiver_wallet_url,
    const char* selection_strategy,
    int16_t target_slate_version,
    const char* message,
    uint8_t *error
);

const char* grin_cancel_tx(
    const char* json_cfg,
    const char* tx_slate_id,
    uint8_t *error
);

const char* grin_post_tx(
    const char* json_cfg,
    const char* tx_slate_id,
    uint8_t *error
);

const char* grin_tx_file_receive(
    const char* json_cfg,
    const char* slate_file_path,
    const char* message,
    uint8_t *error
);

const char*  grin_tx_file_finalize(
    const char* json_cfg,
    const char* slate_file_path,
    uint8_t *error
);

const char* grin_chain_height(
    const char* json_cfg,
    uint8_t *error
);


