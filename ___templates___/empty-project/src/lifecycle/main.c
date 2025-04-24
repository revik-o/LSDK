
void main_loop(lsdk_activity_context *ctx) {
  while (lsdk_is_running(ctx->main_ctx)) {
    ctx->activity(ctx->main_ctx, (void *)ctx);
  }
}

int main(int argc, char *argv[]) {
  const lsdk_gui_context context = lsdk_show_window();
  lsdk_on_init(*context);
  main_loop(*context);
  int code = lsdk_on_exit(*context);
  lsdk_destroy_window(*context);
  return code;
}
