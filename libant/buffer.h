/*
 * TODO: Add licence here
 */

#pragma once

#include "config.h"
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int len;
	int cur_pos;
	int xcoord;
	int ycoord;
	char* ptr;
	char* path;
	char* filename;
	char* response;
	struct libant_setting* settings;
} libant_buffer;

extern int libant_init_buffer(libant_buffer*);

extern int libant_load_buffer_from_file(FILE*, libant_buffer*);
extern int libant_load_buffer_from_file_path(char*, libant_buffer*);

extern int libant_buffer_update_coordinates(libant_buffer*);

int libant_update_1d_coord(libant_buffer*);
int libant_update_2d_coords(libant_buffer*);
