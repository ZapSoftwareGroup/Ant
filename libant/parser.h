#pragma once

#include "command.h"
#include <stdlib.h>
#include <string.h>

typedef struct {
	char* verbname;
	command_function (*cmd)(libant_command*, libant_buffer* );
	int argcount;
} libant_verb;


static const libant_verb available_verbs[] = {
/*	VERB,	COMMAND,	ARGCOUNT */
	{ "set", libant_set_setting, 2 },
	{ "get", libant_get_setting, 1 },
};

extern int libant_parse_line(char* line, libant_command* outcmd);
extern int libant_get_argcount(char* verb);
int libant_get_verb_index(char* verb);
