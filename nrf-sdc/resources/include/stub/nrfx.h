// Minimal stand-in for nrfx.h, so that the nrfxlib headers parse without a
// copy of the nRF MDK. Only the declarations the nrfxlib headers reference are
// provided; none of them affect the layout of the types bindgen emits.
#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define NRF52_SERIES  1
#define NRF52840_XXAA 1

#define __PACKED         __attribute__((packed))
#define __ALIGN(n)       __attribute__((aligned(n)))
#define __STATIC_INLINE  static inline

typedef int IRQn_Type;
