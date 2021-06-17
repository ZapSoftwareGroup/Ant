/*
 * TODO: Add licence here
 */

#pragma once

#include "buffer.h"
#include <string.h>

extern char* libant_get_line(int, struct libant_buffer*);
extern int libant_move_up(int, struct libant_buffer*);
extern int libant_move_down(int, struct libant_buffer*);

int libant_get_distance_to_line_start(char*);
int libant_get_distance_to_line_end(char*);
