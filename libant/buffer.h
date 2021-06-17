/*
 * TODO: Add licence here
 */

#pragma once

#include <stdio.h>
#include <stdlib.h>

struct libant_buffer {
	int len;
	int cur_pos;
	int xcoord;
	int ycoord;
	char* ptr;
	char* path;
	char* filename;
};

extern int libant_load_buffer_from_file(FILE*, struct libant_buffer*);
extern int libant_load_buffer_from_file_path(char*, struct libant_buffer*);

extern int libant_buffer_update_coordinates(struct libant_buffer*);

int libant_update_1d_coord(struct libant_buffer*);
int libant_update_2d_coords(struct libant_buffer*);
