/*
 * Copyright 2024 The Dora Authors.
 * Licensed under the Apache License, Version 2.0.
*/ 

#ifndef _DORA_H
#define _DORA_H

#include <evmc/evmc.h>
#include <evmc/utils.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Creates EVMC Dora VM.
 */
EVMC_EXPORT struct evmc_vm* evmc_create_doravm(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _DORA_H */
