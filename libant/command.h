#pragma once

#include "buffer.h"
#include <string.h>

typedef struct {
	char* cmdname;
	char** args;
	int argcount;
} libant_command;

typedef int *command_function;

command_function libant_set_setting(libant_command*, libant_buffer* );
command_function libant_get_setting(libant_command*, libant_buffer*);

int libant_get_setting_index(char*);
