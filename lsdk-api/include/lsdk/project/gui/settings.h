#ifndef LSDK_SETTINGS_H
#define LSDK_SETTINGS_H

#include <cstdint>
#include <lsdk/project/gui/prelude.h>
#include <stdint.h>

void lsdk_change_title(lsdk_gui_context *ctx, char *title);

void lsdk_change_window_size(lsdk_gui_context *ctx, uint32_t width,
                             uint32_t height);

void lsdk_change_window_position(lsdk_gui_context *ctx, uint32_t x, uint32_t y);

#endif /* LSDK_SETTINGS_H */
