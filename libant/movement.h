/*
 * TODO: Add licence here
 */

#pragma once

#include "buffer.h"
#include <string.h>

extern char* libant_get_line(int, libant_buffer*);
extern int libant_move_up(int, libant_buffer*);
extern int libant_move_down(int, libant_buffer*);
extern int libant_move_left(int, libant_buffer*);
extern int libant_move_right(int, libant_buffer*);

int libant_get_distance_to_line_start(char*);
int libant_get_distance_to_line_end(char*);
