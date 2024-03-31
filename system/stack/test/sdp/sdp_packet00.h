/*
 * Copyright 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <cstddef>

#include "stack/test/sdp/raw_packet.h"

namespace bluetooth {
namespace testing {
namespace stack {
namespace sdp {
namespace packets00 {

// b/321872406#comment32
// Frame (103 bytes)
static const unsigned char pkt1017[103] = {
    0x02, 0x0b, 0x20, 0x62, 0x00, 0x5e, 0x00, 0x46,  // .. b.^.F
    0x00, 0x07, 0x00, 0x00, 0x00, 0x59, 0x00, 0x56,  // .....Y.V
    0x36, 0x00, 0x53, 0x36, 0x00, 0x50, 0x09, 0x00,  // 6.S6.P..
    0x00, 0x0a, 0x00, 0x01, 0x00, 0x09, 0x09, 0x00,  // ........
    0x01, 0x35, 0x11, 0x1c, 0x4d, 0xe1, 0x7a, 0x00,  // .5..M.z.
    0x52, 0xcb, 0x11, 0xe6, 0xbd, 0xf4, 0x08, 0x00,  // R.......
    0x20, 0x0c, 0x9a, 0x66, 0x09, 0x00, 0x02, 0x0a,  //  ..f....
    0x00, 0x8f, 0x51, 0x62, 0x09, 0x00, 0x04, 0x35,  // ..Qb...5
    0x0c, 0x35, 0x03, 0x19, 0x01, 0x00, 0x35, 0x05,  // .5....5.
    0x19, 0x00, 0x03, 0x08, 0x03, 0x09, 0x00, 0x05,  // ........
    0x35, 0x03, 0x19, 0x10, 0x02, 0x09, 0x00, 0x09,  // 5.......
    0x35, 0x08, 0x35, 0x06, 0x19, 0x11, 0x01, 0x09,  // 5.5.....
    0x01, 0x02, 0x09, 0x01, 0x00, 0x00, 0x00         // .......
};

const raw_packet_t rx_pkts[] = {
    {pkt1017, sizeof(pkt1017)},
};
const size_t kNumRxPkts = sizeof(rx_pkts) / sizeof(rx_pkts[0]);

}  // namespace packets00
}  // namespace sdp
}  // namespace stack
}  // namespace testing
}  // namespace bluetooth
