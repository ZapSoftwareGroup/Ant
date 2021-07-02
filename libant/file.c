#include "file.h"

extern int libant_load_config_file(char* filepath, libant_buffer* buf)
{
	FILE* fp = fopen(filepath, "r");
	if(fp == NULL)
		return -1;
	libant_command cmd;
	char line[256];
	while(fgets(line, sizeof(line), fp))
	{
		libant_parse_line(line, &cmd);
		if(cmd.argcount == -1)
			continue;
		int index = libant_get_verb_index(cmd.cmdname);
		available_verbs[index].cmd(&cmd, buf);
	}
	fclose(fp);
	return 0;
}
