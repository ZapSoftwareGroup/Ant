/*
 * TODO: Add licence here
 */

#pragma once

#include <stdio.h>
#include <stdlib.h>

struct libant_buffer {
	char* ptr;
	int len;
	char* path;
	char* filename;
};

extern int libant_load_buffer_from_file(FILE*, struct libant_buffer*);
extern int libant_load_buffer_from_file_path(char*, struct libant_buffer*);
