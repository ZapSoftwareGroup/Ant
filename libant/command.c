#include "command.h"

command_function libant_set_setting(libant_command* cmd, libant_buffer* buf)
{
	int index = libant_get_setting_index(cmd->args[0]);
	buf->settings[index].value = malloc((strlen(cmd->args[1])+1)*sizeof(char));
	strcpy(buf->settings[index].value, cmd->args[1]);
	return 0;
}

command_function libant_get_setting(libant_command* cmd, libant_buffer* buf)
{
	int index = libant_get_setting_index(cmd->args[0]);
	buf->response = malloc((strlen(buf->settings[index].value)+1)*sizeof(char));
	strcpy(buf->response, buf->settings[index].value);
	return 0;
}


int libant_get_setting_index(char* setting)
{
	for(int i = 0; i < sizeof(default_settings)/sizeof(struct libant_setting); ++i)
	{
		if(strcmp(default_settings[i].name, setting) == 0)
			return i;
	}
	return -1;
}
