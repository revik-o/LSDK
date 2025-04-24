#ifndef LSDK_PRELUDE_H
#define LSDK_PRELUDE_H

#include <cstdint>

#define LSDK_API_VERSION 1

typedef struct {
  char *title;
  uint32_t x;
  uint32_t y;
  uint32_t width;
  uint32_t height;
  void *native_context_ptr;
} lsdk_gui_context;

typedef void (*activity_func_ptr)(lsdk_gui_context *, void *);

typedef struct {
    lsdk_gui_context *main_ctx;
    activity_func_ptr activity;
} lsdk_activity_context;


lsdk_gui_context lsdk_show_window();

bool lsdk_is_running(lsdk_gui_context *ctx);

void lsdk_destroy_window(lsdk_gui_context *ctx);

void lsdk_set_activity(lsdk_gui_context *ctx, activity_func_ptr activity);

#endif /* LSDK_PRELUDE_H */
