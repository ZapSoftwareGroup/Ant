#include "../libant.h"

int main()
{
	libant_command cmd;
	char line[50] = "set kameli=best";
	libant_parse_line(line, &cmd);
	printf("Verb: %s\n", cmd.cmdname);
	printf("Args: \n");
	for(int i = 0; i < cmd.argcount; ++i)
		printf("\t%s\n", cmd.args[i]);
	printf("\n");
	return 0;
}
