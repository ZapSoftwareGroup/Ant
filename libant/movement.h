/*
 * TODO: Add licence here
 */

#include "buffer.h"

extern int libant_movement(int, char, struct libant_buffer*);
int libant_movement_action(char, struct libant_buffer*);
int libant_get_distance_to_line_start(struct libant_buffer*);
int libant_get_distance_to_line_end(struct libant_buffer*);
