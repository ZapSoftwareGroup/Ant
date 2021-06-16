/*
 * TODO: Add licence here
 */

#pragma once

#include "buffer.h"
#include <string.h>

extern int libant_movement(int, char, struct libant_buffer*);
extern char* libant_get_line(int, struct libant_buffer*);
int libant_movement_action(char, struct libant_buffer*);
int libant_get_distance_to_line_start(char*);
int libant_get_distance_to_line_end(char*);
