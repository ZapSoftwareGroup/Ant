#include "parser.h"

extern int libant_parse_line(char* line, libant_command* outcmd)
{
	char* verb = NULL;
	char* word = strtok(line, " =\n");
	outcmd->argcount = -1;
	for(int i = 0; word != NULL; ++i)
	{
		if(word[0] == '#')
			break;
		if(i == 0)
		{
			verb = malloc(strlen(word)+1*sizeof(char));
			strcpy(verb, word);
			outcmd->cmdname = malloc(strlen(word)+1*sizeof(char));
			strcpy(outcmd->cmdname, verb);
			outcmd->argcount = libant_get_argcount(verb);
			outcmd->args = malloc(outcmd->argcount*sizeof(char*));
		}
		else
		{
			outcmd->args[i-1] = malloc(strlen(word)+1*sizeof(char));
			strcpy(outcmd->args[i-1], word);
		}
		word = strtok(NULL, " =\n");
	}
	return 0;
}

int libant_get_argcount(char* verb)
{
	int index = libant_get_verb_index(verb);
	return available_verbs[index].argcount;
	return -1;
}

int libant_get_verb_index(char* verb)
{
	for(int i = 0; i < sizeof(available_verbs)/sizeof(libant_verb); ++i)
	{
		if(strcmp(verb, available_verbs[i].verbname) == 0)
			return i;
	}
	return -1;
}
