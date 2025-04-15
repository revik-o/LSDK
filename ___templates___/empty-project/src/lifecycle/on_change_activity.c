// some code
#include <stdint.h>
typedef void (*routineptr)(void *);
//
routineptr lsdk_on_change_activity(uint32_t activity_sig) {
    return draw_activity_main; // TODO link
}
